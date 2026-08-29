use crate::{schemas::display_schemas::RenderImageTextResponse, services::device_service::DeviceService};

pub struct DisplayService {
    device_service: DeviceService,
}

impl DisplayService {
    pub fn new(device_service: DeviceService) -> Self {
        Self {
            device_service
        }
    }

    pub fn render_text_image(&self, message: String) -> RenderImageTextResponse {
        println!("here {}", message);
        RenderImageTextResponse {
            message
        }
    }
}