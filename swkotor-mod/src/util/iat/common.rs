/// Utilities to help with IAT hooking
///
use plthook::ObjectFile;
use std::{error::Error, ffi::c_void, sync::Arc};
use windows::Win32::System::Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_READWRITE};

#[derive(Clone)]
pub struct IatStoreInternal<T: Clone> {
    pub module: String,
    pub symbol: String,
    iat_slot: *const T,
    real_fn: T,
}

unsafe impl<T: Clone> Send for IatStoreInternal<T> {}
unsafe impl<T: Clone> Sync for IatStoreInternal<T> {}

/// Closer to matching multi-threading possibilities
#[derive(Clone)]
#[repr(transparent)]
pub struct IatStore<T: Clone>(Arc<IatStoreInternal<T>>);

impl<T: Clone> IatStore<T> {
    pub fn get_fn(&self) -> T {
        return self.0.real_fn.clone();
    }
}

impl<T: Clone> Drop for IatStoreInternal<T> {
    /// Once dropped, return the original function so that
    /// the original purpose will be handled. Similar as in plthook's own replace
    fn drop(&mut self) {
        log::trace!(
            "Restoring symbol {} function from module {}",
            self.symbol,
            self.module
        );
        let res = replace_function_ptr(self.iat_slot as *mut T, &self.real_fn);
        if let Err(e) = res {
            log::error!("Failed to restore IAT slot: {}", e);
        }
    }
}

/// Replace the function of iat_slot with hook_ptr.
///
/// To do it, open up the memory for readwrite and replace
/// the function itself. Memory protection aversion could
/// be overdoing it, but in case of an error, it will be
/// clearer what happened.
fn replace_function_ptr<T>(iat_slot: *mut T, hook_fn: &T) -> Result<(), Box<dyn Error>> {
    unsafe {
        let mut old_protect = PAGE_PROTECTION_FLAGS::default();
        let size = std::mem::size_of::<*mut c_void>();
        let _ = VirtualProtect(
            iat_slot as *const c_void,
            size,
            PAGE_READWRITE,
            &mut old_protect,
        )?;

        // Overwrite with our hook function pointer
        *iat_slot = std::mem::transmute_copy(hook_fn);

        // Restore old protection
        let _ = VirtualProtect(
            iat_slot as *const c_void,
            size,
            old_protect,
            &mut old_protect,
        )?;
    }

    Ok(())
}

/// Installs a new hook for given symbol inside the module, while storing
/// the information in a IatStore object.
pub fn install_plt_hook<T: Clone>(
    module_name: &str,
    symbol_name: &str,
    function: &T,
) -> Result<IatStore<T>, Box<dyn Error>> {
    let object_file = ObjectFile::open_file(module_name)?;
    // Iterate over symbols to find the IAT slot for CreateFileA
    for symbol in object_file.symbols() {
        let sym_name = symbol.name.to_string_lossy();
        if sym_name == symbol_name {
            // or "_CreateFileA@28" on x86 if decorated
            let iat_slot = symbol.func_address as *mut T;
            let old_ptr = unsafe { std::mem::transmute_copy(&*iat_slot) };

            // This `func_address` is the *address of the pointer*
            // in the IAT. We must treat it as `*mut *mut c_void`
            // so we can overwrite the pointer inside that slot.
            log::trace!(
                "Found {symbol_name} IAT slot at 0x{:X}, pointing to => 0x{:X}",
                iat_slot as usize,
                std::ptr::addr_of!(old_ptr) as usize
            );

            replace_function_ptr::<T>(iat_slot, function)?;
            log::trace!(
                "IAT slot updated. Symbol {symbol_name} in module {module_name} now points to => 0x{:X}",
                std::ptr::addr_of!(*iat_slot) as usize
            );

            return Ok(IatStore(Arc::new(IatStoreInternal {
                module: module_name.to_string(),
                symbol: symbol_name.to_string(),
                real_fn: old_ptr,
                iat_slot: iat_slot as *const T,
            })));
        }
    }
    Err("Correct symbol not found".to_string().into())
}

#[cfg(feature = "liveqa_tests")]
#[path = "tests/iat_register_tests.rs"]
mod iat_register_tests;
