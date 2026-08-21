use crate::schemas::display_schemas::RenderImageTextResponse;

pub struct DisplayService;

impl DisplayService {
    pub fn new() -> Self {
        Self
    }

    pub fn render_text_image(&self, message: String) -> RenderImageTextResponse {
        println!("here {}", message);
        RenderImageTextResponse {
            message
        }
    }
}