use std::collections::HashMap;

use super::{
    device_id::DeviceId,
    device_session::DeviceSession,
};

pub struct DeviceRegistry {
    devices: HashMap<DeviceId, DeviceSession>,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        id: DeviceId,
        session: DeviceSession,
    ) {
        self.devices.insert(id, session);
    }

    pub fn remove(
        &mut self,
        id: &DeviceId,
    ) -> Option<DeviceSession> {
        self.devices.remove(id)
    }

    pub fn get(
        &self,
        id: &DeviceId,
    ) -> Option<&DeviceSession> {
        self.devices.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &DeviceId,
    ) -> Option<&mut DeviceSession> {
        self.devices.get_mut(id)
    }

    pub fn contains(
        &self,
        id: &DeviceId,
    ) -> bool {
        self.devices.contains_key(id)
    }

    pub fn list(
        &self,
    ) -> impl Iterator<Item = &DeviceSession> {
        self.devices.values()
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    pub fn first_mut(&mut self) -> Option<&mut DeviceSession> {
        self.devices.values_mut().next()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::test_utils::{
        FakeDisplayAdapter,
        get_test_device_info,
    };

    #[test]
    fn add_new_registry_device() {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: false,
        };

        let session = DeviceSession::new(
            Box::new(adapter)
        );

        let device_id = info.id.clone();

        registry.add(
            device_id.clone(),
            session,
        );

        assert_eq!(registry.len(), 1);
        assert!(registry.contains(&device_id));
    }

    #[test]
    fn remove_registry_device() {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: false,
        };

        let session = DeviceSession::new(
            Box::new(adapter)
        );

        let device_id = info.id.clone();

        registry.add(
            device_id.clone(),
            session,
        );

        assert_eq!(registry.len(), 1);
        assert!(registry.contains(&device_id));

        
        registry.remove(&device_id);
        

        assert_eq!(registry.len(), 0);
        assert!(!registry.contains(&device_id));
    }
    
    #[test]
    fn get_registry_device() {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: false,
        };

        let session = DeviceSession::new(
            Box::new(adapter)
        );

        let device_id = info.id.clone();

        registry.add(
            device_id.clone(),
            session,
        );

        assert_eq!(registry.len(), 1);
        assert!(registry.contains(&device_id));


        let device = registry.get(&device_id);

        assert!(device.is_some());

        let device = device.unwrap();

        assert_eq!(device.info.id, info.id);
        assert_eq!(device.info.name, info.name);
        assert_eq!(device.info.vendor_id, info.vendor_id);
        assert_eq!(device.info.product_id, info.product_id);
        assert_eq!(device.info.width, info.width);
        assert_eq!(device.info.height, info.height);

        
    }

    #[test]
    fn get_mut_registry_device() {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: false,
        };

        let session = DeviceSession::new(
            Box::new(adapter)
        );

        let device_id = info.id.clone();

        registry.add(
            device_id.clone(),
            session,
        );

        assert_eq!(registry.len(), 1);
        assert!(registry.contains(&device_id));


        let device = registry
            .get_mut(&device_id)
            .expect("device should exist");

        device
            .adapter
            .connect()
            .expect("adapter should connect");

        assert!(device.adapter.is_connected());
    }

    

    #[test]
    fn contains_registry_device() {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: false,
        };

        let session = DeviceSession::new(
            Box::new(adapter)
        );

        let device_id = info.id.clone();

        registry.add(
            device_id.clone(),
            session,
        );

        assert!(registry.contains(&device_id));
    }

    

    #[test]
    fn list_registry_devices() {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: false,
        };

        let session = DeviceSession::new(
            Box::new(adapter)
        );

        let device_id = info.id.clone();

        registry.add(
            device_id.clone(),
            session,
        );

        let devices: Vec<_> = registry.list().collect();

        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].info.id, info.id);
        assert_eq!(devices[0].info.name, info.name);
    }
    

    #[test]
    fn len_registry_devices() {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: false,
        };

        let session = DeviceSession::new(
            Box::new(adapter)
        );

        let device_id = info.id.clone();

        registry.add(
            device_id.clone(),
            session,
        );

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn is_empty_registry_devices() {
        let registry = DeviceRegistry::new();

        assert_eq!(registry.is_empty(), true);
    }

    #[test]
    fn is_not_empty_registry_devices() {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: false,
        };

        let session = DeviceSession::new(
            Box::new(adapter)
        );

        let device_id = info.id.clone();

        registry.add(
            device_id.clone(),
            session,
        );

        assert_eq!(registry.is_empty(), false);
    }

    #[test]
    fn gets_first_device_mut() {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: false,
        };

        let session = DeviceSession::new(
            Box::new(adapter)
        );

        let device_id = info.id.clone();

        registry.add(
            device_id,
            session,
        );

        let device = registry
            .first_mut()
            .expect("device should exist");

        assert_eq!(
            device.info.id,
            info.id
        );

        assert_eq!(
            device.info.name,
            info.name
        );
    }
}
