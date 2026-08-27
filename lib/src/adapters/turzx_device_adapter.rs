use crate::adapters::display_adapter::{
    AdapterError,
    DisplayAdapter,
};

use crate::devices::device_info::DeviceInfo;
use crate::devices::device_id::DeviceId;
use crate::devices::device_model::DeviceModel;

pub struct TurzxDeviceAdapter {
    info: DeviceInfo,
    connected: bool,
}

impl TurzxDeviceAdapter {
    pub fn new() -> Self {
        Self {
            info: DeviceInfo {
                id: DeviceId("turzx".to_string()),
                name: "TURZX1.0".to_string(),
                vendor_id: DeviceModel::Turzx.vendor_id(),
                product_id: DeviceModel::Turzx.product_id(),
                width: 320,
                height: 480,
            },
            connected: false,
        } 
    }
}

impl DisplayAdapter for TurzxDeviceAdapter {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn connect(&mut self) -> Result<(), AdapterError> {
        // TODO: connect to TURZX using rusb
        self.connected = true;

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), AdapterError> {
        // TODO: disconnect from TURZX
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

        // TODO: send frame to TURZX using rusb

        Ok(())
    }

    fn clear(&mut self) -> Result<(), AdapterError> {
        if !self.connected {
            return Err(AdapterError::Disconnected);
        }

        // TODO: send TURZX clear command

        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_turzx_device_adapter() {
        let adapter = TurzxDeviceAdapter::new();

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
        let adapter = TurzxDeviceAdapter::new();

        assert!(!adapter.is_connected());
    }

    #[test]
    fn adapter_can_connect() {
        let mut adapter = TurzxDeviceAdapter::new();

        adapter.connect().unwrap();

        assert!(adapter.is_connected());
    }

    #[test]
    fn adapter_can_disconnect() {
        let mut adapter = TurzxDeviceAdapter::new();

        adapter.connect().unwrap();
        assert!(adapter.is_connected());

        adapter.disconnect().unwrap();

        assert!(!adapter.is_connected());
    }

    #[test]
    fn cannot_send_frame_when_disconnected() {
        let mut adapter = TurzxDeviceAdapter::new();

        let result = adapter.send_frame(&[]);

        assert!(result.is_err());
    }

    #[test]
    fn cannot_clear_when_disconnected() {
        let mut adapter = TurzxDeviceAdapter::new();

        let result = adapter.clear();

        assert!(result.is_err());
    }
}