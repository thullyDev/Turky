use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

const LABEL: &str = "LOADING....";

pub fn render(width: u32, height: u32) -> RgbaImage {
    let mut image = RgbaImage::new(width.max(1), height.max(1));

    for pixel in image.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 255]);
    }

    let font = FontArc::try_from_slice(include_bytes!(
        "../../assets/fonts/Roboto-Bold.ttf"
    ))
    .expect("bundled Roboto Bold font should load");

    let size = (width.min(height) as f32 * 0.08).clamp(10.0, 56.0);
    let scale = PxScale::from(size);
    let scaled = font.as_scaled(scale);
    let text_width: f32 = LABEL
        .chars()
        .map(|ch| scaled.h_advance(scaled.glyph_id(ch)))
        .sum();
    let x = ((width as f32 - text_width) / 2.0).round().max(0.0) as i32;
    let y = ((height as f32 - size) / 2.0).round().max(0.0) as i32;

    draw_text_mut(
        &mut image,
        Rgba([255, 255, 255, 255]),
        x,
        y,
        scale,
        &font,
        LABEL,
    );

    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_frame_is_black_with_bright_pixels_for_text() {
        let image = render(320, 240);

        assert_eq!(image.get_pixel(0, 0), &Rgba([0, 0, 0, 255]));

        let has_text = image
            .pixels()
            .any(|pixel| pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200);

        assert!(has_text, "expected white loading label pixels");
    }
}
