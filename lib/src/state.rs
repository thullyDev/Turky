use crate::services::display_service::DisplayService;
use std::sync::Mutex;

pub struct AppState {
    pub display: Mutex<DisplayService>,
}

impl AppState {
    pub fn new(display: DisplayService) -> Self {
        Self {
            display: Mutex::new(display),
        }
    }
}
