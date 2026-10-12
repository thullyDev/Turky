use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use crate::adapters::display_adapter::{AdapterError, DisplayAdapter};
use crate::devices::device_info::DeviceInfo;
use base64::Engine;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::{ImageEncoder, RgbaImage};
use tauri::window::Color;
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const WINDOW_LABEL: &str = "virtual-display";

#[derive(Clone, serde::Serialize)]
struct FrameEvent {
    width: u32,
    height: u32,
    jpeg: String,
}

#[derive(Clone, serde::Serialize)]
struct ClipBegin {
    id: u64,
    width: u32,
    height: u32,
    frame_count: u32,
    delays_ms: Vec<u32>,
}

#[derive(Clone, serde::Serialize)]
struct ClipFrame {
    id: u64,
    index: u32,
    jpeg: String,
}

#[derive(Clone, serde::Serialize)]
struct OverlayEvent {
    id: u64,
    png: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

pub struct VirtualAdapter {
    info: DeviceInfo,
    connected: bool,
    app: AppHandle,
    prepared: Vec<String>,
    next_prepared: usize,
    clip_id: u64,
}

impl VirtualAdapter {
    pub fn new(app: AppHandle, info: DeviceInfo) -> Self {
        Self {
            info,
            connected: false,
            app,
            prepared: Vec::new(),
            next_prepared: 0,
            clip_id: 0,
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
        .background_color(Color(0, 0, 0, 255))
        .background_throttling(tauri::utils::config::BackgroundThrottlingPolicy::Disabled)
        .build()
        .map_err(|_| AdapterError::ConnectionFailed)?;

        Ok(())
    }

    fn encode_jpeg(image: &RgbaImage, quality: u8) -> Result<Vec<u8>, AdapterError> {
        let mut rgb = Vec::with_capacity((image.width() * image.height() * 3) as usize);
        for pixel in image.pixels() {
            rgb.extend_from_slice(&pixel.0[..3]);
        }
        let mut jpeg = Vec::new();
        let encoder = JpegEncoder::new_with_quality(&mut jpeg, quality);
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

    fn encode_jpeg_base64(image: &RgbaImage, quality: u8) -> Result<String, AdapterError> {
        let jpeg = Self::encode_jpeg(image, quality)?;

        Ok(base64::engine::general_purpose::STANDARD.encode(jpeg))
    }

    fn encode_clip(frames: &[&RgbaImage]) -> Result<Vec<String>, AdapterError> {
        if frames.is_empty() {
            return Ok(Vec::new());
        }

        let (sender, receiver) = std::sync::mpsc::channel();
        let next = AtomicUsize::new(0);
        let workers = std::thread::available_parallelism()
            .map(|count| count.get())
            .unwrap_or(1)
            .clamp(1, frames.len());

        std::thread::scope(|scope| {
            for _ in 0..workers {
                let sender = sender.clone();
                let next = &next;

                scope.spawn(move || {
                    loop {
                        let index = next.fetch_add(1, Ordering::Relaxed);

                        if index >= frames.len() {
                            break;
                        }

                        let encoded = Self::encode_jpeg_base64(frames[index], 85);

                        if sender.send((index, encoded)).is_err() {
                            break;
                        }
                    }
                });
            }
        });

        drop(sender);

        let mut encoded = vec![String::new(); frames.len()];
        let mut failed = false;

        for (index, jpeg) in receiver {
            match jpeg {
                Ok(jpeg) => encoded[index] = jpeg,
                Err(_) => failed = true,
            }
        }

        if failed || encoded.iter().any(String::is_empty) {
            return Err(AdapterError::SendFailed);
        }

        Ok(encoded)
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

fn delay_ms(delay: Duration) -> u32 {
    u32::try_from(delay.as_millis()).unwrap_or(u32::MAX).max(1)
}

fn opaque_bounds(image: &RgbaImage) -> Option<(u32, u32, u32, u32)> {
    let mut min_x = image.width();
    let mut min_y = image.height();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut any = false;

    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel[3] == 0 {
            continue;
        }

        any = true;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    if !any {
        return None;
    }

    Some((min_x, min_y, max_x - min_x + 1, max_y - min_y + 1))
}

fn overlay_event(image: &RgbaImage, id: u64) -> Result<OverlayEvent, AdapterError> {
    let Some((x, y, width, height)) = opaque_bounds(image) else {
        return Ok(OverlayEvent {
            id,
            png: String::new(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        });
    };

    let cropped = image::imageops::crop_imm(image, x, y, width, height).to_image();
    let png = VirtualAdapter::encode_png_base64(&cropped)?;

    Ok(OverlayEvent {
        id,
        png,
        x,
        y,
        width,
        height,
    })
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

        let jpeg = if self.prepared.is_empty() {
            Self::encode_jpeg_base64(image, 75)?
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

    fn preload_frame(&mut self, image: &RgbaImage) -> Result<(), AdapterError> {
        if image.width() != self.info.width || image.height() != self.info.height {
            return Err(AdapterError::SendFailed);
        }

        let jpeg = Self::encode_jpeg_base64(image, 75)?;
        self.prepared.push(jpeg);

        Ok(())
    }

    fn publish_clip(
        &mut self,
        frames: &[&RgbaImage],
        delays: &[Duration],
    ) -> Result<bool, AdapterError> {
        if frames.is_empty() {
            return Ok(false);
        }

        let width = self.info.width;
        let height = self.info.height;

        for frame in frames {
            if frame.width() != width || frame.height() != height {
                return Err(AdapterError::SendFailed);
            }
        }

        let encoded = Self::encode_clip(frames)?;

        self.clip_id = self.clip_id.wrapping_add(1);
        let id = self.clip_id;
        let delays_ms: Vec<u32> = delays.iter().copied().map(delay_ms).collect();

        self.app
            .emit_to(
                WINDOW_LABEL,
                "virtual-clip",
                ClipBegin {
                    id,
                    width,
                    height,
                    frame_count: encoded.len() as u32,
                    delays_ms,
                },
            )
            .map_err(|_| AdapterError::SendFailed)?;

        for (index, jpeg) in encoded.into_iter().enumerate() {
            self.app
                .emit_to(
                    WINDOW_LABEL,
                    "virtual-clip-frame",
                    ClipFrame {
                        id,
                        index: index as u32,
                        jpeg,
                    },
                )
                .map_err(|_| AdapterError::SendFailed)?;
        }

        Ok(true)
    }

    fn publish_overlay(&mut self, overlay: Option<&RgbaImage>) -> Result<(), AdapterError> {
        let event = match overlay {
            Some(image) => overlay_event(image, self.clip_id)?,
            None => OverlayEvent {
                id: self.clip_id,
                png: String::new(),
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
        };

        self.app
            .emit_to(WINDOW_LABEL, "virtual-overlay", event)
            .map_err(|_| AdapterError::SendFailed)?;

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

    #[test]
    fn zero_delay_is_at_least_one_millisecond() {
        assert_eq!(delay_ms(Duration::from_millis(0)), 1);
        assert_eq!(delay_ms(Duration::from_millis(16)), 16);
    }

    #[test]
    fn crops_overlay_to_opaque_pixels() {
        let mut image = RgbaImage::new(4, 4);

        image.put_pixel(2, 1, image::Rgba([255, 255, 255, 255]));

        assert_eq!(opaque_bounds(&image), Some((2, 1, 1, 1)));
    }

    #[test]
    fn fully_transparent_overlay_has_no_bounds() {
        let image = RgbaImage::new(2, 2);

        assert_eq!(opaque_bounds(&image), None);
    }
}
