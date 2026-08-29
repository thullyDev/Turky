#[derive(Clone, Debug)]
pub struct UsbDeviceInfo {
    pub vendor_id: u16,
    pub product_id: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_usb_device_info() {
        let device = UsbDeviceInfo {
            vendor_id: 0x1CBE,
            product_id: 0x0088,
        };

        assert_eq!(device.vendor_id, 0x1CBE);
        assert_eq!(device.product_id, 0x0088);
    }
}