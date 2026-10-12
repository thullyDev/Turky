use crate::services::display_service::DisplayService;
use std::sync::atomic::{AtomicU32, AtomicU64};
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub display: Mutex<DisplayService>,
    pub presented: Arc<AtomicU32>,
    pub playback: Arc<AtomicU64>,
}

impl AppState {
    pub fn new(display: DisplayService) -> Self {
        Self {
            display: Mutex::new(display),
            presented: Arc::new(AtomicU32::new(0)),
            playback: Arc::new(AtomicU64::new(0)),
        }
    }
}
