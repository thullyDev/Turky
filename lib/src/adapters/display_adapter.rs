use crate::devices::device_id::DeviceId;
use crate::devices::device_info::DeviceInfo;

pub trait DisplayAdapter: Send + Sync {
    fn info(&self) -> &DeviceInfo;

    fn connect(&mut self) -> Result<(), AdapterError>;

    fn disconnect(&mut self) -> Result<(), AdapterError>;

    fn send_frame(&mut self, frame: &[u8]) -> Result<(), AdapterError>;

    fn clear(&mut self) -> Result<(), AdapterError>;

    fn is_connected(&self) -> bool;
}

#[derive(Debug)]
pub enum AdapterError {
    ConnectionFailed,
    Disconnected,
    DeviceBusy,
    SendFailed,
    ClearFailed,
    UnsupportedOperation,
    TransferFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeDisplayAdapter {
        info: DeviceInfo,
        connected: bool,
        frames_sent: usize,
    }

    impl FakeDisplayAdapter {
        fn new(info: DeviceInfo) -> Self {
            Self {
                info,
                connected: false,
                frames_sent: 0,
            }
        }
    }

    impl DisplayAdapter for FakeDisplayAdapter {
        fn info(&self) -> &DeviceInfo {
            &self.info
        }

        fn connect(&mut self) -> Result<(), AdapterError> {
            self.connected = true;
            Ok(())
        }

        fn disconnect(&mut self) -> Result<(), AdapterError> {
            self.connected = false;
            Ok(())
        }

        fn is_connected(&self) -> bool {
            self.connected
        }

        fn send_frame(&mut self, _frame: &[u8]) -> Result<(), AdapterError> {
            if !self.connected {
                return Err(AdapterError::Disconnected);
            }

            self.frames_sent += 1;
            Ok(())
        }

        fn clear(&mut self) -> Result<(), AdapterError> {
            if !self.connected {
                return Err(AdapterError::Disconnected);
            }

            Ok(())
        }
    }

    fn get_test_device_info() -> DeviceInfo {
        DeviceInfo {
            id: DeviceId("test-device".to_string()),
            name: "Test Display".to_string(),
            vendor_id: 0x1234,
            product_id: 0x5678,
            width: 320,
            height: 480,
        }
    }

    #[test]
    fn adapter_can_connect() {
        let info = get_test_device_info();

        let mut adapter = FakeDisplayAdapter::new(info);

        assert!(adapter.connect().is_ok());
    }

    #[test]
    fn adapter_returns_device_info() {
        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter::new(info);

        assert_eq!(adapter.info().name, "Test Display");
        assert_eq!(adapter.info().vendor_id, 0x1234);
        assert_eq!(adapter.info().product_id, 0x5678);
    }
}
