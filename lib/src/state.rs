use std::sync::Mutex;
use crate::services::display_service::DisplayService;

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