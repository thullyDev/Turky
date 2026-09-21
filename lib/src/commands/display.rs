use crate::state::AppState;
use image::RgbaImage;

#[tauri::command]
pub fn render_image(bytes: Vec<u8>, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let image = image::load_from_memory(&bytes)
        .map_err(|e| format!("Failed to decode image: {e}"))?
        .to_rgba8();

    let mut display = state
        .display
        .lock()
        .map_err(|e| format!("Failed to lock DisplayService: {e}"))?;

    display.render_image(image);

    Ok(())
}

#[cfg(test)]
mod tests {
    use image::{DynamicImage, Rgba, RgbaImage};
    use std::io::Cursor;

    #[test]
    fn decodes_valid_image_bytes() {
        let image = RgbaImage::from_pixel(480, 1920, Rgba([255, 0, 0, 255]));

        let mut bytes = Vec::new();

        DynamicImage::ImageRgba8(image)
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
            .expect("Failed to encode test image");

        let decoded = image::load_from_memory(&bytes)
            .expect("Failed to decode image")
            .to_rgba8();

        assert_eq!(decoded.width(), 480);
        assert_eq!(decoded.height(), 1920);
        assert_eq!(decoded.get_pixel(0, 0), &Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn rejects_invalid_image_bytes() {
        let bytes = vec![1, 2, 3, 4, 5];

        let result = image::load_from_memory(&bytes);

        assert!(result.is_err());
    }
}
