use crate::schemas::overlay_schemas::{OverlayItem, OverlayMetric};
use crate::services::system_stats_service::SystemStats;

pub struct OverlayFormatter;

impl OverlayFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format(&self, item: &OverlayItem, stats: &SystemStats) -> String {
        match item.metric {
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
            OverlayMetric::Text => item.text.clone(),
        }
    }

    fn format_percent(&self, label: &str, value: Option<f32>) -> String {
        match value {
            Some(value) => format!("{label} {value:.0}%"),
            None => format!("{label} --"),
        }
    }

    fn format_temp(&self, label: &str, value: Option<f32>) -> String {
        match value {
            Some(value) => format!("{label} {value:.0}°C"),
            None => format!("{label} --"),
        }
    }

    fn format_ram(&self, stats: &SystemStats) -> String {
        if stats.ram_total_bytes == 0 {
            return "RAM --".to_string();
        }

        format!(
            "RAM {}/{} GB",
            self.format_gigabytes(stats.ram_used_bytes),
            self.format_gigabytes(stats.ram_total_bytes)
        )
    }

    fn format_memory(&self, label: &str, used: Option<u64>, total: Option<u64>) -> String {
        match (used, total) {
            (Some(used), Some(total)) if total > 0 => format!(
                "{label} {}/{} GB",
                self.format_gigabytes(used),
                self.format_gigabytes(total)
            ),
            (Some(used), _) => format!("{label} {} GB", self.format_gigabytes(used)),
            _ => format!("{label} --"),
        }
    }

    fn format_gigabytes(&self, bytes: u64) -> String {
        let gigabytes = bytes as f64 / 1_073_741_824.0;

        format!("{gigabytes:.1}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::schemas::overlay_schemas::{OverlayColor, OverlayPosition, TextAnchor};

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

    fn item(metric: OverlayMetric) -> OverlayItem {
        OverlayItem {
            metric,
            position: OverlayPosition {
                x: 0,
                y: 0,
                anchor: TextAnchor::TopLeft,
            },
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

    #[test]
    fn formats_system_stats() {
        let formatter = OverlayFormatter::new();
        let stats = stats();

        assert_eq!(formatter.format(&item(OverlayMetric::Cpu), &stats), "CPU 12%");
        assert_eq!(
            formatter.format(&item(OverlayMetric::Ram), &stats),
            "RAM 8.0/16.0 GB"
        );
        assert_eq!(formatter.format(&item(OverlayMetric::Gpu), &stats), "GPU 34%");
        assert_eq!(
            formatter.format(&item(OverlayMetric::GpuMemory), &stats),
            "VRAM 1.2/8.0 GB"
        );
        assert_eq!(
            formatter.format(&item(OverlayMetric::CpuTemp), &stats),
            "CPU 61°C"
        );
        assert_eq!(
            formatter.format(&item(OverlayMetric::GpuTemp), &stats),
            "GPU --"
        );
    }

    #[test]
    fn formats_custom_text() {
        let formatter = OverlayFormatter::new();
        let mut item = item(OverlayMetric::Text);

        item.text = "Turky".to_string();

        assert_eq!(formatter.format(&item, &stats()), "Turky");
    }
}
