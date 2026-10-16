use std::f32::consts::PI;

use ab_glyph::{FontArc, PxScale};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;

use crate::schemas::overlay_schemas::{
    OverlayColor, OverlayItem, OverlayMetric, OverlayTextBlock, WidgetKind,
};
use super::system_stats_service::SystemStats;

pub fn draw_widget(
    image: &mut RgbaImage,
    item: &OverlayItem,
    stats: &SystemStats,
    fps: Option<f32>,
    regular: &FontArc,
    bold: &FontArc,
) {
    match item.widget.kind {
        WidgetKind::Text => draw_text_widget(image, item, stats, fps, regular, bold),
        WidgetKind::Bar | WidgetKind::Stack => {
            draw_bar_widget(image, item, stats, fps, regular, bold)
        }
        WidgetKind::Ring => draw_ring_widget(image, item, stats, fps, regular, bold),
    }
}

fn draw_text_widget(
    image: &mut RgbaImage,
    item: &OverlayItem,
    stats: &SystemStats,
    fps: Option<f32>,
    regular: &FontArc,
    bold: &FontArc,
) {
    let mut y = item.position.y;
    let x = item.position.x;
    let label_block = item.resolved_label();
    let value_block = item.resolved_value();

    if item.widget.show_label {
        let label = maybe_uppercase(&resolve_label(item), &label_block);
        draw_line_at(image, &label_block, regular, bold, &label, x, y);
        y += label_block.size.round() as i32 + 2;
    }

    if item.widget.show_value {
        let value = maybe_uppercase(&format_value(item.metric, stats, fps), &value_block);
        draw_line_at(image, &value_block, regular, bold, &value, x, y);
    }
}

fn draw_bar_widget(
    image: &mut RgbaImage,
    item: &OverlayItem,
    stats: &SystemStats,
    fps: Option<f32>,
    regular: &FontArc,
    bold: &FontArc,
) {
    let mut y = item.position.y;
    let x = item.position.x;
    let label_block = item.resolved_label();
    let value_block = item.resolved_value();

    if item.widget.show_label {
        let label = maybe_uppercase(&resolve_label(item), &label_block);
        draw_line_at(image, &label_block, regular, bold, &label, x, y);
        y += label_block.size.round() as i32 + 4;
    }

    let bar_width = item.widget.bar_width.max(8) as i32;
    let bar_height = item.widget.bar_height.max(2) as i32;
    let track = item.widget.track_color;
    let fill = pick_fill(item);

    draw_filled_rect_mut(
        image,
        Rect::at(x, y).of_size(bar_width as u32, bar_height as u32),
        to_rgba(track),
    );

    if let Some(ratio) = metric_ratio(item.metric, stats, fps) {
        let fill_width = ((bar_width as f32) * ratio.clamp(0.0, 1.0)).round() as i32;

        if fill_width > 0 {
            draw_filled_rect_mut(
                image,
                Rect::at(x, y).of_size(fill_width as u32, bar_height as u32),
                to_rgba(fill),
            );
        }
    }

    y += bar_height + 2;

    if item.widget.show_value {
        let value = maybe_uppercase(&format_value(item.metric, stats, fps), &value_block);
        draw_line_at(image, &value_block, regular, bold, &value, x, y);
    }
}

fn draw_ring_widget(
    image: &mut RgbaImage,
    item: &OverlayItem,
    stats: &SystemStats,
    fps: Option<f32>,
    regular: &FontArc,
    bold: &FontArc,
) {
    let size = item.widget.ring_size.max(16) as i32;
    let thickness = item.widget.ring_thickness.max(2) as i32;
    let x = item.position.x;
    let y = item.position.y;
    let center_x = x + size / 2;
    let center_y = y + size / 2;
    let radius = (size - thickness) / 2;
    let track = to_rgba(item.widget.track_color);
    let fill = to_rgba(pick_fill(item));
    let label_block = item.resolved_label();
    let value_block = item.resolved_value();

    draw_ring_track(image, center_x, center_y, radius, thickness, track);

    if let Some(ratio) = metric_ratio(item.metric, stats, fps) {
        draw_ring_progress(
            image,
            center_x,
            center_y,
            radius,
            thickness,
            fill,
            ratio.clamp(0.0, 1.0),
        );
    }

    let mut text_y = center_y
        - ((label_block.size + value_block.size) / 2.0).round() as i32
        + 4;

    if item.widget.show_label {
        let label = maybe_uppercase(&resolve_label(item), &label_block);
        draw_line_at(
            image,
            &label_block,
            regular,
            bold,
            &label,
            center_x - radius,
            text_y,
        );
        text_y += label_block.size.round() as i32 + 2;
    }

    if item.widget.show_value {
        let value = maybe_uppercase(&format_value(item.metric, stats, fps), &value_block);
        draw_line_at(
            image,
            &value_block,
            regular,
            bold,
            &value,
            center_x - radius,
            text_y,
        );
    }
}

fn draw_ring_track(
    image: &mut RgbaImage,
    center_x: i32,
    center_y: i32,
    radius: i32,
    thickness: i32,
    color: Rgba<u8>,
) {
    stamp_ring(image, center_x, center_y, radius, thickness, color, 1.0);
}

fn draw_ring_progress(
    image: &mut RgbaImage,
    center_x: i32,
    center_y: i32,
    radius: i32,
    thickness: i32,
    color: Rgba<u8>,
    ratio: f32,
) {
    stamp_ring(image, center_x, center_y, radius, thickness, color, ratio);
}

fn stamp_ring(
    image: &mut RgbaImage,
    center_x: i32,
    center_y: i32,
    radius: i32,
    thickness: i32,
    color: Rgba<u8>,
    sweep: f32,
) {
    let steps = ((radius as f32 * sweep * 4.0).max(8.0)) as i32;

    for step in 0..steps {
        let t = step as f32 / steps as f32;
        let angle = -PI / 2.0 + t * sweep * 2.0 * PI;
        let px = center_x + (radius as f32 * angle.cos()).round() as i32;
        let py = center_y + (radius as f32 * angle.sin()).round() as i32;

        draw_filled_rect_mut(
            image,
            Rect::at(px - thickness / 2, py - thickness / 2).of_size(thickness as u32, thickness as u32),
            color,
        );
    }
}

fn draw_line_at(
    image: &mut RgbaImage,
    block: &OverlayTextBlock,
    regular: &FontArc,
    bold: &FontArc,
    text: &str,
    x: i32,
    y: i32,
) {
    let font = if block.typography.font_weight >= 600 {
        bold
    } else {
        regular
    };

    draw_text_mut(
        image,
        to_rgba(block.color),
        x,
        y,
        PxScale::from(block.size),
        font,
        text,
    );
}

fn resolve_label(item: &OverlayItem) -> String {
    let trimmed = item.widget.label.trim();

    if !trimmed.is_empty() {
        return trimmed.to_string();
    }

    default_label(item.metric).to_string()
}

fn default_label(metric: OverlayMetric) -> &'static str {
    match metric {
        OverlayMetric::Cpu => "CPU",
        OverlayMetric::Ram => "RAM",
        OverlayMetric::Gpu => "GPU",
        OverlayMetric::GpuMemory => "VRAM",
        OverlayMetric::CpuTemp => "CPU TEMP",
        OverlayMetric::GpuTemp => "GPU TEMP",
        OverlayMetric::Fps => "FPS",
    }
}

fn maybe_uppercase(text: &str, block: &OverlayTextBlock) -> String {
    if block.typography.uppercase {
        text.to_uppercase()
    } else {
        text.to_string()
    }
}

fn pick_fill(item: &OverlayItem) -> OverlayColor {
    if item.widget.fill_color.a > 0 {
        item.widget.fill_color
    } else {
        item.resolved_value().color
    }
}

fn to_rgba(color: OverlayColor) -> Rgba<u8> {
    Rgba([color.r, color.g, color.b, color.a])
}

fn metric_ratio(metric: OverlayMetric, stats: &SystemStats, fps: Option<f32>) -> Option<f32> {
    match metric {
        OverlayMetric::Cpu => Some(stats.cpu_percent / 100.0),
        OverlayMetric::Gpu => stats.gpu_percent.map(|value| value / 100.0),
        OverlayMetric::Ram if stats.ram_total_bytes > 0 => {
            Some(stats.ram_used_bytes as f32 / stats.ram_total_bytes as f32)
        }
        OverlayMetric::GpuMemory => match (stats.gpu_memory_used_bytes, stats.gpu_memory_total_bytes) {
            (Some(used), Some(total)) if total > 0 => Some(used as f32 / total as f32),
            _ => None,
        },
        OverlayMetric::CpuTemp => stats.cpu_temp_celsius.map(|value| value / 100.0),
        OverlayMetric::GpuTemp => stats.gpu_temp_celsius.map(|value| value / 100.0),
        OverlayMetric::Fps => fps.map(|value| (value / 144.0).clamp(0.0, 1.0)),
        _ => None,
    }
}

fn format_value(metric: OverlayMetric, stats: &SystemStats, fps: Option<f32>) -> String {
    match metric {
        OverlayMetric::Cpu => format!("{:.0}%", stats.cpu_percent),
        OverlayMetric::Ram => format!(
            "{}/{} GB",
            gigabytes(stats.ram_used_bytes),
            gigabytes(stats.ram_total_bytes)
        ),
        OverlayMetric::Gpu => match stats.gpu_percent {
            Some(value) => format!("{value:.0}%"),
            None => "--".to_string(),
        },
        OverlayMetric::GpuMemory => match (stats.gpu_memory_used_bytes, stats.gpu_memory_total_bytes) {
            (Some(used), Some(total)) => format!("{}/{} GB", gigabytes(used), gigabytes(total)),
            _ => "--".to_string(),
        },
        OverlayMetric::CpuTemp => match stats.cpu_temp_celsius {
            Some(value) => format!("{value:.0}°C"),
            None => "--".to_string(),
        },
        OverlayMetric::GpuTemp => match stats.gpu_temp_celsius {
            Some(value) => format!("{value:.0}°C"),
            None => "--".to_string(),
        },
        OverlayMetric::Fps => match fps {
            Some(value) => format!("{value:.0}"),
            None => "--".to_string(),
        },
    }
}

fn gigabytes(bytes: u64) -> String {
    format!("{:.1}", bytes as f64 / 1_073_741_824.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::overlay_schemas::{
        OverlayItem, OverlayPosition, OverlayTextStyle, OverlayWidgetConfig,
    };

    #[test]
    fn resolves_custom_widget_label() {
        let item = OverlayItem {
            metric: OverlayMetric::Cpu,
            position: OverlayPosition { x: 0, y: 0 },
            label: OverlayTextBlock::default(),
            value: OverlayTextBlock::default(),
            size: 20.0,
            color: OverlayColor {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
            style: OverlayTextStyle::default(),
            widget: OverlayWidgetConfig {
                label: "Processor".to_string(),
                ..OverlayWidgetConfig::default()
            },
        };

        assert_eq!(resolve_label(&item), "Processor");
    }
}
