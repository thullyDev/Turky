use crate::services::display_service::DisplayService;

pub struct AppState {
    pub display: DisplayService,
}


impl AppState {
    pub fn new(display: DisplayService) -> Self {
        Self {
            display
        }
    }
}