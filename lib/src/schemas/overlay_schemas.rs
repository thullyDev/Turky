use serde::{Deserialize, Serialize};

pub const DEFAULT_FONT_NAME: &str = "Roboto";

fn default_font() -> String {
    DEFAULT_FONT_NAME.to_string()
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct OverlayConfig {
    pub items: Vec<OverlayItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OverlayItem {
    pub metric: OverlayMetric,
    pub position: OverlayPosition,
    #[serde(default = "default_font")]
    pub font: String,
    pub size: f32,
    pub color: OverlayColor,
    #[serde(default)]
    pub text: String,
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
    Text,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OverlayPosition {
    pub x: i32,
    pub y: i32,
    pub anchor: TextAnchor,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TextAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
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
    fn deserializes_overlay_config_from_the_ui() {
        let json = r#"{
            "items": [
                {
                    "metric": "gpuMemory",
                    "position": { "x": 12, "y": 40, "anchor": "bottomRight" },
                    "font": "Roboto",
                    "size": 28.0,
                    "color": { "r": 255, "g": 255, "b": 255, "a": 255 },
                    "text": ""
                },
                {
                    "metric": "text",
                    "position": { "x": 0, "y": 0, "anchor": "topLeft" },
                    "font": "Roboto",
                    "size": 18.0,
                    "color": { "r": 0, "g": 255, "b": 0, "a": 255 },
                    "text": "Turky"
                }
            ]
        }"#;

        let config: OverlayConfig = serde_json::from_str(json).expect("config should parse");

        assert_eq!(config.items.len(), 2);
        assert_eq!(config.items[0].metric, OverlayMetric::GpuMemory);
        assert_eq!(config.items[0].position.anchor, TextAnchor::BottomRight);
        assert_eq!(config.items[0].position.x, 12);
        assert_eq!(config.items[1].metric, OverlayMetric::Text);
        assert_eq!(config.items[1].text, "Turky");
        assert_eq!(config.items[1].color.g, 255);
    }

    #[test]
    fn text_defaults_to_empty_when_omitted() {
        let json = r#"{
            "items": [
                {
                    "metric": "cpu",
                    "position": { "x": 1, "y": 2, "anchor": "center" },
                    "font": "Roboto",
                    "size": 16.0,
                    "color": { "r": 1, "g": 2, "b": 3, "a": 4 }
                }
            ]
        }"#;

        let config: OverlayConfig = serde_json::from_str(json).expect("config should parse");

        assert_eq!(config.items[0].text, "");
        assert_eq!(config.items[0].position.anchor, TextAnchor::Center);
    }

    #[test]
    fn font_defaults_to_roboto_when_omitted() {
        let json = r#"{
            "items": [
                {
                    "metric": "cpu",
                    "position": { "x": 0, "y": 0, "anchor": "topLeft" },
                    "size": 16.0,
                    "color": { "r": 255, "g": 255, "b": 255, "a": 255 }
                }
            ]
        }"#;

        let config: OverlayConfig = serde_json::from_str(json).expect("config should parse");

        assert_eq!(config.items[0].font, DEFAULT_FONT_NAME);
    }
}
