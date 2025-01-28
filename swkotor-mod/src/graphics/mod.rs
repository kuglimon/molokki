/// Module to house the OpenGL bindings and related generic
/// functions.
pub mod glstate;
pub mod opengl_bindings;
pub mod rendering;

use rendering::Rendable;
use std::sync::Mutex;
use std::{
    error::Error,
    ffi::{c_void, CString},
};
use windows::Win32::Graphics::OpenGL::HGLRC;
use windows::{
    core::PCSTR,
    Win32::{
        Graphics::OpenGL::{wglGetCurrentContext, wglGetProcAddress},
        System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
    },
};

use crate::util::iat::swapbuffers::hook_swapbuffers;
use std::sync::Arc;

/// Struct to store glContext
///
/// Will be used for comparisons.
#[derive(Clone, Copy)]
pub struct GlContextStorage {
    context: HGLRC,
}

unsafe impl Send for GlContextStorage {}
unsafe impl Sync for GlContextStorage {}

impl GlContextStorage {
    fn valid_context(&self) -> bool {
        !self.context.is_invalid()
    }
}

impl PartialEq for GlContextStorage {
    fn eq(&self, other: &Self) -> bool {
        self.context.0 == other.context.0
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

/// Get the OpenGL context storage object
///
/// As is, it can just be used as an identifier on if the context
/// matches with another one.
///
pub fn get_gl_context() -> GlContextStorage {
    GlContextStorage {
        context: unsafe { wglGetCurrentContext() },
    }
}

/// Check if the OpenGL context is ready for rendering.
///
/// During startup movies and loading screens the GL context
/// might not be available. This should be called before trying any
/// rendering operations. Just to be safe.
///
pub fn context_available() -> bool {
    let invalid_context = unsafe { wglGetCurrentContext() }.is_invalid();
    !invalid_context
}

pub struct RendingStore {
    rendable: Arc<Mutex<Box<dyn Rendable + Send + Sync>>>,
}

/// The main generic initialization function to enable rendering
///
/// Specifically for OpenGL, we need to be able to setup context
/// for rendering for our own module. The basic is that this sets
/// up opengl for rendering and sets up a hook to be run for each
/// frame. This allows us to display items over the game, as we
/// are doing it last.
///
/// Note. One trick about this is that it IAT hooks to the SwapBuffer.
/// Then on the first frame (and when it absolutely has valid context),
/// it will enable drawing.
///
/// Note that an improvement would be to hook onto `wglMakeCurrent` or
/// similar that are called when a context is created. But these are
/// not yet tested.
///
/// # Parameters
///
/// * init_cb: Callback function that will be called once per starting
///            rendering. Should be used for one-time initialization of
///            rendering specific contexts/variables
/// * per_frame_cb: Callback called per-frame. Not that slow processing
///                 will equally drop FPS of the game.
///
///
pub fn initialize_pending_setup_rendering(
    rendable: Box<dyn Rendable + Send + Sync>,
) -> RendingStore {
    let rendable = RendingStore {
        rendable: Arc::new(Mutex::new(rendable)),
    };

    {
        let rendable_clone = Arc::clone(&rendable.rendable);
        let res = hook_swapbuffers(Box::new({
            move || {
                let mut rendable = rendable_clone.lock().unwrap();
                rendable.per_frame();
            }
        }));
        if let Err(e) = res {
            log::error!("Failed to hook swapbuffers: {:?}", e);
        }
    }

    rendable
}

/// Handles module loading as required by opengl
///
/// There are a few places where this is needed when using different
/// implementations. Provide a common mechanism here.
///
pub fn handle_load_with(module_name: &str) -> *const c_void {
    let cstr = CString::new(module_name).unwrap();
    let res = unsafe { wglGetProcAddress(PCSTR(cstr.as_ptr() as *const u8)) };

    if let Some(p) = res {
        // Function found, no need to fallback.
        return p as *const c_void;
    }

    // Fall back. Apparently, in case of opengl, this is good practice. No idea how...
    let module = unsafe { GetModuleHandleA(PCSTR::from_raw("opengl32.dll\0".as_ptr())) };
    if module.is_err() {
        log::trace!("Failed to load module {module_name}");
        return std::ptr::null();
    }

    let module = module.unwrap();
    let res = unsafe { GetProcAddress(module, PCSTR(cstr.as_ptr() as *const u8)) };
    if res.is_none() {
        log::trace!("Failed to load module {module_name}");
        return std::ptr::null();
    }
    res.unwrap() as *const c_void
}

/// The function that should be called when having a new OpenGL context
///
/// There might be some misinformation here.
///
/// The OpenGL context requires the setup to be run every time. This
/// enables the functions from the module to be run against the current
/// context.
///
pub fn setup_rendering() -> Result<(), Box<dyn Error>> {
    if !context_available() {
        // No context yet ready
        return Err("No OpenGL context yet".into());
    }

    // somewhere after you have a valid OpenGL context
    log::trace!("Trying to enable drawing with OpenGL. Calling load_with");
    gl::load_with(|s| handle_load_with(s));
    log::trace!("Successfully did not crash when calling load_with");
    Ok(())
}
