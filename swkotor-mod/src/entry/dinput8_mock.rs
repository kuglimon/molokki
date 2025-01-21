/// This module is meant to wrap around the fact that swkotor tries
/// to load DINPUT8.dll before loading one from the system.
///
/// To make it accept our DINPUT8, the symbol "DirectInput8Create" must
/// be found
///
/// But to not make swkotor crash to a segfault (or other undefined behavior
/// ), we need to pass the function call to the actual file from the system
/// with correct parameters and everything.
///
use std::ffi::{c_void, CStr, CString};
use std::sync::OnceLock;
use windows::Win32::Foundation::{HINSTANCE, HMODULE, MAX_PATH};
use windows::Win32::System::SystemInformation::GetSystemDirectoryA;
use windows::Win32::System::LibraryLoader::{LoadLibraryA, GetProcAddress};
use windows::core::{GUID, HRESULT, PCSTR};

type REFIID = *const GUID;
type LPUNKNOWN = *mut core::ffi::c_void;

// We'll store the real function pointer in a static mut for demonstration.
// In a robust solution, you might initialize once during DllMain or similar.
type DirectInput8CreateFn = unsafe extern "system"
    fn(HINSTANCE, u32, *const GUID, *mut *mut c_void, *mut c_void) -> HRESULT;

// This OnceLock will store our function pointer once.
static REAL_DINPUT8_CREATE: OnceLock<DirectInput8CreateFn> = OnceLock::new();

fn get_system_directory_ansi() -> Result<String, String> {
    // Create a buffer of length MAX_PATH (260 bytes)
    let mut buffer = [0u8; MAX_PATH as usize];

    unsafe {
        let len = GetSystemDirectoryA(Some(buffer.as_mut()));
        if len == 0 {
            return Err("Failed to get system directory".to_string());
        }
        // Convert buffer to a &CStr up to the first null.
        let cstr = CStr::from_ptr(buffer.as_ptr() as *const i8);
        Ok(cstr.to_string_lossy().into_owned())
    }
}

// "system" calling convention is correct for most Windows API functions.
#[no_mangle]
pub extern "system" fn DirectInput8Create(
    hinst: HINSTANCE,
    dw_version: u32,
    riidltf: REFIID,
    ppv_out: *mut *mut c_void,
    punk_outer: LPUNKNOWN,
) -> HRESULT {
    log::trace!("Mocking DirectInput8Create");

    if REAL_DINPUT8_CREATE.get().is_none() {
        // Load the real dinput8.dll
        let system_dir = match get_system_directory_ansi() {
            Err(e) => {
                log::error!("Failed to get system directory {e}");
                return HRESULT(-2147467259i32) // HRESULT_FROM_WIN32(E_FAIL = 0x80004005)
            }
            Ok(dir) => dir
        };
        let dll_path = CString::new(format!("{system_dir}/dinput8.dll")).unwrap();
        let res = unsafe { LoadLibraryA(PCSTR(dll_path.as_ptr() as *const u8)) };

        let real_module = match res {
            Err(e) => {
                log::error!("Failed to load dinput8: {e}");
                return HRESULT(-2147024770i32); // HRESULT_FROM_WIN32(ERROR_MOD_NOT_FOUND = 0x8007007E)
            },
            Ok(module) => module.0
        };

        if real_module == std::ptr::null_mut() {
            // Fail if we can't load the real library
            log::error!("Loaded dinput8, but pointer was null");
            return HRESULT(-2147024770i32); // HRESULT_FROM_WIN32(ERROR_MOD_NOT_FOUND = 0x8007007E)
        }

        let proc_name = CString::new("DirectInput8Create").unwrap();
        let real_proc = unsafe { GetProcAddress(HMODULE(real_module), PCSTR(proc_name.as_ptr() as *const u8)) };
        if real_proc.is_none() {
            // Fail if not found
            log::error!("Failed to get DirectInput8Create address");
            return HRESULT(-2147024770i32); // HRESULT_FROM_WIN32(ERROR_MOD_NOT_FOUND = 0x8007007E)
        }
        unsafe {
            let real_func: DirectInput8CreateFn = std::mem::transmute(real_proc);
            let res = REAL_DINPUT8_CREATE.set(real_func);
            if res != Ok(()) {
                log::error!("Failed to set real function pointer");
                return HRESULT(-2147467259i32); // HRESULT_FROM_WIN32(E_FAIL = 0x80004005)
            }
        }
    }

    // Call the real function
    if let Some(real_func) = REAL_DINPUT8_CREATE.get() {
        unsafe {
            (real_func)(hinst, dw_version, riidltf, ppv_out, punk_outer)
        }
    } else {
        // Should never happen, but just in case
        return HRESULT(-2147467259i32) // HRESULT_FROM_WIN32(E_FAIL = 0x80004005)
    }
}