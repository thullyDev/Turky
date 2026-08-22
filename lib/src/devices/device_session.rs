use crate::adapters::display_adapter::DisplayAdapter;
use crate::devices::device_info::DeviceInfo;

pub struct DeviceSession {
    pub info: DeviceInfo,
    pub adapter: Box<dyn DisplayAdapter>,
}

impl DeviceSession {
    pub fn new(
        adapter: Box<dyn DisplayAdapter>,
    ) -> Self {
        let info = adapter.info().clone();

        Self {
            info,
            adapter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::display_adapter::AdapterError;
    use crate::devices::device_id::DeviceId;

    struct FakeDisplayAdapter {
        info: DeviceInfo,
    }

    impl DisplayAdapter for FakeDisplayAdapter {
        fn info(&self) -> &DeviceInfo {
            &self.info
        }

        fn connect(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }

        fn disconnect(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }

        fn send_frame(
            &mut self,
            _frame: &[u8],
        ) -> Result<(), AdapterError> {
            Ok(())
        }

        fn is_connected(&self) -> bool {
            self.connected
        }

        fn clear(&mut self) -> Result<(), AdapterError> {
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
    fn creates_device_session_from_adapter() {
        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: true
        };

        let session = DeviceSession::new(
            Box::new(adapter)
        );

        assert_eq!(session.info.id, info.id);
        assert_eq!(session.info.name, info.name);
        assert_eq!(session.info.vendor_id, info.vendor_id);
        assert_eq!(session.info.product_id, info.product_id);
        assert_eq!(session.info.width, info.width);
        assert_eq!(session.info.height, info.height);
    }
}