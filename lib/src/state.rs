use std::sync::atomic::{AtomicU32, AtomicU64};
use std::sync::{Arc, Mutex};

use crate::services::creation_service::CreationService;
use crate::services::display_service::DisplayService;
use crate::services::font_service::FontService;

pub struct AppState {
    pub display: Mutex<DisplayService>,
    pub presented: Arc<AtomicU32>,
    pub playback: Arc<AtomicU64>,
    pub fonts: FontService,
    pub creations: CreationService,
}

impl AppState {
    pub fn new(display: DisplayService, fonts: FontService, creations: CreationService) -> Self {
        Self {
            display: Mutex::new(display),
            presented: Arc::new(AtomicU32::new(0)),
            playback: Arc::new(AtomicU64::new(0)),
            fonts,
            creations,
        }
    }
}
