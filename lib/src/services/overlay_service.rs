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
    layer_bounds: Option<(u32, u32, u32, u32)>,
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
            layer_bounds: None,
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
        self.layer_bounds = None;
        self.layer_at = None;
    }

    pub fn set_fps(&mut self, fps: Option<f32>) {
        self.fps = fps;
    }

    pub fn refresh_layer(&mut self, width: u32, height: u32) -> Option<RgbaImage> {
        if self.config.items.is_empty() {
            self.layer = None;
            self.layer_bounds = None;
            self.layer_at = None;

            return None;
        }

        let stats = self.stats.collect();
        let blank = RgbaImage::new(width, height);
        let painted = self.painter.paint(&blank, &stats, &self.config, self.fps);

        self.store_layer(painted);

        self.layer.clone()
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
            let painted = self.painter.paint(&blank, &stats, &self.config, fps);

            self.store_layer(painted);
        }

        self.frames_since_sample += 1;

        let bounds = self.layer_bounds;
        let layer = self.layer.as_ref().expect("stats layer was just created");

        blit(frame, layer, bounds)
    }

    fn store_layer(&mut self, painted: RgbaImage) {
        self.layer_bounds = opaque_bounds(&painted);
        self.layer = Some(painted);
        self.layer_at = Some(Instant::now());

        #[cfg(test)]
        {
            self.refreshes += 1;
        }
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

}

fn opaque_bounds(image: &RgbaImage) -> Option<(u32, u32, u32, u32)> {
    let mut min_x = image.width();
    let mut min_y = image.height();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut any = false;

    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel[3] == 0 {
            continue;
        }

        any = true;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    if !any {
        return None;
    }

    Some((min_x, min_y, max_x - min_x + 1, max_y - min_y + 1))
}

fn blit(
    background: &RgbaImage,
    overlay: &RgbaImage,
    bounds: Option<(u32, u32, u32, u32)>,
) -> RgbaImage {
    let mut frame = background.clone();

    let Some((x0, y0, width, height)) = bounds else {
        return frame;
    };

    let frame_width = frame.width() as usize;
    let frame_height = frame.height() as usize;
    let overlay_width = overlay.width() as usize;
    let overlay_height = overlay.height() as usize;
    let src = overlay.as_raw();
    let dst = frame.as_mut();

    for row in 0..height as usize {
        let y = y0 as usize + row;

        if y >= overlay_height || y >= frame_height {
            continue;
        }

        for col in 0..width as usize {
            let x = x0 as usize + col;

            if x >= overlay_width || x >= frame_width {
                continue;
            }

            let src_index = (y * overlay_width + x) * 4;
            let alpha = src[src_index + 3] as u16;

            if alpha == 0 {
                continue;
            }

            let dst_index = (y * frame_width + x) * 4;

            if alpha == 255 {
                dst[dst_index..dst_index + 4].copy_from_slice(&src[src_index..src_index + 4]);

                continue;
            }

            let inverse = 255 - alpha;

            for channel in 0..4 {
                let blended = src[src_index + channel] as u16
                    + (dst[dst_index + channel] as u16 * inverse + 127) / 255;

                dst[dst_index + channel] = blended as u8;
            }
        }
    }

    frame
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

    #[test]
    fn stats_layer_bounds_cover_the_text_and_not_the_whole_frame() {
        let mut overlay = service();
        let frame = RgbaImage::new(320, 80);

        let _painted = overlay.compose(&frame);
        let (x, y, width, height) = overlay.layer_bounds.expect("text should have bounds");

        assert!(x < 320);
        assert!(y < 80);
        assert!(width < 320, "bounds should skip the empty part of the frame");
        assert!(height < 80, "bounds should skip the empty part of the frame");
        assert!(x + width <= 320);
        assert!(y + height <= 80);
    }
}
