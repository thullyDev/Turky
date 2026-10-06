use image::RgbaImage;

use crate::schemas::overlay_schemas::OverlayConfig;

use super::gif_cutter::{GifCutter, GifFrame};
use super::stats_painter::StatsPainter;
use super::system_stats_service::SystemStatsService;

pub struct OverlayService {
    cutter: GifCutter,
    stats: SystemStatsService,
    painter: StatsPainter,
    config: OverlayConfig,
}

impl OverlayService {
    pub fn new(cutter: GifCutter, stats: SystemStatsService, painter: StatsPainter) -> Self {
        Self {
            cutter,
            stats,
            painter,
            config: OverlayConfig::stacked(),
        }
    }

    pub fn set_config(&mut self, config: OverlayConfig) {
        self.config = config;
    }

    pub fn cut(&self, bytes: &[u8]) -> Result<Vec<GifFrame>, String> {
        self.cutter.cut(bytes)
    }

    pub fn apply(&mut self, frame: &RgbaImage) -> RgbaImage {
        if self.config.items.is_empty() {
            return frame.clone();
        }

        let stats = self.stats.collect();

        self.painter.paint(frame, &stats, &self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::overlay_schemas::OverlayConfig;
    use crate::services::system_stats_service::SystemStats;
    use gif::{Encoder, Frame, Repeat};
    use image::Rgba;

    fn black_gif() -> Vec<u8> {
        let mut bytes = Vec::new();

        {
            let mut encoder =
                Encoder::new(&mut bytes, 320, 80, &[]).expect("Failed to create GIF encoder");

            encoder
                .set_repeat(Repeat::Infinite)
                .expect("Failed to set GIF repeat");

            let pixels = vec![0u8; 320 * 80 * 3];
            let mut frame = Frame::from_rgb(320, 80, &pixels);

            frame.delay = 1;

            encoder
                .write_frame(&frame)
                .expect("Failed to write GIF frame");
        }

        bytes
    }

    fn service() -> OverlayService {
        OverlayService::new(
            GifCutter::new(),
            SystemStatsService::fixed(SystemStats {
                cpu_percent: 100.0,
                ..SystemStats::empty()
            }),
            StatsPainter::new(),
        )
    }

    #[test]
    fn cuts_the_gif_and_adds_current_stats_to_each_frame() {
        let mut overlay = service();
        let frames = overlay.cut(&black_gif()).expect("gif should cut");

        assert_eq!(frames.len(), 1);

        let painted = overlay.apply(&frames[0].image);

        assert_eq!(painted.dimensions(), frames[0].image.dimensions());

        let has_text = painted
            .pixels()
            .any(|pixel| pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200);

        assert!(has_text, "expected current stats on the frame");
        assert_eq!(frames[0].image.get_pixel(8, 8), &Rgba([0, 0, 0, 255]));
    }

    #[test]
    fn gui_config_chooses_which_stats_are_drawn() {
        let mut overlay = service();
        let frames = overlay.cut(&black_gif()).expect("gif should cut");

        overlay.set_config(OverlayConfig::default());

        let cleared = overlay.apply(&frames[0].image);

        assert_eq!(cleared.as_raw(), frames[0].image.as_raw());
    }
}
