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
            OverlayMetric::Fps,
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
                    label: OverlayTextBlock {
                        size: 11.0,
                        color: default_white(),
                        typography: OverlayTextStyle {
                            font_weight: 700,
                            uppercase: true,
                            letter_spacing: 0.08,
                            ..OverlayTextStyle::default()
                        },
                        css: String::new(),
                    },
                    value: OverlayTextBlock {
                        size: 24.0,
                        color: default_white(),
                        typography: OverlayTextStyle {
                            font_weight: 700,
                            ..OverlayTextStyle::default()
                        },
                        css: String::new(),
                    },
                    size: 0.0,
                    color: default_white(),
                    style: OverlayTextStyle::default(),
                    widget: OverlayWidgetConfig::default(),
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
    #[serde(default)]
    pub label: OverlayTextBlock,
    #[serde(default)]
    pub value: OverlayTextBlock,
    #[serde(default)]
    pub size: f32,
    #[serde(default = "default_white")]
    pub color: OverlayColor,
    #[serde(default)]
    pub style: OverlayTextStyle,
    #[serde(default)]
    pub widget: OverlayWidgetConfig,
}

impl OverlayItem {
    pub fn resolved_label(&self) -> OverlayTextBlock {
        if self.label.size > 0.0 {
            return self.label.clone();
        }

        OverlayTextBlock {
            size: (self.size * 0.46).max(10.0),
            color: self.color,
            typography: self.style.clone(),
            css: String::new(),
        }
    }

    pub fn resolved_value(&self) -> OverlayTextBlock {
        if self.value.size > 0.0 {
            return self.value.clone();
        }

        OverlayTextBlock {
            size: self.size,
            color: self.color,
            typography: self.style.clone(),
            css: String::new(),
        }
    }

    pub fn value_size(&self) -> f32 {
        let size = self.resolved_value().size;

        if size.is_finite() && size > 0.0 {
            size
        } else {
            self.size
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct OverlayTextBlock {
    pub size: f32,
    pub color: OverlayColor,
    pub typography: OverlayTextStyle,
    pub css: String,
}

impl Default for OverlayTextBlock {
    fn default() -> Self {
        Self {
            size: 0.0,
            color: default_white(),
            typography: OverlayTextStyle::default(),
            css: String::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct OverlayWidgetConfig {
    pub kind: WidgetKind,
    pub label: String,
    pub show_label: bool,
    pub show_value: bool,
    pub bar_width: u32,
    pub bar_height: u32,
    pub bar_border_radius: u32,
    pub bar_track_css: String,
    pub bar_fill_css: String,
    pub ring_size: u32,
    pub ring_thickness: u32,
    pub ring_track_css: String,
    pub ring_fill_css: String,
    pub track_color: OverlayColor,
    pub fill_color: OverlayColor,
}

impl Default for OverlayWidgetConfig {
    fn default() -> Self {
        Self {
            kind: WidgetKind::Text,
            label: String::new(),
            show_label: true,
            show_value: true,
            bar_width: 128,
            bar_height: 10,
            bar_border_radius: 999,
            bar_track_css: String::new(),
            bar_fill_css: String::new(),
            ring_size: 56,
            ring_thickness: 7,
            ring_track_css: String::new(),
            ring_fill_css: String::new(),
            track_color: OverlayColor {
                r: 255,
                g: 255,
                b: 255,
                a: 40,
            },
            fill_color: OverlayColor {
                r: 139,
                g: 92,
                b: 246,
                a: 255,
            },
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WidgetKind {
    #[default]
    Text,
    Bar,
    Ring,
    Stack,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct OverlayTextStyle {
    pub font_preset: OverlayFontPreset,
    pub font_weight: u16,
    pub italic: bool,
    pub letter_spacing: f32,
    pub uppercase: bool,
    #[serde(default)]
    pub custom_font_id: Option<String>,
}

impl Default for OverlayTextStyle {
    fn default() -> Self {
        Self {
            font_preset: OverlayFontPreset::Roboto,
            font_weight: 400,
            italic: false,
            letter_spacing: 0.0,
            uppercase: false,
            custom_font_id: None,
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OverlayFontPreset {
    #[default]
    Roboto,
    Inter,
    Mono,
    Serif,
    Compact,
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
    Fps,
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

fn default_white() -> OverlayColor {
    OverlayColor {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    }
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
        assert_eq!(config.items[0].resolved_value().size, 28.0);
        assert_eq!(
            config.items[0].style.font_preset,
            OverlayFontPreset::Roboto
        );
    }

    #[test]
    fn deserializes_items_without_style_using_defaults() {
        let json = r#"{
            "items": [
                {
                    "metric": "cpu",
                    "position": { "x": 0, "y": 0 },
                    "size": 20.0,
                    "color": { "r": 255, "g": 255, "b": 255, "a": 255 }
                }
            ]
        }"#;

        let config: OverlayConfig = serde_json::from_str(json).expect("config should parse");

        assert_eq!(config.items[0].style.font_weight, 400);
        assert!(!config.items[0].style.uppercase);
        assert_eq!(config.items[0].widget.kind, WidgetKind::Text);
    }
}
