use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct OverlayConfig {
    pub items: Vec<OverlayItem>,
}

impl OverlayConfig {
    pub fn stacked() -> Self {
        let metrics = [
            OverlayMetric::Cpu,
            OverlayMetric::Ram,
            OverlayMetric::Gpu,
            OverlayMetric::GpuMemory,
            OverlayMetric::CpuTemp,
            OverlayMetric::GpuTemp,
        ];

        Self {
            items: metrics
                .into_iter()
                .enumerate()
                .map(|(index, metric)| OverlayItem {
                    metric,
                    position: OverlayPosition {
                        x: 8,
                        y: 8 + index as i32 * 32,
                    },
                    size: 24.0,
                    color: OverlayColor {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OverlayItem {
    pub metric: OverlayMetric,
    pub position: OverlayPosition,
    pub size: f32,
    pub color: OverlayColor,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OverlayMetric {
    Cpu,
    Ram,
    Gpu,
    GpuMemory,
    CpuTemp,
    GpuTemp,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct OverlayPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct OverlayColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_the_stats_gui() {
        let json = r#"{
            "items": [
                {
                    "metric": "gpuMemory",
                    "position": { "x": 12, "y": 40 },
                    "size": 28.0,
                    "color": { "r": 255, "g": 0, "b": 0, "a": 255 }
                }
            ]
        }"#;

        let config: OverlayConfig = serde_json::from_str(json).expect("config should parse");

        assert_eq!(config.items.len(), 1);
        assert_eq!(config.items[0].metric, OverlayMetric::GpuMemory);
        assert_eq!(config.items[0].position.x, 12);
        assert_eq!(config.items[0].color.r, 255);
    }
}
