use ab_glyph::{Font, FontArc, PxScale, ScaleFont};

use crate::schemas::overlay_schemas::TextAnchor;

pub struct TextLayout;

impl TextLayout {
    pub fn new() -> Self {
        Self
    }

    pub fn size(&self, font: &FontArc, size: f32, text: &str) -> (i32, i32) {
        let scaled = font.as_scaled(PxScale::from(size));

        let width: f32 = text
            .chars()
            .map(|character| scaled.h_advance(scaled.glyph_id(character)))
            .sum();

        (width.round() as i32, scaled.height().round() as i32)
    }

    pub fn origin(&self, anchor: TextAnchor, x: i32, y: i32, width: i32, height: i32) -> (i32, i32) {
        match anchor {
            TextAnchor::TopLeft => (x, y),
            TextAnchor::TopRight => (x - width, y),
            TextAnchor::BottomLeft => (x, y - height),
            TextAnchor::BottomRight => (x - width, y - height),
            TextAnchor::Center => (x - width / 2, y - height / 2),
        }
    }
}
