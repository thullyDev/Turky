use rusb::{
    Device,
    DeviceHandle,
    GlobalContext,
};

use crate::adapters::usbs::usb_adapter::UsbAdapter;
use crate::adapters::usbs::usb_connection::UsbConnection;
use crate::adapters::usbs::usb_device_info::UsbDeviceInfo;
use crate::adapters::usbs::usb_errors::UsbError;

pub struct RusbAdapter;

impl RusbAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl UsbAdapter for RusbAdapter {
    fn discover_devices(
        &self,
    ) -> Result<Vec<UsbDeviceInfo>, UsbError> {

        let devices = rusb::devices()
            .map_err(|_| UsbError::DeviceNotFound)?;

        let mut device_info = Vec::new();

        for device in devices.iter() {

            let descriptor = device
                .device_descriptor()
                .map_err(|_| UsbError::DeviceNotFound)?;

            device_info.push(
                UsbDeviceInfo {
                    vendor_id: descriptor.vendor_id(),
                    product_id: descriptor.product_id(),
                    bus_number: device.bus_number(),
                    address: device.address(),
                }
            );
        }

        Ok(device_info)
    }

    fn connect(
        &self,
        info: &UsbDeviceInfo,
    ) -> Result<Box<dyn UsbConnection>, UsbError> {

        let devices = rusb::devices()
            .map_err(|_| UsbError::DeviceNotFound)?;

        let device = devices
            .iter()
            .find(|device| {
                device.bus_number() == info.bus_number
                    && device.address() == info.address
            })
            .ok_or(UsbError::DeviceNotFound)?;

        Ok(Box::new(
            RusbConnection::new(device)
        ))
    }
}


pub struct RusbConnection {
    device: Device<GlobalContext>,
    handle: Option<DeviceHandle<GlobalContext>>,
}

impl RusbConnection {
    pub fn new(
        device: Device<GlobalContext>,
    ) -> Self {

        Self {
            device,
            handle: None,
        }
    }
}

impl UsbConnection for RusbConnection {
    fn open(&mut self) -> Result<(), UsbError> {
        println!("=== USB OPEN ===");

        println!("Opening device...");

        let mut handle = self.device.open().map_err(|error| {
            println!("OPEN ERROR: {:?}", error);

            match error {
                rusb::Error::NoDevice => UsbError::DeviceNotFound,
                rusb::Error::Access => UsbError::AccessDenied,
                rusb::Error::Busy => UsbError::Busy,
                _ => UsbError::OpenFailed,
            }
        })?;

        println!("Device opened successfully");

        println!("Setting configuration 1...");

        handle.set_active_configuration(1).map_err(|error| {
            println!("CONFIG ERROR: {:?}", error);
            UsbError::OpenFailed
        })?;

        println!("Configuration set successfully");

        let config = self.device.active_config_descriptor()
            .map_err(|error| {
                println!("DESCRIPTOR ERROR: {:?}", error);
                UsbError::OpenFailed
            })?;

        println!("=== USB DESCRIPTORS ===");

        for interface in config.interfaces() {
            for descriptor in interface.descriptors() {
                println!(
                    "Interface {} | Alternate {} | Class 0x{:02X}",
                    descriptor.interface_number(),
                    descriptor.setting_number(),
                    descriptor.class_code()
                );

                for endpoint in descriptor.endpoint_descriptors() {
                    println!(
                        "Endpoint 0x{:02X} | Direction: {:?} | Type: {:?} | Max Packet: {}",
                        endpoint.address(),
                        endpoint.direction(),
                        endpoint.transfer_type(),
                        endpoint.max_packet_size()
                    );
                }
            }
        }

        println!("Claiming interface 0...");

        handle.claim_interface(0).map_err(|error| {
            println!("CLAIM ERROR: {:?}", error);

            match error {
                rusb::Error::Busy => UsbError::Busy,
                rusb::Error::Access => UsbError::AccessDenied,
                _ => UsbError::OpenFailed,
            }
        })?;

        println!("Interface 0 claimed successfully");

        self.handle = Some(handle);

        println!("=== USB OPEN COMPLETE ===");

        Ok(())
    }
    
    fn close(   
        &mut self,
    ) {
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
        
        match handle.write_bulk(
            endpoint,
            data,
            std::time::Duration::from_secs(1),
        ) {
            Ok(size) => {
                println!("RUSB WRITE SUCCESS: {} bytes", size);
                Ok(size)
            }

            Err(error) => {
                println!("==============================");
                println!("RUSB WRITE FAILED");
                println!("ERROR TYPE: {}", std::any::type_name_of_val(&error));
                println!("ERROR: {:?}", error);
                println!("==============================");

                Err(UsbError::TransferFailed)
            }
        }
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
    fn creates_rusb_adapter() {
        let _adapter = RusbAdapter::new();
    }

    #[test]
    fn discovers_connected_usb_devices() {

        let adapter = RusbAdapter::new();

        let result = adapter.discover_devices();

        assert!(result.is_ok());

        let devices = result.unwrap();

        assert!(!devices.is_empty());

        for device in devices {

            println!(
                "VID: {:04X}, PID: {:04X}, Bus: {}, Address: {}",
                device.vendor_id,
                device.product_id,
                device.bus_number,
                device.address,
            );

            assert_ne!(device.vendor_id, 0);
            assert_ne!(device.product_id, 0);
        }
    }

    #[test]
    fn creates_connection_for_discovered_device() {

        let adapter = RusbAdapter::new();

        let devices = adapter
            .discover_devices()
            .expect("Failed to discover USB devices");

        let info = &devices[0];

        let result = adapter.connect(info);

        assert!(result.is_ok());
    }

    #[test]
    fn connection_fails_when_not_open() {

        let adapter = RusbAdapter::new();

        let devices = adapter
            .discover_devices()
            .expect("Failed to discover USB devices");

        let info = &devices[0];

        let mut connection = adapter
            .connect(info)
            .expect("Failed to create connection");

        let data = [1, 2, 3, 4];

        let result = connection.write(
            0x01,
            &data,
        );

        assert!(matches!(
            result.unwrap_err(),
            UsbError::NotOpen
        ));
    }

    #[test]
    fn read_fails_when_not_open() {

        let adapter = RusbAdapter::new();

        let devices = adapter
            .discover_devices()
            .expect("Failed to discover USB devices");

        let info = &devices[0];

        let mut connection = adapter
            .connect(info)
            .expect("Failed to create connection");

        let mut data = [0u8; 4];

        let result = connection.read(
            0x81,
            &mut data,
        );

        assert!(matches!(
            result.unwrap_err(),
            UsbError::NotOpen
        ));
    }

    #[test]
    fn close_closes_connection() {

        let adapter = RusbAdapter::new();

        let devices = adapter
            .discover_devices()
            .expect("Failed to discover USB devices");

        let info = &devices[0];

        let mut connection = adapter
            .connect(info)
            .expect("Failed to create connection");

        connection.open()
            .expect("Failed to open connection");

        connection.close();

        let data = [1, 2, 3, 4];

        let result = connection.write(
            0x01,
            &data,
        );

        assert!(matches!(
            result.unwrap_err(),
            UsbError::NotOpen
        ));
    }
}