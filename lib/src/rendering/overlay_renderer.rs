use ab_glyph::PxScale;
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

use super::font_registry::FontRegistry;
use super::overlay_formatter::OverlayFormatter;
use super::text_layout::TextLayout;
use crate::schemas::overlay_schemas::{OverlayConfig, OverlayItem};
use crate::services::system_stats_service::SystemStats;

pub struct OverlayRenderer {
    fonts: FontRegistry,
    formatter: OverlayFormatter,
    layout: TextLayout,
    layer: Option<RgbaImage>,
    layer_key: Option<LayerKey>,
    #[cfg(test)]
    redraws: u32,
}

#[derive(PartialEq)]
struct LayerKey {
    width: u32,
    height: u32,
    config: OverlayConfig,
    lines: Vec<String>,
}

impl OverlayRenderer {
    pub fn new(fonts: FontRegistry, formatter: OverlayFormatter, layout: TextLayout) -> Self {
        Self {
            fonts,
            formatter,
            layout,
            layer: None,
            layer_key: None,
            #[cfg(test)]
            redraws: 0,
        }
    }

    pub fn register_font(&mut self, name: String, bytes: Vec<u8>) -> Result<(), String> {
        self.fonts.register(name, bytes)?;
        self.layer = None;
        self.layer_key = None;

        Ok(())
    }

    pub fn font_names(&self) -> Vec<String> {
        self.fonts.names()
    }

    pub fn render(
        &mut self,
        background: &RgbaImage,
        stats: &SystemStats,
        config: &OverlayConfig,
    ) -> RgbaImage {
        if config.items.is_empty() {
            return background.clone();
        }

        self.ensure_layer(background.width(), background.height(), stats, config);

        let layer = self.layer.as_ref().expect("stats layer was just created");

        composite(background, layer)
    }

    pub fn render_layer(
        &mut self,
        width: u32,
        height: u32,
        stats: &SystemStats,
        config: &OverlayConfig,
    ) -> Option<RgbaImage> {
        if config.items.is_empty() || !self.ensure_layer(width, height, stats, config) {
            return None;
        }

        self.layer.clone()
    }

    fn ensure_layer(
        &mut self,
        width: u32,
        height: u32,
        stats: &SystemStats,
        config: &OverlayConfig,
    ) -> bool {
        let key = LayerKey {
            width,
            height,
            config: config.clone(),
            lines: config
                .items
                .iter()
                .map(|item| self.formatter.format(item, stats))
                .collect(),
        };

        if self.layer_key.as_ref() == Some(&key) {
            return false;
        }

        self.layer = Some(self.draw_layer(width, height, stats, config));
        self.layer_key = Some(key);

        #[cfg(test)]
        {
            self.redraws += 1;
        }

        true
    }

    fn draw_layer(
        &self,
        width: u32,
        height: u32,
        stats: &SystemStats,
        config: &OverlayConfig,
    ) -> RgbaImage {
        let mut layer = RgbaImage::new(width, height);

        for item in &config.items {
            self.draw_item(&mut layer, item, stats);
        }

        layer
    }

    fn draw_item(&self, image: &mut RgbaImage, item: &OverlayItem, stats: &SystemStats) {
        if !item.size.is_finite() || item.size <= 0.0 {
            return;
        }

        let text = self.formatter.format(item, stats);

        if text.is_empty() {
            return;
        }

        let font = self.fonts.get(&item.font);
        let (width, height) = self.layout.size(font, item.size, &text);
        let (x, y) = self.layout.origin(
            item.position.anchor,
            item.position.x,
            item.position.y,
            width,
            height,
        );

        draw_text_mut(
            image,
            Rgba([item.color.r, item.color.g, item.color.b, item.color.a]),
            x,
            y,
            PxScale::from(item.size),
            font,
            &text,
        );
    }
}

fn composite(background: &RgbaImage, overlay: &RgbaImage) -> RgbaImage {
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::schemas::overlay_schemas::{OverlayColor, OverlayMetric, OverlayPosition, TextAnchor};

    fn renderer() -> OverlayRenderer {
        OverlayRenderer::new(
            FontRegistry::bundled(),
            OverlayFormatter::new(),
            TextLayout::new(),
        )
    }

    fn stats() -> SystemStats {
        SystemStats {
            cpu_percent: 12.4,
            ram_used_bytes: 8_589_934_592,
            ram_total_bytes: 17_179_869_184,
            gpu_percent: Some(34.2),
            gpu_memory_used_bytes: Some(1_288_490_189),
            gpu_memory_total_bytes: Some(8_589_934_592),
            cpu_temp_celsius: Some(61.2),
            gpu_temp_celsius: None,
        }
    }

    fn item(metric: OverlayMetric, anchor: TextAnchor, x: i32, y: i32) -> OverlayItem {
        OverlayItem {
            metric,
            position: OverlayPosition { x, y, anchor },
            font: "Roboto".to_string(),
            size: 28.0,
            color: OverlayColor {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
            text: String::new(),
        }
    }

    fn config(item: OverlayItem) -> OverlayConfig {
        OverlayConfig { items: vec![item] }
    }

    fn ink_bounds(image: &RgbaImage) -> (u32, u32, u32, u32) {
        let mut min_x = image.width();
        let mut min_y = image.height();
        let mut max_x = 0;
        let mut max_y = 0;
        let mut found = false;

        for (x, y, pixel) in image.enumerate_pixels() {
            if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
                continue;
            }

            found = true;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        assert!(found, "expected overlay text to change pixels");

        (min_x, min_y, max_x, max_y)
    }

    #[test]
    fn empty_config_leaves_the_image_unchanged() {
        let mut renderer = renderer();
        let source = RgbaImage::from_pixel(8, 8, Rgba([1, 2, 3, 255]));

        let rendered = renderer.render(&source, &stats(), &OverlayConfig::default());

        assert_eq!(rendered.as_raw(), source.as_raw());
    }

    #[test]
    fn draws_cpu_text_in_the_requested_color() {
        let mut renderer = renderer();
        let source = RgbaImage::from_pixel(320, 80, Rgba([0, 0, 0, 255]));

        let mut overlay = item(OverlayMetric::Cpu, TextAnchor::TopLeft, 8, 8);

        overlay.color = OverlayColor {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };

        let rendered = renderer.render(&source, &stats(), &config(overlay));

        let colored = rendered.pixels().any(|pixel| pixel[0] > 200 && pixel[1] < 40 && pixel[2] < 40);

        assert!(colored, "expected red overlay text");
    }

    #[test]
    fn top_right_anchor_draws_on_the_right() {
        let mut renderer = renderer();
        let source = RgbaImage::from_pixel(400, 80, Rgba([0, 0, 0, 255]));
        let overlay = item(OverlayMetric::Cpu, TextAnchor::TopRight, 400, 0);

        let rendered = renderer.render(&source, &stats(), &config(overlay));
        let (min_x, _min_y, max_x, _max_y) = ink_bounds(&rendered);

        assert!(min_x > source.width() / 2, "text started too far left: {min_x}");
        assert!(max_x > source.width() * 3 / 4, "text did not reach the right side: {max_x}");
    }

    #[test]
    fn top_left_anchor_draws_on_the_left() {
        let mut renderer = renderer();
        let source = RgbaImage::from_pixel(400, 80, Rgba([0, 0, 0, 255]));
        let overlay = item(OverlayMetric::Cpu, TextAnchor::TopLeft, 0, 0);

        let rendered = renderer.render(&source, &stats(), &config(overlay));
        let (min_x, min_y, max_x, _max_y) = ink_bounds(&rendered);

        assert!(min_x < 10, "text did not start at the left: {min_x}");
        assert!(min_y < 20, "text did not start at the top: {min_y}");
        assert!(max_x < source.width() / 2, "text reached too far right: {max_x}");
    }

    #[test]
    fn missing_font_uses_the_bundled_font() {
        let mut renderer = renderer();
        let source = RgbaImage::from_pixel(320, 80, Rgba([0, 0, 0, 255]));

        let bundled = item(OverlayMetric::Cpu, TextAnchor::TopLeft, 8, 8);
        let mut missing = bundled.clone();

        missing.font = "Missing".to_string();

        let with_bundled = renderer.render(&source, &stats(), &config(bundled));
        let with_missing = renderer.render(&source, &stats(), &config(missing));

        assert_eq!(with_bundled.as_raw(), with_missing.as_raw());
    }

    #[test]
    fn stats_layer_sits_on_top_of_the_gif_frame() {
        let mut renderer = renderer();
        let background = RgbaImage::from_pixel(320, 80, Rgba([255, 0, 0, 255]));
        let overlay = config(item(OverlayMetric::Cpu, TextAnchor::TopLeft, 4, 4));

        let rendered = renderer.render(&background, &stats(), &overlay);

        assert_eq!(rendered.get_pixel(300, 70), &Rgba([255, 0, 0, 255]));

        let has_text = rendered
            .pixels()
            .any(|pixel| pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200);

        assert!(has_text, "expected stats text on top of the gif frame");
    }

    #[test]
    fn reuses_the_stats_layer_until_the_text_changes() {
        let mut renderer = renderer();
        let background = RgbaImage::from_pixel(320, 80, Rgba([0, 0, 0, 255]));
        let overlay = config(item(OverlayMetric::Cpu, TextAnchor::TopLeft, 8, 8));

        let mut same_text = stats();
        same_text.cpu_percent = 12.2;

        renderer.render(&background, &same_text, &overlay);

        same_text.cpu_percent = 12.6;

        renderer.render(&background, &same_text, &overlay);

        assert_eq!(renderer.redraws, 1);

        same_text.cpu_percent = 80.0;

        renderer.render(&background, &same_text, &overlay);

        assert_eq!(renderer.redraws, 2);
    }
}
