/// Utilities and traits that are usable for general rendering
///
use std::error::Error;
use std::os::raw::c_void;
use std::sync::LazyLock;
use std::sync::Mutex;

use windows::Win32::Graphics::OpenGL::wglGetCurrentContext;

use crate::graphics::get_gl_context;
use crate::graphics::setup_rendering;

use super::glstate::store_gl_state;
use super::GlContextStorage;

/// Trait needed to setup rendering
///
/// Handles all different states and quirks that are needed
/// for Kotor rendering in the current hook system
///
/// dyn-compatible
///
pub trait Rendable {
    /// Handle the setup for rending the given object
    ///
    /// This method should be callable multiple times
    /// per process. This is because kotor drops the
    /// OpenGL context when minimized or when playing a
    /// video. After being restored to the game, setup
    /// will be called again.
    ///
    fn setup(self: &mut Self) -> Result<(), Box<dyn Error>>;

    /// The render function for the actual render work
    ///
    /// Called per frame after the game's own rendering,
    /// allowing drawing on top of it.
    ///
    /// For convenience, the viewport dimensions are passed here.
    ///
    /// Handling context checks and gl state checks is done
    /// by the trait outside of this function. See `per_frame`
    ///
    fn render(self: &mut Self, viewport_width: f32, viewport_height: f32);

    /// Called when the OpenGL context is lost
    ///
    /// When called, OpenGL context was lost, so it would
    /// be smart to drop all things that are gl context
    /// based/dependant.
    ///
    fn deinit(self: &mut Self);

    /// The function called per-frame
    ///
    /// Handles the logic of running normal operation and
    /// setting up when needed.
    ///
    fn per_frame(self: &mut Self) {
        let mut initialization_guard = state_guard();
        let (initialized, stored_gl_context) = *initialization_guard;

        let current_gl_context = get_gl_context();

        if !current_gl_context.valid_context() {
            // No point in trying anything if there is no context.
            // SWKotor might drop the context, it's a bit unclear.
            return;
        }

        match (
            initialized,
            context_eq(&current_gl_context, &stored_gl_context),
        ) {
            (false, false) => {
                let res = setup_rendering();
                if let Err(e) = res {
                    log::error!("Rendering setup failed: {}", e);
                    return;
                }

                log::trace!("Got context. Setup.");
                let res = self.setup();
                if let Err(e) = res {
                    log::error!("Rendering setup failed: {}", e);
                    return;
                }

                *initialization_guard = (true, Some(get_gl_context()));
            }
            (true, false) => {
                // Context has disappeared on us. Deinitialize and fix on
                // next loop.
                // Context will most likely be dropped when
                // 1. Minimizing window
                // 2. A video is playing
                // At this point, there is no guarantee that we are running
                // in any context. Hence redo it on next frame.
                log::trace!("Lost context. Deinit.");
                self.deinit();
                *initialization_guard = (false, None);
                return;
            }
            // We've just received context. Setup
            (false, true) => {
                log::error!(
                    "Bug. We have a context but we are not initialized. Should be impossible"
                );
                panic!("Bug. We have a context but we are not initialized");
            }
            // Normal rending operation. Just go on
            (true, true) => (),
        }

        log::trace!("Render called, context ptr: {:p}", unsafe {
            wglGetCurrentContext().0 as *const c_void
        });

        let state_store = store_gl_state();
        let (width, height) = state_store.get_viewport();
        self.render(width, height);
    }
}

/// Just a helper function to make other code more readable
fn context_eq(lhv: &GlContextStorage, opt_rhv: &Option<GlContextStorage>) -> bool {
    match opt_rhv {
        None => false,
        Some(rhv) => lhv == rhv,
    }
}

/// Mutex guard to provide a state and mutex for the above functions
///
/// Kotor does single thread rendering, but any hooked function
/// should not be trusted to hold data. In order to
/// not make this difficult for consumer, have a global
/// mutex stored here.
///
/// That, plus the single threaded part is a bit questionable.
///
/// Also, the state does not need to be here. But for our modding
/// purposes, it should be fine.
///
fn state_guard() -> std::sync::MutexGuard<'static, (bool, Option<GlContextStorage>)> {
    static GUARD: LazyLock<Mutex<(bool, Option<GlContextStorage>)>> =
        LazyLock::new(|| Mutex::new((false, None)));

    match GUARD.lock() {
        Ok(guard) => guard,
        Err(_) => panic!("Failed to acquire rendering lock"),
    }
}
