use image::imageops::FilterType;
use image::{Rgba, RgbaImage};

pub fn fit_cover(image: &RgbaImage, target_width: u32, target_height: u32) -> RgbaImage {
    let target_width = target_width.max(1);
    let target_height = target_height.max(1);

    if image.width() == target_width && image.height() == target_height {
        return image.clone();
    }

    let src_w = image.width().max(1) as f32;
    let src_h = image.height().max(1) as f32;
    let target_w = target_width as f32;
    let target_h = target_height as f32;

    let scale = (target_w / src_w).max(target_h / src_h);
    let scaled_w = (src_w * scale).round().max(1.0) as u32;
    let scaled_h = (src_h * scale).round().max(1.0) as u32;

    let scaled = image::imageops::resize(image, scaled_w, scaled_h, FilterType::Triangle);

    let x0 = ((scaled_w as i64 - target_width as i64) / 2).max(0) as u32;
    let y0 = ((scaled_h as i64 - target_height as i64) / 2).max(0) as u32;

    image::imageops::crop_imm(&scaled, x0, y0, target_width, target_height).to_image()
}

pub fn blank_frame(width: u32, height: u32) -> RgbaImage {
    RgbaImage::from_pixel(width.max(1), height.max(1), Rgba([0, 0, 0, 255]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    #[test]
    fn fit_cover_fills_target_dimensions() {
        let source = RgbaImage::from_pixel(100, 50, Rgba([255, 0, 0, 255]));
        let fitted = fit_cover(&source, 200, 200);

        assert_eq!(fitted.dimensions(), (200, 200));
    }

    #[test]
    fn fit_cover_keeps_exact_match_without_resampling() {
        let source = RgbaImage::from_pixel(320, 240, Rgba([0, 255, 0, 255]));
        let fitted = fit_cover(&source, 320, 240);

        assert_eq!(fitted.as_raw(), source.as_raw());
    }
}
