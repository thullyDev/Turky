use ab_glyph::{FontArc, PxScale};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

use crate::schemas::overlay_schemas::{OverlayConfig, OverlayItem, OverlayMetric};
use super::system_stats_service::SystemStats;

pub struct StatsPainter {
    font: FontArc,
}

impl StatsPainter {
    pub fn new() -> Self {
        let font = FontArc::try_from_slice(include_bytes!(
            "../../assets/fonts/Roboto-Regular.ttf"
        ))
        .expect("bundled Roboto font should load");

        Self { font }
    }

    pub fn paint(
        &self,
        frame: &RgbaImage,
        stats: &SystemStats,
        config: &OverlayConfig,
    ) -> RgbaImage {
        let mut image = frame.clone();

        for item in &config.items {
            self.draw_item(&mut image, item, stats);
        }

        image
    }

    fn draw_item(&self, image: &mut RgbaImage, item: &OverlayItem, stats: &SystemStats) {
        if !item.size.is_finite() || item.size <= 0.0 {
            return;
        }

        let text = self.line(item.metric, stats);

        draw_text_mut(
            image,
            Rgba([item.color.r, item.color.g, item.color.b, item.color.a]),
            item.position.x,
            item.position.y,
            PxScale::from(item.size),
            &self.font,
            &text,
        );
    }

    fn line(&self, metric: OverlayMetric, stats: &SystemStats) -> String {
        match metric {
            OverlayMetric::Cpu => format!("CPU {:.0}%", stats.cpu_percent),
            OverlayMetric::Ram => self.format_ram(stats),
            OverlayMetric::Gpu => self.format_percent("GPU", stats.gpu_percent),
            OverlayMetric::GpuMemory => self.format_memory(
                "VRAM",
                stats.gpu_memory_used_bytes,
                stats.gpu_memory_total_bytes,
            ),
            OverlayMetric::CpuTemp => self.format_temp("CPU", stats.cpu_temp_celsius),
            OverlayMetric::GpuTemp => self.format_temp("GPU", stats.gpu_temp_celsius),
        }
    }

    fn format_ram(&self, stats: &SystemStats) -> String {
        format!(
            "RAM {}/{} GB",
            self.gigabytes(stats.ram_used_bytes),
            self.gigabytes(stats.ram_total_bytes)
        )
    }

    fn format_percent(&self, label: &str, value: Option<f32>) -> String {
        match value {
            Some(value) => format!("{label} {value:.0}%"),
            None => format!("{label} --"),
        }
    }

    fn format_memory(&self, label: &str, used: Option<u64>, total: Option<u64>) -> String {
        match (used, total) {
            (Some(used), Some(total)) => {
                format!("{label} {}/{} GB", self.gigabytes(used), self.gigabytes(total))
            }
            _ => format!("{label} --"),
        }
    }

    fn format_temp(&self, label: &str, value: Option<f32>) -> String {
        match value {
            Some(value) => format!("{label} {value:.0}°C"),
            None => format!("{label} --"),
        }
    }

    fn gigabytes(&self, bytes: u64) -> String {
        format!("{:.1}", bytes as f64 / 1_073_741_824.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::overlay_schemas::OverlayConfig;

    #[test]
    fn paints_cpu_text_onto_the_frame() {
        let frame = RgbaImage::from_pixel(320, 200, Rgba([0, 0, 0, 255]));
        let stats = SystemStats {
            cpu_percent: 100.0,
            ..SystemStats::empty()
        };

        let painted = StatsPainter::new().paint(&frame, &stats, &OverlayConfig::stacked());

        let has_text = painted
            .pixels()
            .any(|pixel| pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200);

        assert!(has_text, "expected white stats text");
        assert_eq!(painted.dimensions(), frame.dimensions());
    }
}
