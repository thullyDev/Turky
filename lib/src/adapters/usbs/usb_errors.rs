#[derive(Debug)]
pub enum UsbError {
    DeviceNotFound,
    NotOpen,
    TransferFailed,
}