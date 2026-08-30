use crate::adapters::usbs::usb_connection::UsbConnection;
use crate::adapters::usbs::usb_device_info::UsbDeviceInfo;
use crate::adapters::usbs::usb_errors::UsbError;

pub trait UsbAdapter: Send + Sync {
    fn discover_devices(
        &self,
    ) -> Result<Vec<UsbDeviceInfo>, UsbError>;

    fn connect(
        &self,
        info: &UsbDeviceInfo,
    ) -> Result<Box<dyn UsbConnection>, UsbError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::usbs::usb_connection::UsbConnection;

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


    impl FakeUsbAdapter {
        fn new() -> Self {
            Self {
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


        fn connect(
            &self,
            _device: &UsbDeviceInfo,
        ) -> Result<Box<dyn UsbConnection>, UsbError> {

            Ok(Box::new(
                FakeUsbConnection
            ))
        }
    }


    #[test]
    fn discovers_usb_devices() {
        let adapter = FakeUsbAdapter::new();
        let result = adapter.discover_devices();

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }


    #[test]
    fn connects_to_usb_device() {
        let adapter = FakeUsbAdapter::new();
        let device = UsbDeviceInfo {
            vendor_id: 0x1CBE,
            product_id: 0x0088,
            bus_number: 1,
            address: 1,
        };

        let result = adapter.connect(&device);

        assert!(result.is_ok());
    }
}
