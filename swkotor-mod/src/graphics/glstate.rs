/// Module to handle gl state
///
/// There is a high chance that any renderings that happen
/// break the opengl state. Broken state means that there
/// will be graphical glitches. To combat this, store the
/// state and pop it after any operations

/// Holder for the glstate waiting for drop
///
/// Stores the state until the consumer drops it, restoring
/// OpenGL state.
pub struct GLStateStore {
    // Add fields to store OpenGL state here
    // For example:
    blend: bool,
    depth_test: bool,
    scissor_test: bool,
    viewport: [i32; 4],
}

impl GLStateStore {
    /// Save the current OpenGL state
    pub fn save() -> Self {
        let mut viewport = [0; 4];
        unsafe {
            gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr());
        }
        GLStateStore {
            blend: unsafe { gl::IsEnabled(gl::BLEND) == gl::TRUE },
            depth_test: unsafe { gl::IsEnabled(gl::DEPTH_TEST) == gl::TRUE },
            scissor_test: unsafe { gl::IsEnabled(gl::SCISSOR_TEST) == gl::TRUE },
            viewport,
        }
    }

    /// QoL function to get viewport sizes
    ///
    /// As we must have it stored, it can be more convenient to get the
    /// full viewport with this function.
    ///
    /// # Returns
    ///
    /// viewport size (x,y)
    ///
    pub fn get_viewport(&self) -> (f32, f32) {
        (self.viewport[2] as f32, self.viewport[3] as f32)
    }

    /// Restore the saved OpenGL state
    fn restore(&self) {
        if self.blend {
            unsafe { gl::Enable(gl::BLEND) };
        } else {
            unsafe { gl::Disable(gl::BLEND) };
        }

        if self.depth_test {
            unsafe { gl::Enable(gl::DEPTH_TEST) };
        } else {
            unsafe { gl::Disable(gl::DEPTH_TEST) };
        }

        if self.scissor_test {
            unsafe { gl::Enable(gl::SCISSOR_TEST) };
        } else {
            unsafe { gl::Disable(gl::SCISSOR_TEST) };
        }

        unsafe {
            gl::Viewport(
                self.viewport[0],
                self.viewport[1],
                self.viewport[2],
                self.viewport[3],
            );
        }
    }
}

impl Drop for GLStateStore {
    fn drop(&mut self) {
        self.restore();
    }
}

/// Store the state in OpenGL
///
/// When we do rendering, there is a high chance that the
pub fn store_gl_state() -> GLStateStore {
    GLStateStore::save()
}

/// Manually restore the OpenGL state
pub fn restore_gl_state(store: GLStateStore) {
    drop(store)
}
