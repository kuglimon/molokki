use std::{
    error::Error,
    sync::{LazyLock, Mutex},
};
/// A module to do IAT hook to SwapBuffers
///
/// This module is used to do IAT hook to SwapBuffers function in GDI32.dll.
/// Apparently, swkotor does not interface to the wglSpySwapBuffers function,
/// so we have to do IAT hook to SwapBuffers function in GDI32.dll.
///
/// This provides a per-frame hook into the rendering pipeline.
use windows::Win32::Foundation::{BOOL, FALSE, HANDLE};

use super::common::{install_plt_hook, IatStore};

type HDC = HANDLE;

type SwapBuffersFn = unsafe extern "system" fn(hdc: HDC) -> BOOL;

pub type SwapbufferCallback = Box<dyn FnMut() + Send + Sync>;

/// Struct that holds the IatStore and
struct SwapBufferIatContext {
    iat_store: IatStore<SwapBuffersFn>,
    iat_callback: SwapbufferCallback,
}
unsafe impl Send for SwapBufferIatContext {}
unsafe impl Sync for SwapBufferIatContext {}

impl SwapBufferIatContext {
    fn run_real_function(&self, hdc: HDC) -> BOOL {
        let func = self.iat_store.get_fn();
        unsafe { func(hdc) }
    }

    fn run_callback(&mut self) {
        (self.iat_callback)()
    }
}

static REAL_SWAPBUFFERS: LazyLock<Mutex<Option<SwapBufferIatContext>>> =
    LazyLock::new(|| Mutex::new(None));

fn set_real_swapbuffers(context: SwapBufferIatContext) -> Result<(), Box<dyn Error>> {
    let mut guard = REAL_SWAPBUFFERS.lock()?;
    *guard = Some(context);
    Ok(())
}

fn call_hooked_swapbuffers(handle: HANDLE) -> BOOL {
    let mut guard = match REAL_SWAPBUFFERS.lock() {
        Err(e) => {
            log::error!("Failed to lock in swapbuffers. {e}");
            return FALSE;
        }
        Ok(guard) => guard,
    };

    let ctx = match guard.as_mut() {
        None => {
            log::error!("Missing context. Cannot call SwapBuffers. Game is now broken");
            panic!("Game broken");
        }
        Some(context) => context,
    };

    // Run the semi-global callback set up elsewhere
    ctx.run_callback();

    ctx.run_real_function(handle)
}

// Our hooked SwapBuffers implementation
unsafe extern "system" fn my_swapbuffers(hdc: HDC) -> BOOL {
    call_hooked_swapbuffers(hdc)
}

pub fn hook_swapbuffers(callback: SwapbufferCallback) -> Result<(), Box<dyn Error>> {
    let store = install_plt_hook::<SwapBuffersFn>(
        "swkotor.exe",
        "SwapBuffers",
        &(my_swapbuffers as SwapBuffersFn),
    )?;

    set_real_swapbuffers(SwapBufferIatContext {
        iat_store: store,
        iat_callback: callback,
    })
}
