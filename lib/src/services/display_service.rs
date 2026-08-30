use ab_glyph::FontRef;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use crate::{
    schemas::display_schemas::RenderImageTextResponse,
    services::device_service::DeviceService,
};

pub struct DisplayService {
    device_serv: DeviceService,
}

impl DisplayService {
    pub fn new(device_serv: DeviceService) -> Self {
        Self {
            device_serv,
        }
    }

    pub fn render_text_image(
        &mut self,
        message: String,
    ) -> RenderImageTextResponse {
        let image = self.text_to_image(&message);

        println!(
            "Generated image: {}x{}",
            image.width(),
            image.height()
        );
        let registry = self
            .device_serv
            .registry();
        
        println!("Length ===> {}", registry.len());

        if let Some(device) = registry
            .first_mut()
        {
            let frame = image.as_raw();

            device
                .adapter
                .send_frame(frame)
                .expect("Failed to send frame");
        } else {
            println!("No display device available");
        }

        RenderImageTextResponse {
            message,
        }
    }

    fn text_to_image(&self, message: &str) -> RgbaImage {
        let width = 320;
        let height = 240;

        let mut image = ImageBuffer::from_pixel(
            width,
            height,
            Rgba([0, 0, 0, 255]),
        );

        let font = FontRef::try_from_slice(
            include_bytes!("../../assets/dejavu-sans-bold.ttf"),
        )
        .expect("Failed to load font");

        draw_text_mut(
            &mut image,
            Rgba([255, 255, 255, 255]),
            20,
            20,
            32.0,
            &font,
            message,
        );

        image
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        adapters::usbs::{
            usb_adapter::UsbAdapter,
            usb_connection::UsbConnection,
            usb_device_info::UsbDeviceInfo,
            usb_errors::UsbError,
        },
        devices::{
            device_registry::DeviceRegistry,
            device_session::DeviceSession,
        },
        factories::device_factory::DeviceFactory,
        utils::test_utils::{
            FakeDisplayAdapter,
            get_test_device_info,
        },
    };

    struct FakeUsbConnection;

    impl UsbConnection for FakeUsbConnection {
        fn open(&mut self) -> Result<(), UsbError> {
            Ok(())
        }

        fn close(&mut self) {}

        fn write(
            &mut self,
            _endpoint: u8,
            data: &[u8],
        ) -> Result<usize, UsbError> {
            Ok(data.len())
        }

        fn read(
            &mut self,
            _endpoint: u8,
            data: &mut [u8],
        ) -> Result<usize, UsbError> {
            Ok(data.len())
        }
    }

    struct FakeUsbAdapter {
        devices: Vec<UsbDeviceInfo>,
    }

    impl UsbAdapter for FakeUsbAdapter {
        fn discover_devices(
            &self,
        ) -> Result<Vec<UsbDeviceInfo>, UsbError> {
            Ok(self.devices.clone())
        }

        fn connect(
            &self,
            _device: &UsbDeviceInfo,
        ) -> Result<Box<dyn UsbConnection>, UsbError> {
            Ok(Box::new(FakeUsbConnection))
        }
    }

    #[test]
    fn creates_display_service() {
        let registry = DeviceRegistry::new();
        let factory = DeviceFactory::new();

        let usb = FakeUsbAdapter {
            devices: vec![],
        };

        let device_service = DeviceService::new(
            registry,
            factory,
            Box::new(usb),
        );

        let _service = DisplayService::new(
            device_service,
        );
    }

    #[test]
    fn renders_text_image_when_device_exists() {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: true,
        };

        let session = DeviceSession::new(
            Box::new(adapter),
        );

        registry.add(
            info.id.clone(),
            session,
        );

        let factory = DeviceFactory::new();

        let usb = FakeUsbAdapter {
            devices: vec![],
        };

        let device_service = DeviceService::new(
            registry,
            factory,
            Box::new(usb),
        );

        let mut service = DisplayService::new(
            device_service,
        );

        let result = service.render_text_image(
            "Hello".to_string(),
        );

        assert_eq!(
            result.message,
            "Hello"
        );
    }
}