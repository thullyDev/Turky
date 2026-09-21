use crate::services::device_service::DeviceService;
use image::{codecs::png::PngEncoder, ImageEncoder, RgbaImage};

pub struct DisplayService {
    device_serv: DeviceService,
}

impl DisplayService {
    pub fn new(device_serv: DeviceService) -> Self {
        Self { device_serv }
    }

    pub fn render_image(&mut self, image: RgbaImage) {
        let png = self.prepare_image(&image);
        let registry = self.device_serv.registry();

        if let Some(device) = registry.first_mut() {
            device
                .adapter
                .send_frame(&png)
                .expect("Failed to send frame");
        } else {
            println!("No display device available");
        }
    }

    fn prepare_image(&self, image: &RgbaImage) -> Vec<u8> {
        let png = self.rgba_image_to_png(&image);
        png
    }

    fn rgba_image_to_png(&self, image: &RgbaImage) -> Vec<u8> {
        let mut png = Vec::new();

        let encoder = PngEncoder::new(&mut png);

        encoder
            .write_image(
                image.as_raw(),
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgba8,
            )
            .expect("Failed to encode image as PNG");

        png
    }
}

#[cfg(test)]
mod tests {
    use image::Rgba;

    use super::*;

    use std::sync::{Arc, Mutex};

    use crate::{
        adapters::usbs::{
            usb_adapter::UsbAdapter, usb_connection::UsbConnection, usb_device_info::UsbDeviceInfo,
            usb_errors::UsbError,
        },
        devices::{device_registry::DeviceRegistry, device_session::DeviceSession},
        factories::device_factory::DeviceFactory,
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
    fn renders_text_image_when_device_exists() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));
        let mut service = create_test_service(Arc::clone(&sent_frames));
        let image = RgbaImage::from_pixel(480, 1920, Rgba([255, 0, 0, 255]));

        service.render_image(image);

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 1);
        assert!(!frames[0].is_empty());

        let decoded =
            image::load_from_memory(&frames[0]).expect("Sent frame should be a valid PNG");

        assert_eq!(decoded.width(), 480);
        assert_eq!(decoded.height(), 1920);
    }

    #[test]
    fn does_not_panic_when_no_device_exists() {
        let registry = DeviceRegistry::new();
        let factory = DeviceFactory::new();
        let usb = FakeUsbAdapter { devices: vec![] };
        let device_service = DeviceService::new_for_test(registry, factory, Box::new(usb));
        let mut service = DisplayService::new(device_service);
        let image = RgbaImage::from_pixel(480, 1920, Rgba([255, 0, 0, 255]));

        service.render_image(image);
    }
}
