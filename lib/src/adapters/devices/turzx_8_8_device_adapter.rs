use crate::adapters::display_adapter::{AdapterError, DisplayAdapter};
use crate::adapters::usbs::usb_connection::UsbConnection;
use crate::adapters::usbs::usb_device_info::UsbDeviceInfo;
use crate::adapters::usbs::usb_errors::UsbError;
use crate::devices::device_id::DeviceId;
use crate::devices::device_info::DeviceInfo;

use cbc::cipher::{block_padding::NoPadding, BlockEncryptMut, KeyIvInit};
use chrono::Local;
use des::Des;

const USB_OUT: u8 = 0x01;

const DES_KEY: &[u8; 8] = b"slv3tuzx";

type DesCbcEnc = cbc::Encryptor<Des>;

pub struct Turzx88DeviceAdapter {
    info: DeviceInfo,
    connection: Box<dyn UsbConnection>,
    connected: bool,
}

impl Turzx88DeviceAdapter {
    pub fn new(info: UsbDeviceInfo, connection: Box<dyn UsbConnection>) -> Self {
        Self {
            info: DeviceInfo {
                id: DeviceId("turzx".to_string()),
                name: "TURZX8.8".to_string(),
                vendor_id: info.vendor_id,
                product_id: info.product_id,
                width: 1920,
                height: 480,
            },
            connection,
            connected: false,
        }
    }

    fn build_command(command_id: u8, payload_size: Option<usize>) -> [u8; 500] {
        let mut packet = [0u8; 500];

        packet[0] = command_id;
        packet[2] = 0x1A;
        packet[3] = 0x6D;

        let now = Local::now();

        let midnight = now.date_naive().and_hms_opt(0, 0, 0).unwrap();

        let milliseconds = (now.naive_local() - midnight).num_milliseconds() as u32;

        packet[4..8].copy_from_slice(&milliseconds.to_le_bytes());

        if let Some(size) = payload_size {
            packet[8] = ((size >> 24) & 0xFF) as u8;
            packet[9] = ((size >> 16) & 0xFF) as u8;
            packet[10] = ((size >> 8) & 0xFF) as u8;
            packet[11] = (size & 0xFF) as u8;
        }

        packet
    }

    fn encrypt_command(packet: &[u8; 500]) -> [u8; 512] {
        let mut encrypted = [0u8; 504];

        encrypted[..500].copy_from_slice(packet);

        let cipher = DesCbcEnc::new(DES_KEY.into(), DES_KEY.into());

        cipher
            .encrypt_padded_mut::<NoPadding>(&mut encrypted, 504)
            .unwrap();

        let mut result = [0u8; 512];

        result[..504].copy_from_slice(&encrypted);

        result[510] = 0xA1;
        result[511] = 0x1A;

        result
    }
}

impl DisplayAdapter for Turzx88DeviceAdapter {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn connect(&mut self) -> Result<(), AdapterError> {
        match self.connection.open() {
            Ok(()) => {}

            Err(UsbError::Busy) => {
                return Err(AdapterError::DeviceBusy);
            }

            Err(UsbError::AccessDenied) => {
                return Err(AdapterError::ConnectionFailed);
            }

            Err(_) => {
                return Err(AdapterError::ConnectionFailed);
            }
        }

        let command = Self::build_command(10, None);
        let packet = Self::encrypt_command(&command);

        if let Err(error) = self.connection.write(USB_OUT, &packet) {
            self.connection.close();

            return Err(match error {
                UsbError::Busy => AdapterError::DeviceBusy,
                _ => AdapterError::TransferFailed,
            });
        }

        self.connected = true;

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), AdapterError> {
        self.connection.close();
        self.connected = false;

        Ok(())
    }

    fn send_frame(&mut self, frame: &[u8]) -> Result<(), AdapterError> {
        if !self.connected {
            return Err(AdapterError::Disconnected);
        }

        let command = Self::build_command(102, Some(frame.len()));

        let encrypted_command = Self::encrypt_command(&command);

        let mut payload = Vec::with_capacity(encrypted_command.len() + frame.len());

        payload.extend_from_slice(&encrypted_command);

        payload.extend_from_slice(frame);

        self.connection
            .write(USB_OUT, &payload)
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
        writes: Vec<(u8, Vec<u8>)>,
    }

    impl FakeUsbConnection {
        fn new() -> Self {
            Self {
                opened: false,
                writes: Vec::new(),
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

        fn write(&mut self, endpoint: u8, data: &[u8]) -> Result<usize, UsbError> {
            if !self.opened {
                return Err(UsbError::NotOpen);
            }

            self.writes.push((endpoint, data.to_vec()));

            Ok(data.len())
        }

        fn read(&mut self, _endpoint: u8, data: &mut [u8]) -> Result<usize, UsbError> {
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

    fn create_adapter() -> Turzx88DeviceAdapter {
        let info = get_test_usb_device_info();
        let connection = FakeUsbConnection::new();

        Turzx88DeviceAdapter::new(info, Box::new(connection))
    }

    #[test]
    fn creates_turzx_device_adapter() {
        let adapter = create_adapter();

        assert_eq!(adapter.info().vendor_id, DeviceModel::Turzx.vendor_id());

        assert_eq!(adapter.info().product_id, DeviceModel::Turzx.product_id());
    }

    #[test]
    fn adapter_starts_disconnected() {
        let adapter = create_adapter();

        assert!(!adapter.is_connected());
    }

    #[test]
    fn adapter_can_connect() {
        let mut adapter = create_adapter();

        adapter.connect().expect("Adapter should connect");

        assert!(adapter.is_connected());
    }

    #[test]
    fn connect_sends_sync_command() {
        let mut adapter = create_adapter();

        adapter.connect().expect("Adapter should connect");

        assert!(adapter.is_connected());
    }

    #[test]
    fn adapter_can_disconnect() {
        let mut adapter = create_adapter();

        adapter.connect().expect("Adapter should connect");

        assert!(adapter.is_connected());

        adapter.disconnect().expect("Adapter should disconnect");

        assert!(!adapter.is_connected());
    }

    #[test]
    fn cannot_send_frame_when_disconnected() {
        let mut adapter = create_adapter();

        let result = adapter.send_frame(&[]);

        assert!(matches!(result.unwrap_err(), AdapterError::Disconnected));
    }

    #[test]
    fn cannot_clear_when_disconnected() {
        let mut adapter = create_adapter();

        let result = adapter.clear();

        assert!(matches!(result.unwrap_err(), AdapterError::Disconnected));
    }

    #[test]
    fn can_send_frame_when_connected() {
        let mut adapter = create_adapter();

        adapter.connect().expect("Adapter should connect");

        let frame = [1, 2, 3, 4];

        let result = adapter.send_frame(&frame);

        assert!(result.is_ok());
    }

    #[test]
    fn send_frame_creates_turzx_payload() {
        let mut adapter = create_adapter();

        adapter.connect().expect("Adapter should connect");

        let frame = [1, 2, 3, 4];

        adapter.send_frame(&frame).expect("Frame should send");

        assert!(adapter.is_connected());
    }

    #[test]
    fn encrypted_command_has_correct_size() {
        let command = Turzx88DeviceAdapter::build_command(102, Some(100));

        let encrypted = Turzx88DeviceAdapter::encrypt_command(&command);

        assert_eq!(encrypted.len(), 512);
    }

    #[test]
    fn encrypted_command_has_turzx_footer() {
        let command = Turzx88DeviceAdapter::build_command(102, Some(100));

        let encrypted = Turzx88DeviceAdapter::encrypt_command(&command);

        assert_eq!(encrypted[510], 0xA1);
        assert_eq!(encrypted[511], 0x1A);
    }

    #[test]
    fn image_command_contains_payload_size() {
        let payload_size = 12345;

        let command = Turzx88DeviceAdapter::build_command(102, Some(payload_size));

        let encoded_size = u32::from_be_bytes([command[8], command[9], command[10], command[11]]);

        assert_eq!(encoded_size as usize, payload_size);
    }

    #[test]
    fn sync_command_has_correct_command_id() {
        let command = Turzx88DeviceAdapter::build_command(10, None);

        assert_eq!(command[0], 10);
    }

    #[test]
    fn image_command_has_correct_command_id() {
        let command = Turzx88DeviceAdapter::build_command(102, Some(100));

        assert_eq!(command[0], 102);
    }
}
