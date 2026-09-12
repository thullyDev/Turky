#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceModel {
    Turzx,
}

impl DeviceModel {
    pub fn vendor_id(&self) -> u16 {
        match self {
            Self::Turzx => 0x1CBE,
        }
    }

    pub fn product_id(&self) -> u16 {
        match self {
            Self::Turzx => 0x0088,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turzx_has_correct_vendor_id() {
        assert_eq!(DeviceModel::Turzx.vendor_id(), 0x1CBE);
    }

    #[test]
    fn turzx_has_correct_product_id() {
        assert_eq!(DeviceModel::Turzx.product_id(), 0x0088);
    }
}
