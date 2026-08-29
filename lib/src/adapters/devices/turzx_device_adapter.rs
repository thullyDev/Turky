use crate::adapters::display_adapter::{
    AdapterError,
    DisplayAdapter,
};
use crate::adapters::usbs::usb_connection::UsbConnection;
use crate::adapters::usbs::usb_device_info::UsbDeviceInfo;
use crate::devices::device_info::DeviceInfo;
use crate::devices::device_id::DeviceId;

pub struct TurzxDeviceAdapter {
    info: DeviceInfo,
    connection: Box<dyn UsbConnection>,
    connected: bool,
}


impl TurzxDeviceAdapter {
    pub fn new(
        info: UsbDeviceInfo,
        connection: Box<dyn UsbConnection>,
    ) -> Self {
        Self {
            info: DeviceInfo {
                id: DeviceId("turzx".to_string()),
                name: "TURZX1.0".to_string(),
                vendor_id: info.vendor_id,
                product_id: info.product_id,
                width: 320,
                height: 480,
            },
            connection,
            connected: false,
        }
    }
}
impl DisplayAdapter for TurzxDeviceAdapter {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn connect(&mut self) -> Result<(), AdapterError> {
        self.connection
            .open()
            .map_err(|_| AdapterError::ConnectionFailed)?;
        self.connected = true;
        
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), AdapterError> {
        self.connection.close();
        self.connected = false;

        Ok(())
    }

    fn send_frame(
        &mut self,
        frame: &[u8],
    ) -> Result<(), AdapterError> {
        if !self.connected {
            return Err(AdapterError::Disconnected);
        }

        self.connection
            .write(0x01, frame)
            .map_err(|_| AdapterError::TransferFailed)?;

        Ok(())
    }

    fn clear(&mut self) -> Result<(), AdapterError> {

        if !self.connected {
            return Err(AdapterError::Disconnected);
        }

        self.connection
            .write(0x01, &[/* clear command */])
            .map_err(|_| AdapterError::Disconnected)?;

        Ok(())
    }
    fn is_connected(&self) -> bool {
        self.connected
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::adapters::usbs::usb_connection::UsbConnection;
    use crate::adapters::usbs::usb_device_info::UsbDeviceInfo;
    use crate::adapters::usbs::usb_errors::UsbError;
use crate::devices::device_model::DeviceModel;


    struct FakeUsbConnection {
        opened: bool,
    }


    impl FakeUsbConnection {

        fn new() -> Self {
            Self {
                opened: false,
            }
        }
    }


    impl UsbConnection for FakeUsbConnection {

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

            if !self.opened {
                return Err(UsbError::NotOpen);
            }

            Ok(data.len())
        }

        fn read(
            &mut self,
            _endpoint: u8,
            data: &mut [u8],
        ) -> Result<usize, UsbError> {

            if !self.opened {
                return Err(UsbError::NotOpen);
            }

            Ok(data.len())
        }
    }


    fn get_test_usb_device_info() -> UsbDeviceInfo {
        UsbDeviceInfo {
            vendor_id: DeviceModel::Turzx.vendor_id(),
            product_id: DeviceModel::Turzx.product_id(),
            bus_number: 1,
            address: 1,
        }
    }


    fn create_adapter() -> TurzxDeviceAdapter {

        let info = get_test_usb_device_info();

        let connection = FakeUsbConnection::new();

        TurzxDeviceAdapter::new(
            info,
            Box::new(connection),
        )
    }


    #[test]
    fn creates_turzx_device_adapter() {

        let adapter = create_adapter();

        assert_eq!(
            adapter.info().vendor_id,
            DeviceModel::Turzx.vendor_id()
        );

        assert_eq!(
            adapter.info().product_id,
            DeviceModel::Turzx.product_id()
        );
    }


    #[test]
    fn adapter_starts_disconnected() {

        let adapter = create_adapter();

        assert!(!adapter.is_connected());
    }


    #[test]
    fn adapter_can_connect() {

        let mut adapter = create_adapter();

        adapter
            .connect()
            .unwrap();

        assert!(adapter.is_connected());
    }


    #[test]
    fn adapter_can_disconnect() {

        let mut adapter = create_adapter();

        adapter
            .connect()
            .unwrap();

        assert!(adapter.is_connected());

        adapter
            .disconnect()
            .unwrap();

        assert!(!adapter.is_connected());
    }


    #[test]
    fn cannot_send_frame_when_disconnected() {

        let mut adapter = create_adapter();

        let result = adapter.send_frame(&[]);

        assert!(result.is_err());

        assert!(matches!(
            result.unwrap_err(),
            AdapterError::Disconnected
        ));
    }


    #[test]
    fn cannot_clear_when_disconnected() {

        let mut adapter = create_adapter();

        let result = adapter.clear();

        assert!(result.is_err());

        assert!(matches!(
            result.unwrap_err(),
            AdapterError::Disconnected
        ));
    }


    #[test]
    fn can_send_frame_when_connected() {

        let mut adapter = create_adapter();

        adapter
            .connect()
            .unwrap();

        let frame = [1, 2, 3, 4];

        let result = adapter.send_frame(&frame);

        assert!(result.is_ok());
    }
}