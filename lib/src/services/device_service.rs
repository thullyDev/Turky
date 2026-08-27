use crate::factories::device_factory::DeviceFactory;
use crate::devices::device_registry::DeviceRegistry;

pub struct DeviceService {
    factory: DeviceFactory,
    registry: DeviceRegistry,
}

impl DeviceService {
    pub fn new(
        registry: DeviceRegistry,
        factory: DeviceFactory,
    ) -> Self {
        // TODO: also add USB adaptor
        Self {
            factory,
            registry,
        }
    }

    pub fn discover_devices(
        &mut self,
    ) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_device_service() {
        let registry = DeviceRegistry::new();
        let factory = DeviceFactory::new();

        let _service = DeviceService::new(
            registry,
            factory,
        );
    }

    #[test]
    fn discovers_connected_devices() {
        let factory = DeviceFactory::new();
        let registry = DeviceRegistry::new();

        let mut service = DeviceService::new(
            registry,
            factory,
        );

        service.discover_devices();
        assert_eq!(service.registry.len(), 1);
    }
}