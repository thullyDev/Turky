use crate::adapters::usbs::usb_errors::UsbError;
use crate::adapters::usbs::usb_device_info::UsbDeviceInfo;

pub trait UsbAdapter {
    fn discover_devices(&self) -> Result<Vec<UsbDeviceInfo>, UsbError>;

    fn open(&mut self) -> Result<(), UsbError>;

    fn close(&mut self);

    fn write(
        &mut self,
        endpoint: u8,
        data: &[u8],
    ) -> Result<usize, UsbError>;

    fn read(
        &mut self,
        endpoint: u8,
        data: &mut [u8],
    ) -> Result<usize, UsbError>;
}


#[cfg(test)]
mod tests {
    use super::*;

    struct FakeUsbAdapter {
        opened: bool,
        devices: Vec<UsbDeviceInfo>,
    }

    impl FakeUsbAdapter {
        fn new() -> Self {
            Self {
                opened: false,
                devices: Vec::new(),
            }
        }
    }

    impl UsbAdapter for FakeUsbAdapter {
        fn discover_devices(
            &self,
        ) -> Result<Vec<UsbDeviceInfo>, UsbError> {
            Ok(self.devices.clone())
        }

        fn open(&mut self) -> Result<(), UsbError> {
            self.opened = true;
            Ok(())
        }

        fn close(&mut self) {
            self.opened = false;
        }

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
            for byte in data.iter_mut() {
                *byte = 0;
            }

            Ok(data.len())
        }
    }

    #[test]
    fn opens_usb_adapter() {
        let mut adapter = FakeUsbAdapter::new();

        let result = adapter.open();

        assert!(result.is_ok());
        assert!(adapter.opened);
    }

    #[test]
    fn closes_usb_adapter() {
        let mut adapter = FakeUsbAdapter::new();

        adapter.open().unwrap();
        adapter.close();

        assert!(!adapter.opened);
    }

    #[test]
    fn writes_data_to_endpoint() {
        let mut adapter = FakeUsbAdapter::new();

        let data = [1, 2, 3, 4];

        let result = adapter.write(
            0x01,
            &data,
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn reads_data_from_endpoint() {
        let mut adapter = FakeUsbAdapter::new();

        let mut data = [1, 2, 3, 4];

        let result = adapter.read(
            0x81,
            &mut data,
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);

        assert_eq!(
            data,
            [0, 0, 0, 0]
        );
    }
}