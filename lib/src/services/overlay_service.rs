use std::time::{Duration, Instant};

use image::RgbaImage;

use crate::schemas::overlay_schemas::OverlayConfig;

use super::gif_cutter::{GifCutter, GifFrame};
use super::stats_painter::StatsPainter;
use super::system_stats_service::SystemStatsService;

const STATS_INTERVAL: Duration = Duration::from_secs(2);

pub struct OverlayService {
    cutter: GifCutter,
    stats: SystemStatsService,
    painter: StatsPainter,
    config: OverlayConfig,
    layer: Option<RgbaImage>,
    layer_at: Option<Instant>,
    frames_since_sample: u32,
    sample_started: Option<Instant>,
    fps: Option<f32>,
    #[cfg(test)]
    refreshes: u32,
}

impl OverlayService {
    pub fn new(cutter: GifCutter, stats: SystemStatsService, painter: StatsPainter) -> Self {
        Self {
            cutter,
            stats,
            painter,
            config: OverlayConfig::stacked(),
            layer: None,
            layer_at: None,
            frames_since_sample: 0,
            sample_started: None,
            fps: None,
            #[cfg(test)]
            refreshes: 0,
        }
    }

    pub fn set_config(&mut self, config: OverlayConfig) {
        self.config = config;
        self.layer = None;
        self.layer_at = None;
    }

    pub fn cut(&self, bytes: &[u8]) -> Result<Vec<GifFrame>, String> {
        self.cutter.cut(bytes)
    }

    #[cfg(test)]
    pub fn apply(&mut self, frame: &RgbaImage) -> RgbaImage {
        if self.config.items.is_empty() {
            return frame.clone();
        }

        let stats = self.stats.collect();

        self.painter.paint(frame, &stats, &self.config, self.fps)
    }

    pub fn compose(&mut self, frame: &RgbaImage) -> RgbaImage {
        if self.config.items.is_empty() {
            return frame.clone();
        }

        if self.stats_are_due(frame) {
            let fps = self.measured_fps();
            let stats = self.stats.collect();
            let blank = RgbaImage::new(frame.width(), frame.height());

            self.layer = Some(self.painter.paint(&blank, &stats, &self.config, fps));
            self.layer_at = Some(Instant::now());

            #[cfg(test)]
            {
                self.refreshes += 1;
            }
        }

        self.frames_since_sample += 1;

        let layer = self.layer.as_ref().expect("stats layer was just created");

        self.blit(frame, layer)
    }

    fn stats_are_due(&self, frame: &RgbaImage) -> bool {
        let Some(layer) = &self.layer else {
            return true;
        };

        if layer.width() != frame.width() || layer.height() != frame.height() {
            return true;
        }

        self.layer_at
            .is_none_or(|painted_at| painted_at.elapsed() >= STATS_INTERVAL)
    }

    fn measured_fps(&mut self) -> Option<f32> {
        let frames = self.frames_since_sample;

        self.frames_since_sample = 0;

        let started = self.sample_started.replace(Instant::now());

        let Some(started) = started else {
            return self.fps;
        };

        let seconds = started.elapsed().as_secs_f32();

        if seconds <= 0.0 || frames == 0 {
            return self.fps;
        }

        let fps = frames as f32 / seconds;

        self.fps = Some(fps);

        Some(fps)
    }

    fn blit(&self, background: &RgbaImage, overlay: &RgbaImage) -> RgbaImage {
        let mut frame = background.clone();

        for (x, y, src) in overlay.enumerate_pixels() {
            let src_a = src[3] as u16;

            if src_a == 0 {
                continue;
            }

            let dst = frame.get_pixel_mut(x, y);

            if src_a == 255 {
                *dst = *src;

                continue;
            }

            let inv = 255 - src_a;

            for channel in 0..4 {
                let blended = src[channel] as u16 + (dst[channel] as u16 * inv + 127) / 255;

                dst[channel] = blended as u8;
            }
        }

        frame
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

    #[test]
    fn compose_keeps_the_gif_and_reuses_stats_for_two_seconds() {
        let mut overlay = service();
        let frames = overlay.cut(&black_gif()).expect("gif should cut");
        let background = RgbaImage::from_pixel(320, 80, Rgba([255, 0, 0, 255]));

        let first = overlay.compose(&background);
        let second = overlay.compose(&frames[0].image);

        assert_eq!(overlay.refreshes, 1);
        assert_eq!(first.get_pixel(300, 70), &Rgba([255, 0, 0, 255]));

        let has_text = first
            .pixels()
            .any(|pixel| pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200);

        assert!(has_text, "expected cached stats on the gif frame");
        assert_ne!(second.as_raw(), frames[0].image.as_raw());
    }
}
