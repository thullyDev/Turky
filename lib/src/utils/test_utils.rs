#[cfg(test)]
use std::sync::{Arc, Mutex};

#[cfg(test)]
use crate::adapters::display_adapter::{AdapterError, DisplayAdapter};

#[cfg(test)]
use crate::devices::{device_id::DeviceId, device_info::DeviceInfo};

#[cfg(test)]
pub struct FakeDisplayAdapter {
    pub info: DeviceInfo,
    pub connected: bool,
    pub sent_frames: Arc<Mutex<Vec<Vec<u8>>>>,
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

    fn send_frame(&mut self, frame: &[u8]) -> Result<(), AdapterError> {
        self.sent_frames.lock().unwrap().push(frame.to_vec());

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
