use crate::adapters::display_adapter::{AdapterError, DisplayAdapter};
use crate::devices::device_info::DeviceInfo;
use base64::Engine;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::{ImageEncoder, RgbaImage};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const WINDOW_LABEL: &str = "virtual-display";

#[derive(Clone, serde::Serialize)]
struct FrameEvent {
    width: u32,
    height: u32,
    jpeg: String,
}

#[derive(Clone, serde::Serialize)]
struct GifEvent {
    gif: String,
}

#[derive(Clone, serde::Serialize)]
struct OverlayEvent {
    width: u32,
    height: u32,
    png: String,
}

pub struct VirtualAdapter {
    info: DeviceInfo,
    connected: bool,
    app: AppHandle,
    prepared: Vec<String>,
    next_prepared: usize,
    gif_background: bool,
}

impl VirtualAdapter {
    pub fn new(app: AppHandle, info: DeviceInfo) -> Self {
        Self {
            info,
            connected: false,
            app,
            prepared: Vec::new(),
            next_prepared: 0,
            gif_background: false,
        }
    }

    fn create_window(&self) -> Result<(), AdapterError> {
        if self.app.get_webview_window(WINDOW_LABEL).is_some() {
            return Ok(());
        }

        WebviewWindowBuilder::new(
            &self.app,
            WINDOW_LABEL,
            WebviewUrl::App("index.html?virtual-display".into()),
        )
        .title("Turky Virtual Display")
        .inner_size(self.info.width as f64, self.info.height as f64)
        .resizable(true)
        .build()
        .map_err(|_| AdapterError::ConnectionFailed)?;

        Ok(())
    }

    fn encode_jpeg(image: &RgbaImage) -> Result<Vec<u8>, AdapterError> {
        let mut rgb = Vec::with_capacity((image.width() * image.height() * 3) as usize);
        for pixel in image.pixels() {
            rgb.extend_from_slice(&pixel.0[..3]);
        }
        let mut jpeg = Vec::new();
        let encoder = JpegEncoder::new_with_quality(&mut jpeg, 75);
        encoder
            .write_image(
                &rgb,
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgb8,
            )
            .map_err(|_| AdapterError::SendFailed)?;
        Ok(jpeg)
    }

    fn encode_jpeg_base64(image: &RgbaImage) -> Result<String, AdapterError> {
        let jpeg = Self::encode_jpeg(image)?;

        Ok(base64::engine::general_purpose::STANDARD.encode(jpeg))
    }

    fn encode_png_base64(image: &RgbaImage) -> Result<String, AdapterError> {
        let mut png = Vec::new();
        let encoder = PngEncoder::new(&mut png);

        encoder
            .write_image(
                image.as_raw(),
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgba8,
            )
            .map_err(|_| AdapterError::SendFailed)?;

        Ok(base64::engine::general_purpose::STANDARD.encode(png))
    }
}

fn validate_frame(info: &DeviceInfo, frame: &[u8]) -> Result<(), AdapterError> {
    let expected_size = (info.width * info.height * 4) as usize;

    if frame.len() != expected_size {
        return Err(AdapterError::SendFailed);
    }

    Ok(())
}

impl DisplayAdapter for VirtualAdapter {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn connect(&mut self) -> Result<(), AdapterError> {
        if self.connected {
            return Ok(());
        }

        self.create_window()?;

        self.connected = true;

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), AdapterError> {
        if !self.connected {
            return Ok(());
        }

        if let Some(window) = self.app.get_webview_window(WINDOW_LABEL) {
            window.close().map_err(|_| AdapterError::ConnectionFailed)?;
        }

        self.connected = false;

        Ok(())
    }

    fn send_frame(&mut self, image: &RgbaImage) -> Result<(), AdapterError> {
        if !self.connected {
            return Err(AdapterError::Disconnected);
        }

        if image.width() != self.info.width || image.height() != self.info.height {
            return Err(AdapterError::SendFailed);
        }

        if self.gif_background {
            let event = OverlayEvent {
                width: image.width(),
                height: image.height(),
                png: Self::encode_png_base64(image)?,
            };

            self.app
                .emit_to(WINDOW_LABEL, "virtual-overlay", event)
                .map_err(|error| {
                    eprintln!("Failed to emit virtual-overlay: {error}");
                    AdapterError::SendFailed
                })?;

            return Ok(());
        }

        let jpeg = if self.prepared.is_empty() {
            Self::encode_jpeg_base64(image)?
        } else {
            let jpeg = self.prepared[self.next_prepared].clone();
            self.next_prepared = (self.next_prepared + 1) % self.prepared.len();
            jpeg
        };

        let event = FrameEvent {
            width: image.width(),
            height: image.height(),
            jpeg,
        };

        self.app
            .emit_to(WINDOW_LABEL, "virtual-frame", event)
            .map_err(|error| {
                eprintln!("Failed to emit virtual-frame: {error}");
                AdapterError::SendFailed
            })?;

        Ok(())
    }

    fn present_gif(&mut self, bytes: &[u8]) -> Result<bool, AdapterError> {
        if !self.connected {
            return Err(AdapterError::Disconnected);
        }

        self.prepared.clear();
        self.next_prepared = 0;
        self.gif_background = true;

        let event = GifEvent {
            gif: base64::engine::general_purpose::STANDARD.encode(bytes),
        };

        self.app
            .emit_to(WINDOW_LABEL, "virtual-gif", event)
            .map_err(|error| {
                eprintln!("Failed to emit virtual-gif: {error}");
                AdapterError::SendFailed
            })?;

        Ok(true)
    }

    fn dismiss_gif(&mut self) {
        if !self.gif_background {
            return;
        }

        self.gif_background = false;

        let event = GifEvent { gif: String::new() };

        if let Err(error) = self.app.emit_to(WINDOW_LABEL, "virtual-gif", event) {
            eprintln!("Failed to emit virtual-gif: {error}");
        }
    }

    fn preload_frame(&mut self, image: &RgbaImage) -> Result<(), AdapterError> {
        if image.width() != self.info.width || image.height() != self.info.height {
            return Err(AdapterError::SendFailed);
        }

        let jpeg = Self::encode_jpeg_base64(image)?;
        self.prepared.push(jpeg);

        Ok(())
    }

    fn reset_preloaded(&mut self) {
        self.prepared.clear();
        self.next_prepared = 0;
    }

    fn clear(&mut self) -> Result<(), AdapterError> {
        if !self.connected {
            return Err(AdapterError::Disconnected);
        }

        self.app
            .emit_to(WINDOW_LABEL, "virtual-clear", ())
            .map_err(|_| AdapterError::ClearFailed)?;

        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::device_id::DeviceId;

    fn test_device_info() -> DeviceInfo {
        DeviceInfo {
            id: DeviceId("virtual-001".to_string()),
            name: "Virtual Display".to_string(),
            vendor_id: 0xFFFF,
            product_id: 0x0001,
            width: 2,
            height: 2,
        }
    }

    #[test]
    fn frame_size_for_2x2_display_is_16_bytes() {
        let info = test_device_info();

        let expected_size = (info.width * info.height * 4) as usize;

        assert_eq!(expected_size, 16);
    }

    #[test]
    fn accepts_valid_frame() {
        let info = test_device_info();

        let frame = vec![0u8; 16];

        let result = validate_frame(&info, &frame);

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_frame_that_is_too_small() {
        let info = test_device_info();

        let frame = vec![0u8; 15];

        let result = validate_frame(&info, &frame);

        assert!(matches!(result, Err(AdapterError::SendFailed)));
    }

    #[test]
    fn rejects_frame_that_is_too_large() {
        let info = test_device_info();

        let frame = vec![0u8; 17];

        let result = validate_frame(&info, &frame);

        assert!(matches!(result, Err(AdapterError::SendFailed)));
    }

    #[test]
    fn rejects_empty_frame() {
        let info = test_device_info();

        let frame = vec![];

        let result = validate_frame(&info, &frame);

        assert!(matches!(result, Err(AdapterError::SendFailed)));
    }

    #[test]
    fn accepts_turzx_8_8_frame() {
        let info = DeviceInfo {
            id: DeviceId("virtual-turzx-8.8".to_string()),
            name: "Virtual TURZX 8.8".to_string(),
            vendor_id: 0x1CBE,
            product_id: 0x0088,
            width: 480,
            height: 1920,
        };

        let frame = vec![0u8; 480 * 1920 * 4];

        let result = validate_frame(&info, &frame);

        assert!(result.is_ok());
    }

    #[test]
    fn frame_event_contains_correct_dimensions_and_pixels() {
        let info = test_device_info();

        let jpeg = "encoded-frame".to_string();

        let event = FrameEvent {
            width: info.width,
            height: info.height,
            jpeg: jpeg.clone(),
        };

        assert_eq!(event.width, 2);
        assert_eq!(event.height, 2);
        assert_eq!(event.jpeg, jpeg);
    }
}
