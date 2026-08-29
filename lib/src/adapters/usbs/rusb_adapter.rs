use rusb::{
    Device,
    DeviceHandle,
    GlobalContext,
};

use crate::adapters::usbs::usb_adapter::UsbAdapter;
use crate::adapters::usbs::usb_device_info::UsbDeviceInfo;
use crate::adapters::usbs::usb_errors::UsbError;

pub struct RusbAdapter {
    device: Device<GlobalContext>,
    handle: Option<DeviceHandle<GlobalContext>>,
    devices: Vec<UsbDeviceInfo>,
}

impl RusbAdapter {
    pub fn new(
        device: Device<GlobalContext>,
    ) -> Self {
        Self {
            device,
            handle: None,
            devices: Vec::new(),
        }
    }
}

impl UsbAdapter for RusbAdapter {
    fn discover_devices(
        &self,
    ) -> Result<Vec<UsbDeviceInfo>, UsbError> {
        let mut device_info = Vec::new();
        let devices = rusb::devices().map_err(|_| UsbError::DeviceNotFound)?;

        for device in devices.iter() {
            let descriptor = device.device_descriptor().map_err(|_| UsbError::DeviceNotFound)?;
            
            device_info.push(UsbDeviceInfo {
                vendor_id: descriptor.vendor_id(),
                product_id: descriptor.product_id(),
            })
        }

        Ok(device_info.clone())
    }

    fn open(&mut self) -> Result<(), UsbError> {
        let handle = self.device
            .open()
            .map_err(|_| UsbError::DeviceNotFound)?;

        self.handle = Some(handle);

        Ok(())
    }

    fn close(&mut self) {
        self.handle = None;
    }

    fn write(
        &mut self,
        endpoint: u8,
        data: &[u8],
    ) -> Result<usize, UsbError> {
        let handle = self
            .handle
            .as_mut()
            .ok_or(UsbError::NotOpen)?;

        handle
            .write_bulk(
                endpoint,
                data,
                std::time::Duration::from_secs(1),
            )
            .map_err(|_| UsbError::TransferFailed)
    }

    fn read(
        &mut self,
        endpoint: u8,
        data: &mut [u8],
    ) -> Result<usize, UsbError> {
        let handle = self
            .handle
            .as_mut()
            .ok_or(UsbError::NotOpen)?;

        handle
            .read_bulk(
                endpoint,
                data,
                std::time::Duration::from_secs(1),
            )
            .map_err(|_| UsbError::TransferFailed)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_connected_usb_devices() {
        let device = rusb::devices()
            .unwrap()
            .iter()
            .next()
            .expect("No USB device connected");

        let adapter = RusbAdapter::new(device);
        let result = adapter.discover_devices();

        assert!(result.is_ok());

        let devices = result.unwrap();

        assert!(!devices.is_empty());

        for device in devices {
            assert_ne!(device.vendor_id, 0);
            assert_ne!(device.product_id, 0);
        }
    }

    #[test]
    fn write_fails_when_device_is_not_open() {
        let device = rusb::devices()
            .unwrap()
            .iter()
            .next()
            .expect("No USB device connected");

        let mut adapter = RusbAdapter::new(device);

        let data = [1, 2, 3, 4];

        let result = adapter.write(
            0x01,
            &data,
        );

        assert!(result.is_err());

        assert!(matches!(
            result.unwrap_err(),
            UsbError::NotOpen
        ));
    }

    #[test]
    fn read_fails_when_device_is_not_open() {
        let device = rusb::devices()
            .unwrap()
            .iter()
            .next()
            .expect("No USB device connected");

        let mut adapter = RusbAdapter::new(device);

        let mut data = [0u8; 4];

        let result = adapter.read(
            0x81,
            &mut data,
        );

        assert!(result.is_err());

        assert!(matches!(
            result.unwrap_err(),
            UsbError::NotOpen
        ));
    }

    #[test]
    fn close_closes_opened_handle() {
        let device = rusb::devices()
            .unwrap()
            .iter()
            .next()
            .expect("No USB device connected");

        let mut adapter = RusbAdapter::new(device);

        adapter.close();

        let data = [1, 2, 3, 4];

        let result = adapter.write(
            0x01,
            &data,
        );

        assert!(matches!(
            result.unwrap_err(),
            UsbError::NotOpen
        ));
    }
}