pub struct DeviceAdapterFactory;
use crate::devices::device_model::DeviceModel;

impl DeviceFactory {
    pub fn new(registry: DeviceRegistry) -> Self {
        Self {
            registry: registry
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_turzx_adapter_for_supported_device() {
        let turzx_vid = DeviceModel::Turzx.vendor_id();
        let turzx_pid = DeviceModel::Turzx.product_id();
        let factory = DeviceFactory;
        let adapter = factory.create(
            turzx_vid,
            turzx_pid,
        );
        assert!(adapter.is_ok());

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
        let factory = DefaultDeviceFactory;

        let adapter = factory.create_device(
            0xFFFF,
            0xFFFF,
        );

        assert!(adapter.is_none());
    }

}