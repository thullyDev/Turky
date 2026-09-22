use crate::adapters::display_adapter::{AdapterError, DisplayAdapter};
use crate::devices::device_info::DeviceInfo;
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const WINDOW_LABEL: &str = "virtual-display";

#[derive(Clone, serde::Serialize)]
struct FrameEvent {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

pub struct VirtualAdapter {
    info: DeviceInfo,
    connected: bool,
    app: AppHandle,
}

impl VirtualAdapter {
    pub fn new(app: AppHandle, info: DeviceInfo) -> Self {
        Self {
            info,
            connected: false,
            app,
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

    fn send_frame(&mut self, frame: &[u8]) -> Result<(), AdapterError> {
        if !self.connected {
            return Err(AdapterError::Disconnected);
        }

        let image = image::load_from_memory(frame)
            .map_err(|_| AdapterError::SendFailed)?
            .to_rgba8();

        if image.width() != self.info.width || image.height() != self.info.height {
            return Err(AdapterError::SendFailed);
        }

        println!(
            "Sending virtual frame: {}x{} ({} bytes)",
            image.width(),
            image.height(),
            image.as_raw().len()
        );

        let event = FrameEvent {
            width: image.width(),
            height: image.height(),
            pixels: image.into_raw(),
        };

        self.app
            .emit_to(WINDOW_LABEL, "virtual-frame", event)
            .map_err(|_| AdapterError::SendFailed)?;

        println!("virtual-frame emitted");

        Ok(())
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

        let pixels = vec![
            255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
        ];

        let event = FrameEvent {
            width: info.width,
            height: info.height,
            pixels: pixels.clone(),
        };

        assert_eq!(event.width, 2);
        assert_eq!(event.height, 2);
        assert_eq!(event.pixels, pixels);
    }
}
