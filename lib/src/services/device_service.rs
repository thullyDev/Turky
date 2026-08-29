use crate::adapters::usbs::usb_adapter::UsbAdapter;
use crate::devices::device_session::DeviceSession;
use crate::factories::device_factory::DeviceFactory;
use crate::devices::device_registry::DeviceRegistry;

pub struct DeviceService {
    factory: DeviceFactory,
    registry: DeviceRegistry,
    usb: Box<dyn UsbAdapter>
}

impl DeviceService {
    pub fn new(
        registry: DeviceRegistry,
        factory: DeviceFactory,
        usb: Box<dyn UsbAdapter>
    ) -> Self {
        Self {
            factory,
            registry,
            usb
        }
    }

    pub fn discover_devices(
        &mut self,
    ) {
        let devices = self.usb.discover_devices().unwrap();

        for device in devices {
            let adapter = self.factory.create_device(device.vendor_id, device.product_id);

            if let Some(adapter) = adapter {
                let device_id = adapter.info().id.clone();
                let session = DeviceSession::new(adapter);
                self.registry.add(device_id, session);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{adapters::usbs::{usb_device_info::UsbDeviceInfo, usb_errors::UsbError}, devices::device_model::DeviceModel::Turzx};
    struct FakeUsbAdapter {
        devices: Vec<UsbDeviceInfo>,
    }

    impl UsbAdapter for FakeUsbAdapter {
        fn discover_devices(
            &self,
        ) -> Result<Vec<UsbDeviceInfo>, UsbError> {
            Ok(self.devices.clone())
        }

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


    #[test]
    fn creates_device_service() {
        let registry = DeviceRegistry::new();
        let factory = DeviceFactory::new();

        let usb = FakeUsbAdapter {
            devices: vec![],
        };

        let _service = DeviceService::new(
            registry,
            factory,
            Box::new(usb),
        );
    }

    #[test]
    fn discovers_connected_devices() {
        let factory = DeviceFactory::new();
        let registry = DeviceRegistry::new();

        let usb = FakeUsbAdapter {
            devices: vec![
                UsbDeviceInfo {
                    vendor_id: Turzx.vendor_id(),
                    product_id: Turzx.product_id(),
                },
            ],
        };

        let mut service = DeviceService::new(
            registry,
            factory,
            Box::new(usb),
        );

        service.discover_devices();

        assert_eq!(service.registry.len(), 1);
    }
}