use crate::devices::device_model::DeviceModel::Turzx;
use crate::adapters::display_adapter::DisplayAdapter;
use crate::adapters::devices::turzx_device_adapter::TurzxDeviceAdapter;

pub struct DeviceFactory;

impl DeviceFactory {
    pub fn new() -> Self {
        Self
    }

    pub fn create_device(
        &mut self,
        vendor_id: u16,
        product_id: u16,
    ) -> Option<Box<dyn DisplayAdapter>> {
        if vendor_id == Turzx.vendor_id()
            && product_id == Turzx.product_id()
        {
            Some(Box::new(
                TurzxDeviceAdapter::new()
            ))
        } else {
            None        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_turzx_adapter_for_supported_device() {
        let turzx_vid = Turzx.vendor_id();
        let turzx_pid = Turzx.product_id();
        let mut factory = DeviceFactory::new();
        let adapter = factory.create_device(
            turzx_vid,
            turzx_pid,
        );

        let adapter = adapter.unwrap();

        assert_eq!(
            adapter.info().vendor_id,
            turzx_vid
        );

        assert_eq!(
            adapter.info().product_id,
            turzx_pid
        );
    }

    #[test]
    fn returns_none_for_unsupported_device() {
        let mut factory = DeviceFactory::new();
        let adapter = factory.create_device(
            0xFFFF,
            0xFFFF,
        );

        assert!(adapter.is_none());
    }

}