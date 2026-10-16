use std::f32::consts::{PI, TAU};

use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;

use crate::schemas::overlay_schemas::{
    OverlayColor, OverlayItem, OverlayMetric, OverlayTextBlock, RingChromeStyle, WidgetKind,
};
use super::system_stats_service::SystemStats;

const TOP_ANGLE: f32 = -PI / 2.0;

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
    let size = item.widget.ring_size.max(16) as f32;
    let thickness = item.widget.ring_thickness.max(2) as f32;
    let x = item.position.x as f32;
    let y = item.position.y as f32;
    let center_x = x + size / 2.0;
    let center_y = y + size / 2.0;
    let radius = (size - thickness) / 2.0;
    let half = thickness / 2.0;
    let label_block = item.resolved_label();
    let value_block = item.resolved_value();
    let use_gradient = item.widget.ring_style == RingChromeStyle::Gradient;
    let fill_start = pick_fill(item);
    let fill_end = item.widget.fill_gradient_color;
    let ratio = metric_ratio(item.metric, stats, fps)
        .unwrap_or(0.0)
        .clamp(0.0, 1.0);

    let glass = Rgba([255, 255, 255, 56]);
    paint_ring_stroke(
        image,
        center_x,
        center_y,
        radius,
        half,
        1.0,
        |_, _| glass,
        true,
    );

    let mut track = item.widget.track_color;
    track.a = ((track.a as f32) * 0.65).round() as u8;
    paint_ring_stroke(
        image,
        center_x,
        center_y,
        radius,
        half,
        1.0,
        |_, _| to_rgba(track),
        true,
    );

    if ratio > 0.0 {
        paint_ring_stroke(
            image,
            center_x,
            center_y,
            radius,
            half,
            ratio,
            |_px, py| {
                if use_gradient {
                    let top = center_y - radius - half;
                    let bottom = center_y + radius + half;
                    let span = (bottom - top).max(1.0);
                    let t = ((py - top) / span).clamp(0.0, 1.0);
                    to_rgba(lerp_color(fill_start, fill_end, t))
                } else {
                    to_rgba(fill_start)
                }
            },
            true,
        );
    }

    let label_h = if item.widget.show_label {
        label_block.size
    } else {
        0.0
    };
    let value_h = if item.widget.show_value {
        value_block.size
    } else {
        0.0
    };
    let gap = if item.widget.show_label && item.widget.show_value {
        2.0
    } else {
        0.0
    };
    let block_h = label_h + gap + value_h;
    let mut text_y = center_y - block_h / 2.0;

    if item.widget.show_label {
        let label = maybe_uppercase(&resolve_label(item), &label_block);
        draw_line_centered(
            image,
            &label_block,
            regular,
            bold,
            &label,
            center_x,
            text_y.round() as i32,
        );
        text_y += label_h + gap;
    }

    if item.widget.show_value {
        let value = maybe_uppercase(&format_value(item.metric, stats, fps), &value_block);
        draw_line_centered(
            image,
            &value_block,
            regular,
            bold,
            &value,
            center_x,
            text_y.round() as i32,
        );
    }
}

fn paint_ring_stroke<F>(
    image: &mut RgbaImage,
    center_x: f32,
    center_y: f32,
    radius: f32,
    half_thickness: f32,
    ratio: f32,
    color_at: F,
    round_cap: bool,
) where
    F: Fn(f32, f32) -> Rgba<u8>,
{
    let sweep = ratio.clamp(0.0, 1.0) * TAU;
    let full_circle = ratio >= 1.0 - f32::EPSILON;
    let pad = half_thickness.ceil() as i32 + 2;
    let min_x = (center_x - radius - pad as f32).floor() as i32;
    let max_x = (center_x + radius + pad as f32).ceil() as i32;
    let min_y = (center_y - radius - pad as f32).floor() as i32;
    let max_y = (center_y + radius + pad as f32).ceil() as i32;
    let (start_x, start_y) = point_on_ring(center_x, center_y, radius, TOP_ANGLE);
    let end_angle = TOP_ANGLE + sweep;
    let (end_x, end_y) = point_on_ring(center_x, center_y, radius, end_angle);
    let cap_r = half_thickness;

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            let fx = px as f32 + 0.5;
            let fy = py as f32 + 0.5;
            let dx = fx - center_x;
            let dy = fy - center_y;
            let dist = (dx * dx + dy * dy).sqrt();
            let ring_dist = (dist - radius).abs();
            let mut coverage = aa_coverage(half_thickness, ring_dist);

            let mut on_stroke = full_circle;

            if !on_stroke {
                on_stroke = angle_on_arc(dx, dy, TOP_ANGLE, sweep);

                if round_cap && sweep > 0.0 {
                    if cap_hit(fx, fy, start_x, start_y, cap_r) {
                        on_stroke = true;
                    }
                    if cap_hit(fx, fy, end_x, end_y, cap_r) {
                        on_stroke = true;
                    }
                }
            }

            if !on_stroke {
                continue;
            }

            if round_cap && sweep > 0.0 && !full_circle {
                let cap_boost = cap_coverage(
                    fx,
                    fy,
                    start_x,
                    start_y,
                    end_x,
                    end_y,
                    cap_r,
                    ring_dist,
                    half_thickness,
                );
                coverage = coverage.max(cap_boost);
            }

            if coverage <= 0.0 {
                continue;
            }

            let color = color_at(fx, fy);
            blend_pixel(image, px, py, color, coverage);
        }
    }
}

fn point_on_ring(center_x: f32, center_y: f32, radius: f32, angle: f32) -> (f32, f32) {
    (
        center_x + radius * angle.cos(),
        center_y + radius * angle.sin(),
    )
}

fn cap_hit(px: f32, py: f32, cap_x: f32, cap_y: f32, cap_r: f32) -> bool {
    let dx = px - cap_x;
    let dy = py - cap_y;
    dx * dx + dy * dy <= cap_r * cap_r
}

fn cap_coverage(
    px: f32,
    py: f32,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    cap_r: f32,
    ring_dist: f32,
    half_thickness: f32,
) -> f32 {
    let mut best = 0.0_f32;
    for (cx, cy) in [(start_x, start_y), (end_x, end_y)] {
        let dx = px - cx;
        let dy = py - cy;
        let d = (dx * dx + dy * dy).sqrt();
        best = best.max(aa_coverage(cap_r, d).min(aa_coverage(half_thickness, ring_dist)));
    }
    best
}

fn normalize_angle(angle: f32) -> f32 {
    let mut a = angle % TAU;
    if a < 0.0 {
        a += TAU;
    }
    a
}

fn angle_on_arc(dx: f32, dy: f32, start: f32, sweep: f32) -> bool {
    if sweep <= 0.0 {
        return false;
    }

    let angle = normalize_angle(dy.atan2(dx));
    let start_n = normalize_angle(start);
    let end = start_n + sweep;

    if end <= TAU {
        angle >= start_n && angle <= end
    } else {
        angle >= start_n || angle <= end - TAU
    }
}

fn aa_coverage(half_thickness: f32, distance: f32) -> f32 {
    let outer = half_thickness + 0.75;
    let inner = half_thickness - 0.75;

    if distance <= inner {
        return 1.0;
    }

    if distance >= outer {
        return 0.0;
    }

    1.0 - (distance - inner) / (outer - inner)
}

fn blend_pixel(image: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>, coverage: f32) {
    if coverage <= 0.0 {
        return;
    }

    let width = image.width() as i32;
    let height = image.height() as i32;

    if x < 0 || y < 0 || x >= width || y >= height {
        return;
    }

    let src_alpha = (color[3] as f32 / 255.0) * coverage;

    if src_alpha <= 0.0 {
        return;
    }

    let pixel = image.get_pixel_mut(x as u32, y as u32);
    let dst_alpha = pixel[3] as f32 / 255.0;
    let out_alpha = src_alpha + dst_alpha * (1.0 - src_alpha);

    if out_alpha <= 0.0 {
        *pixel = Rgba([0, 0, 0, 0]);
        return;
    }

    for channel in 0..3 {
        let src = color[channel] as f32 / 255.0;
        let dst = pixel[channel] as f32 / 255.0;
        let blended = (src * src_alpha + dst * dst_alpha * (1.0 - src_alpha)) / out_alpha;
        pixel[channel] = (blended * 255.0).round().clamp(0.0, 255.0) as u8;
    }

    pixel[3] = (out_alpha * 255.0).round().clamp(0.0, 255.0) as u8;
}

fn lerp_color(a: OverlayColor, b: OverlayColor, t: f32) -> OverlayColor {
    let t = t.clamp(0.0, 1.0);
    let lerp = |from: u8, to: u8| -> u8 {
        (from as f32 + (to as f32 - from as f32) * t).round() as u8
    };

    OverlayColor {
        r: lerp(a.r, b.r),
        g: lerp(a.g, b.g),
        b: lerp(a.b, b.b),
        a: lerp(a.a, b.a),
    }
}

fn draw_line_centered(
    image: &mut RgbaImage,
    block: &OverlayTextBlock,
    regular: &FontArc,
    bold: &FontArc,
    text: &str,
    center_x: f32,
    y: i32,
) {
    let font = pick_font(block, regular, bold);
    let width = text_width(font, block.size, text);
    let x = (center_x - width / 2.0).round() as i32;

    draw_line_at(image, block, regular, bold, text, x, y);
}

fn text_width(font: &FontArc, size: f32, text: &str) -> f32 {
    let scaled = font.as_scaled(PxScale::from(size));

    text.chars()
        .map(|ch| scaled.glyph_id(ch))
        .map(|id| scaled.h_advance(id))
        .sum()
}

fn pick_font<'a>(block: &OverlayTextBlock, regular: &'a FontArc, bold: &'a FontArc) -> &'a FontArc {
    if block.typography.font_weight >= 600 {
        bold
    } else {
        regular
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
    let font = pick_font(block, regular, bold);

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
        OverlayItem, OverlayPosition, OverlayTextStyle, OverlayWidgetConfig, WidgetKind,
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

    #[test]
    fn ring_widget_uses_smooth_stroke_pixels() {
        let item = OverlayItem {
            metric: OverlayMetric::Cpu,
            position: OverlayPosition { x: 40, y: 40 },
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
                kind: WidgetKind::Ring,
                ring_size: 72,
                ring_thickness: 8,
                ring_style: RingChromeStyle::Gradient,
                fill_color: OverlayColor {
                    r: 168,
                    g: 85,
                    b: 247,
                    a: 255,
                },
                fill_gradient_color: OverlayColor {
                    r: 34,
                    g: 211,
                    b: 238,
                    a: 255,
                },
                show_label: false,
                show_value: false,
                ..OverlayWidgetConfig::default()
            },
        };

        let mut image = RgbaImage::new(200, 200);
        let stats = SystemStats {
            cpu_percent: 50.0,
            ..SystemStats::empty()
        };
        let painter_regular = FontArc::try_from_slice(include_bytes!(
            "../../assets/fonts/Roboto-Regular.ttf"
        ))
        .expect("font");
        let painter_bold = FontArc::try_from_slice(include_bytes!(
            "../../assets/fonts/Roboto-Bold.ttf"
        ))
        .expect("font");

        draw_widget(
            &mut image,
            &item,
            &stats,
            None,
            &painter_regular,
            &painter_bold,
        );

        let mut purple = 0;
        let mut cyan = 0;

        for pixel in image.pixels() {
            if pixel[3] == 0 {
                continue;
            }

            if pixel[2] > 200 && pixel[0] < 120 {
                cyan += 1;
            }

            if pixel[0] > 150 && pixel[2] > 150 {
                purple += 1;
            }
        }

        assert!(purple > 0, "expected gradient ring pixels");
        assert!(cyan > 0, "expected cyan portion of gradient ring");
    }
}
