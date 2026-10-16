use ab_glyph::FontArc;
use image::RgbaImage;

use crate::schemas::overlay_schemas::OverlayConfig;
use super::system_stats_service::SystemStats;
use super::widget_painter;

pub struct StatsPainter {
    regular: FontArc,
    bold: FontArc,
}

impl StatsPainter {
    pub fn new() -> Self {
        let regular = FontArc::try_from_slice(include_bytes!(
            "../../assets/fonts/Roboto-Regular.ttf"
        ))
        .expect("bundled Roboto font should load");

        let bold = FontArc::try_from_slice(include_bytes!(
            "../../assets/fonts/Roboto-Bold.ttf"
        ))
        .expect("bundled Roboto Bold font should load");

        Self { regular, bold }
    }

    pub fn paint(
        &self,
        frame: &RgbaImage,
        stats: &SystemStats,
        config: &OverlayConfig,
        fps: Option<f32>,
    ) -> RgbaImage {
        let mut image = frame.clone();

        for item in &config.items {
            let size = item.value_size();

            if !size.is_finite() || size <= 0.0 {
                continue;
            }

            widget_painter::draw_widget(
                &mut image,
                item,
                stats,
                fps,
                &self.regular,
                &self.bold,
            );
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::overlay_schemas::{OverlayConfig, OverlayMetric};
    use image::{Rgba, RgbaImage};

    #[test]
    fn paints_cpu_text_onto_the_frame() {
        let frame = RgbaImage::from_pixel(320, 200, Rgba([0, 0, 0, 255]));
        let stats = SystemStats {
            cpu_percent: 100.0,
            ..SystemStats::empty()
        };

        let painted = StatsPainter::new().paint(&frame, &stats, &OverlayConfig::stacked(), None);

        let has_text = painted
            .pixels()
            .any(|pixel| pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200);

        assert!(has_text, "expected white stats text");
        assert_eq!(painted.dimensions(), frame.dimensions());
    }

    #[test]
    fn paints_bar_widget_pixels() {
        let frame = RgbaImage::from_pixel(320, 200, Rgba([0, 0, 0, 255]));
        let mut config = OverlayConfig::stacked();
        config.items.retain(|item| item.metric == OverlayMetric::Cpu);
        config.items[0].widget.kind = crate::schemas::overlay_schemas::WidgetKind::Bar;
        config.items[0].widget.show_label = false;
        config.items[0].widget.show_value = false;

        let stats = SystemStats {
            cpu_percent: 100.0,
            ..SystemStats::empty()
        };

        let painted = StatsPainter::new().paint(&frame, &stats, &config, None);

        let has_fill = painted
            .pixels()
            .any(|pixel| pixel[0] > 120 && pixel[2] > 120);

        assert!(has_fill, "expected bar fill pixels");
    }
}
