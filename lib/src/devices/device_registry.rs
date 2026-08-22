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

    // pub fn add(
    //     &mut self,
    //     id: DeviceId,
    //     session: DeviceSession,
    // ) {
    //     self.devices.insert(id, session);
    // }

    // pub fn remove(
    //     &mut self,
    //     id: &DeviceId,
    // ) -> Option<DeviceSession> {
    //     self.devices.remove(id)
    // }

    // pub fn get(
    //     &self,
    //     id: &DeviceId,
    // ) -> Option<&DeviceSession> {
    //     self.devices.get(id)
    // }

    // pub fn get_mut(
    //     &mut self,
    //     id: &DeviceId,
    // ) -> Option<&mut DeviceSession> {
    //     self.devices.get_mut(id)
    // }

    // pub fn contains(
    //     &self,
    //     id: &DeviceId,
    // ) -> bool {
    //     self.devices.contains_key(id)
    // }

    // pub fn list(
    //     &self,
    // ) -> impl Iterator<Item = &DeviceSession> {
    //     self.devices.values()
    // }

    // pub fn len(&self) -> usize {
    //     self.devices.len()
    // }

    // pub fn is_empty(&self) -> bool {
    //     self.devices.is_empty()
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn add_new_registry
}

// pub fn add(
// pub fn remove(
// pub fn get(
// pub fn get_mut(
// pub fn contains(
// pub fn list(
// pub fn len(&self) -> usize {
// pub fn is_empty(&self) -> bool {