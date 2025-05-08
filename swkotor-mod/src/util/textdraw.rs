/// A PoC module that will just draw a square with some fake text in it
///
/// This assumes that swkotor uses old context/states from opengl,
/// roughly meaning that it is able to fetch the context and draw by
/// guessing the default state. It should work as it has been tested.
///
/// Note that at this moment, there is now actual text anywhere. It is
/// just a fake one, drawing some squares instead of characters.
///
use gl::types::*;

use crate::graphics::{opengl_bindings, rendering::Rendable};

pub struct TextdrawContext {
    font_text_id: GLuint,
}

unsafe impl Send for TextdrawContext {}
unsafe impl Sync for TextdrawContext {}

impl TextdrawContext {
    // Call this once to create a simple texture in OpenGL
    unsafe fn init_font_texture(&mut self) {
        // In a real scenario, load a .png or .dds from memory or resource
        // For brevity, let's pretend we have raw RGBA data or something.
        // We'll do a placeholder that just creates a white texture.
        let mut tex_id = 0;
        gl::GenTextures(1, &mut tex_id);
        gl::BindTexture(gl::TEXTURE_2D, tex_id);

        // Some placeholder 8x8 white image
        let pixels = vec![255u8; 8 * 8 * 4]; // RGBA=255 => white
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            8,
            8,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            pixels.as_ptr() as *const _,
        );

        // Set texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Unbind
        gl::BindTexture(gl::TEXTURE_2D, 0);

        self.font_text_id = tex_id;
    }

    unsafe fn begin_2d_drawing(&self, viewport_width: f32, viewport_height: f32) {
        // Save current matrices
        opengl_bindings::MatrixMode(opengl_bindings::PROJECTION);
        opengl_bindings::PushMatrix();
        opengl_bindings::LoadIdentity();

        // "Orthographic" from 0..viewport_width horizontally, 0..viewport_height vertically
        // We'll assume (0,0) is the top-left, (width,height) is bottom-right, so invert the Y.
        opengl_bindings::Ortho(
            0.0,
            viewport_width.into(),
            viewport_height.into(),
            0.0,
            -1.0,
            1.0,
        );

        opengl_bindings::MatrixMode(opengl_bindings::MODELVIEW);
        opengl_bindings::PushMatrix();
        opengl_bindings::LoadIdentity();

        // Possibly disable depth test, enable alpha blending, etc.
        gl::Disable(opengl_bindings::CULL_FACE);
        gl::Disable(opengl_bindings::LIGHTING);
        gl::Disable(gl::DEPTH_TEST);
        gl::Enable(opengl_bindings::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    unsafe fn draw_text_private(&self, mut x: f32, y: f32, text: &str) {
        // Bind our "font" texture
        gl::BindTexture(gl::TEXTURE_2D, self.font_text_id);

        gl::Enable(gl::TEXTURE_2D);

        opengl_bindings::Color4f(1.0, 1.0, 1.0, 1.0); // White color

        // Example: draw each character as an 8x8 quad, spaced 8 pixels
        // We'll skip actual ASCII logic. We'll just draw N times for each char.
        for (i, _ch) in text.chars().enumerate() {
            // In real code, compute the glyph texture coords for the char

            let r = if i % 2 == 0 { 1.0 } else { 0.0 };
            let g = if i % 2 == 1 { 1.0 } else { 0.0 };
            opengl_bindings::Color3f(r, g, 0.0);
            let quad_size = 8.0;
            let x2 = x + quad_size;
            let y2 = y + quad_size;

            opengl_bindings::Begin(gl::QUADS);
            // top-left
            opengl_bindings::TexCoord2f(0.0, 0.0);
            opengl_bindings::Vertex2f(x, y);

            // top-right
            opengl_bindings::TexCoord2f(1.0, 0.0);
            opengl_bindings::Vertex2f(x2, y);

            // bottom-right
            opengl_bindings::TexCoord2f(1.0, 1.0);
            opengl_bindings::Vertex2f(x2, y2);

            // bottom-left
            opengl_bindings::TexCoord2f(0.0, 1.0);
            opengl_bindings::Vertex2f(x, y2);
            opengl_bindings::End();

            x += quad_size; // Move for the next character
        }

        gl::Disable(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    unsafe fn end_2d_drawing(&self) {
        // Restore everything
        opengl_bindings::MatrixMode(opengl_bindings::MODELVIEW);
        opengl_bindings::PopMatrix();
        opengl_bindings::MatrixMode(opengl_bindings::PROJECTION);
        opengl_bindings::PopMatrix();
    }
}

pub struct TextdrawRendable {
    context: Option<TextdrawContext>,
}

unsafe impl Send for TextdrawRendable {}
unsafe impl Sync for TextdrawRendable {}

impl TextdrawRendable {
    pub fn new() -> Self {
        Self { context: None }
    }

    fn set(&mut self, ctx: TextdrawContext) {
        self.context = Some(ctx);
    }

    fn get(&mut self) -> &mut TextdrawContext {
        match self.context.as_mut() {
            Some(ctx) => ctx,
            None => {
                log::error!("Bug. Failed to get textdraw context.");
                panic!("Bug. Failed to get textdraw context.");
            }
        }
    }
}

impl Rendable for TextdrawRendable {
    fn setup(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        self.set(TextdrawContext { font_text_id: 0 });

        unsafe {
            self.get().init_font_texture();
        }
        Ok(())
    }

    fn render(self: &mut Self, viewport_width: f32, viewport_height: f32) {
        unsafe {
            let ctx = self.get();
            ctx.begin_2d_drawing(viewport_width, viewport_height);
            ctx.draw_text_private(0.0, 0.0, "Hello world");
            ctx.end_2d_drawing();
        };
    }

    fn deinit(self: &mut Self) {
        todo!()
    }
}
