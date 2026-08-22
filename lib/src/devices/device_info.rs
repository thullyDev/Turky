use super::device_id::DeviceId;

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: DeviceId,
    pub name: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub width: u32,
    pub height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_device_info() {
        let info = DeviceInfo {
            id: DeviceId("turing-001".to_string()),
            name: "Turing Smart Screen 2.1".to_string(),
            vendor_id: 0x1CBE,
            product_id: 0x0088,
            width: 320,
            height: 480,
        };

        assert_eq!(info.name, "Turing Smart Screen 2.1");
        assert_eq!(info.vendor_id, 0x1CBE);
        assert_eq!(info.product_id, 0x0088);
        assert_eq!(info.width, 320);
        assert_eq!(info.height, 480);
    }
}