#[derive(Debug)]
pub enum UsbError {
    DeviceNotFound,
    AccessDenied,
    Busy,
    OpenFailed,
    NotOpen,
    TransferFailed,
}
