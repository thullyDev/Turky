use std::io::Cursor;

use crate::{devices::device_id::DeviceId, services::device_service::DeviceService};

use image::{codecs::gif::GifDecoder, AnimationDecoder, RgbaImage};

pub struct DisplayService {
    pub device_serv: DeviceService,
}

impl DisplayService {
    pub fn new(device_serv: DeviceService) -> Self {
        Self { device_serv }
    }

    pub fn render_image(&mut self, image: &RgbaImage, device_id: &DeviceId) {
        if let Some(device) = self.device_serv.registry().get_mut(device_id) {
            device
                .adapter
                .send_frame(image)
                .expect("Failed to send frame");
        }
    }

    pub fn render_gif(&mut self, bytes: &[u8], device_id: &DeviceId) -> Result<(), String> {
        self.play_gif(bytes, device_id, None)
    }

    fn play_gif(
        &mut self,
        bytes: &[u8],
        device_id: &DeviceId,
        repeats: Option<usize>,
    ) -> Result<(), String> {
        let decoder = GifDecoder::new(Cursor::new(bytes))
            .map_err(|e| format!("Failed to create GIF decoder: {e}"))?;

        let frame_duration = std::time::Duration::from_secs_f64(1.0 / 60.0);
        let mut frames = decoder.into_frames();
        let mut decoded = Vec::new();

        loop {
            let Some(frame_result) = frames.next() else {
                break;
            };

            let frame = frame_result.map_err(|e| format!("Failed to decode GIF frame: {e}"))?;

            decoded.push(frame.into_buffer());
        }

        if decoded.is_empty() || self.device_serv.registry().get(device_id).is_none() {
            return Ok(());
        }

        if let Some(device) = self.device_serv.registry().get_mut(device_id) {
            device.adapter.reset_preloaded();

            for image in &decoded {
                device
                    .adapter
                    .preload_frame(image)
                    .map_err(|error| format!("Failed to prepare frame: {error:?}"))?;
            }
        }

        let mut played = 0;

        loop {
            for image in &decoded {
                let start = std::time::Instant::now();

                self.render_image(image, device_id);

                let elapsed = start.elapsed();

                if elapsed < frame_duration {
                    std::thread::sleep(frame_duration - elapsed);
                }
            }

            played += 1;

            if repeats.is_some_and(|limit| played >= limit) {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use gif::{Encoder, Frame, Repeat};
    use image::Rgba;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use crate::{
        adapters::usbs::{
            usb_adapter::UsbAdapter, usb_connection::UsbConnection, usb_device_info::UsbDeviceInfo,
            usb_errors::UsbError,
        },
        devices::{device_registry::DeviceRegistry, device_session::DeviceSession},
        factories::device_factory::DeviceFactory,
        services::device_service::DeviceService,
        utils::test_utils::{get_test_device_info, FakeDisplayAdapter},
    };

    struct FakeUsbConnection;

    impl UsbConnection for FakeUsbConnection {
        fn open(&mut self) -> Result<(), UsbError> {
            Ok(())
        }

        fn close(&mut self) {}

        fn write(&mut self, _endpoint: u8, data: &[u8]) -> Result<usize, UsbError> {
            Ok(data.len())
        }

        fn read(&mut self, _endpoint: u8, data: &mut [u8]) -> Result<usize, UsbError> {
            Ok(data.len())
        }
    }

    struct FakeUsbAdapter {
        devices: Vec<UsbDeviceInfo>,
    }

    impl UsbAdapter for FakeUsbAdapter {
        fn discover_devices(&self) -> Result<Vec<UsbDeviceInfo>, UsbError> {
            Ok(self.devices.clone())
        }

        fn connect(&self, _device: &UsbDeviceInfo) -> Result<Box<dyn UsbConnection>, UsbError> {
            Ok(Box::new(FakeUsbConnection))
        }
    }

    fn create_test_service(sent_frames: Arc<Mutex<Vec<Vec<u8>>>>) -> DisplayService {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: true,
            sent_frames,
        };

        let session = DeviceSession::new(Box::new(adapter));

        registry.add(info.id.clone(), session);

        let factory = DeviceFactory::new();

        let usb = FakeUsbAdapter { devices: vec![] };

        let device_service = DeviceService::new_for_test(registry, factory, Box::new(usb));

        DisplayService::new(device_service)
    }

    #[test]
    fn creates_display_service() {
        let registry = DeviceRegistry::new();

        let factory = DeviceFactory::new();

        let usb = FakeUsbAdapter { devices: vec![] };

        let device_service = DeviceService::new_for_test(registry, factory, Box::new(usb));

        let _service = DisplayService::new(device_service);
    }

    #[test]
    fn renders_image_when_device_exists() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(Arc::clone(&sent_frames));

        let image = RgbaImage::from_pixel(480, 1920, Rgba([255, 0, 0, 255]));

        let device_id = get_test_device_info().id.clone();

        service.render_image(&image, &device_id);

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 1);

        assert_eq!(frames[0].len(), 480 * 1920 * 4);
    }

    #[test]
    fn does_not_panic_when_no_device_exists() {
        let registry = DeviceRegistry::new();

        let factory = DeviceFactory::new();

        let usb = FakeUsbAdapter { devices: vec![] };

        let device_service = DeviceService::new_for_test(registry, factory, Box::new(usb));

        let mut service = DisplayService::new(device_service);

        let image = RgbaImage::from_pixel(480, 1920, Rgba([255, 0, 0, 255]));

        let device_id = get_test_device_info().id.clone();

        service.render_image(&image, &device_id);
    }

    fn create_test_gif() -> Vec<u8> {
        let mut bytes = Vec::new();

        {
            let mut encoder =
                Encoder::new(&mut bytes, 2, 2, &[]).expect("Failed to create GIF encoder");

            encoder
                .set_repeat(Repeat::Infinite)
                .expect("Failed to set GIF repeat");

            let red = vec![255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];

            let mut red_frame = Frame::from_rgb(2, 2, &red);

            red_frame.delay = 1;

            encoder
                .write_frame(&red_frame)
                .expect("Failed to write red frame");

            let blue = vec![0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0, 255];

            let mut blue_frame = Frame::from_rgb(2, 2, &blue);

            blue_frame.delay = 1;

            encoder
                .write_frame(&blue_frame)
                .expect("Failed to write blue frame");
        }

        bytes
    }

    #[test]
    fn renders_gif_frames_when_device_exists() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(Arc::clone(&sent_frames));

        let device_id = get_test_device_info().id.clone();

        let gif = create_test_gif();

        let result = service.play_gif(&gif, &device_id, Some(1));

        assert!(result.is_ok(), "render_gif failed: {:?}", result.err());

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 2, "Expected 2 GIF frames to be rendered");

        assert_eq!(frames[0].len(), 2 * 2 * 4);

        assert_eq!(frames[1].len(), 2 * 2 * 4);
    }

    #[test]
    fn render_gif_returns_error_for_invalid_data() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(sent_frames);

        let device_id = get_test_device_info().id.clone();

        let invalid_gif = b"this is not a gif";

        let result = service.render_gif(invalid_gif, &device_id);

        assert!(result.is_err(), "Expected invalid GIF to return an error");

        let error = result.unwrap_err();

        assert!(
            error.contains("Failed to create GIF decoder"),
            "Unexpected error: {error}"
        );
    }

    #[test]
    fn render_gif_targets_30_fps() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(sent_frames);

        let device_id = get_test_device_info().id.clone();

        let gif = create_test_gif();

        let start = Instant::now();

        service
            .play_gif(&gif, &device_id, Some(1))
            .expect("render_gif failed");

        let elapsed = start.elapsed();

        assert!(
            elapsed >= Duration::from_millis(60),
            "GIF rendered too quickly: {:?}",
            elapsed
        );
    }

    #[test]
    fn render_gif_with_missing_device_does_not_send_frames() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(Arc::clone(&sent_frames));

        let device_id = DeviceId("does-not-exist".to_string());

        let gif = create_test_gif();

        let result = service.render_gif(&gif, &device_id);

        assert!(result.is_ok(), "Expected render_gif to complete");

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 0, "No frames should have been sent");
    }
}
