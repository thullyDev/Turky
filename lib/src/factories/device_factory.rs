use crate::adapters::usbs::usb_connection::UsbConnection;
use crate::adapters::usbs::usb_device_info::UsbDeviceInfo;
use crate::devices::device_model::DeviceModel::Turzx;
use crate::adapters::display_adapter::DisplayAdapter;
use crate::adapters::devices::turzx_device_adapter::TurzxDeviceAdapter;

pub struct DeviceFactory;

impl DeviceFactory {
    pub fn new() -> Self {
        Self
    }

    pub fn create_device(
        &self,
        info: UsbDeviceInfo,
        connection: Box<dyn UsbConnection>,
    ) -> Option<Box<dyn DisplayAdapter>> {

        if info.vendor_id == Turzx.vendor_id()
            && info.product_id == Turzx.product_id()
        {
            Some(Box::new(
                TurzxDeviceAdapter::new(
                    info,
                    connection,
                )
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::adapters::usbs::usb_connection::UsbConnection;
    use crate::adapters::usbs::usb_device_info::UsbDeviceInfo;
    use crate::adapters::usbs::usb_errors::UsbError;
    use crate::devices::device_model::DeviceModel::Turzx;


    struct FakeUsbConnection;


    impl FakeUsbConnection {

        fn new() -> Self {
            Self
        }
    }


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


    fn create_test_info() -> UsbDeviceInfo {
        UsbDeviceInfo {
            vendor_id: Turzx.vendor_id(),
            product_id: Turzx.product_id(),
            bus_number: 1,
            address: 1,
        }
    }


    #[test]
    fn creates_turzx_adapter_for_supported_device() {

        let info = create_test_info();

        let connection = FakeUsbConnection::new();

        let factory = DeviceFactory::new();

        let adapter = factory.create_device(
            info,
            Box::new(connection),
        );

        assert!(adapter.is_some());

        let adapter = adapter.unwrap();

        assert_eq!(
            adapter.info().vendor_id,
            Turzx.vendor_id()
        );

        assert_eq!(
            adapter.info().product_id,
            Turzx.product_id()
        );
    }


    #[test]
    fn returns_none_for_unsupported_device() {

        let info = UsbDeviceInfo {
            vendor_id: 0xFFFF,
            product_id: 0xFFFF,
            bus_number: 1,
            address: 1,
        };

        let connection = FakeUsbConnection::new();

        let factory = DeviceFactory::new();

        let adapter = factory.create_device(
            info,
            Box::new(connection),
        );

        assert!(adapter.is_none());
    }
}

