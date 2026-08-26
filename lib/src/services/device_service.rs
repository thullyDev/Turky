use crate::schemas::display_schemas::RenderImageTextResponse;

pub struct DeviceService;

impl DeviceService {
    pub fn new(registry: DeviceRegistry) -> Self {
        Self {
            registry: registry
        }
    }
}

