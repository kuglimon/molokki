use std::error::Error;

use crate::graphics::handle_load_with;
use crate::graphics::rendering::Rendable;
use imgui::*;
use imgui_opengl_renderer::Renderer;

struct ImgUiContext {
    imgui: Option<Context>,
    renderer: Option<Renderer>,
}

unsafe impl Send for ImgUiContext {}
unsafe impl Sync for ImgUiContext {}

pub struct ImguiRendable {
    context: ImgUiContext,
}

unsafe impl Send for ImguiRendable {}
unsafe impl Sync for ImguiRendable {}

impl ImguiRendable {
    pub fn new() -> Self {
        Self {
            context: ImgUiContext {
                imgui: None,
                renderer: None,
            },
        }
    }
}

impl Rendable for ImguiRendable {
    fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        if let None = self.context.imgui {
            // Do not re-initialize imgui.
            let mut imgui = Context::create();
            // Disable saving state
            imgui.set_ini_filename(None);
            self.context.imgui = Some(imgui);
        }

        let imgui = self.context.imgui.as_mut().unwrap();

        let renderer = Renderer::new(imgui, |s| {
            // Does this need to actually be called every time?
            handle_load_with(s)
        });

        self.context.renderer = Some(renderer);
        Ok(())
    }

    fn render(&mut self, viewport_width: f32, viewport_height: f32) {
        let context = &mut self.context;
        let imgui = match &mut context.imgui {
            None => {
                log::trace!("Crash?! 3");
                log::error!("Bug. Render called but imgui does not exist");
                return;
            }
            Some(imgui) => imgui,
        };

        imgui.io_mut().display_size = [viewport_width, viewport_height];
        let ui = imgui.frame();

        let renderer = match &context.renderer {
            None => {
                log::error!("Bug. Render called but renderer does not exist");
                return;
            }
            Some(renderer) => renderer,
        };

        // Draw a simple window
        ui.window("Hello ImGui!")
            .size([300.0, 100.0], Condition::Always)
            .build(|| {
                ui.text("This is a test ImGui window.");
            });

        // Render UI
        renderer.render(imgui);
    }

    fn deinit(&mut self) {
        // Drop renderer, but keep imgui
        self.context.renderer = None;
    }
}
