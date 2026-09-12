#[cfg(test)]
use crate::adapters::display_adapter::{AdapterError, DisplayAdapter};

#[cfg(test)]
use crate::devices::{device_id::DeviceId, device_info::DeviceInfo};

#[cfg(test)]
pub struct FakeDisplayAdapter {
    pub info: DeviceInfo,
    pub connected: bool,
}

#[cfg(test)]
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

    fn send_frame(&mut self, _frame: &[u8]) -> Result<(), AdapterError> {
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn clear(&mut self) -> Result<(), AdapterError> {
        Ok(())
    }
}

#[cfg(test)]
pub fn get_test_device_info() -> DeviceInfo {
    DeviceInfo {
        id: DeviceId("test-device".to_string()),
        name: "Test Display".to_string(),
        vendor_id: 0x1234,
        product_id: 0x5678,
        width: 320,
        height: 480,
    }
}
