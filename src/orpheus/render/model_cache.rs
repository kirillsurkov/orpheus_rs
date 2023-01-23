use super::*;

use std::collections::HashMap;

pub struct ModelCache {
    storage: HashMap<String, model::Model>,
}

impl ModelCache {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn prepare(&mut self){}/*, device: &mut wgpu::Device, name: &str) {
        if !self.storage.contains_key(name) {
            self.storage.insert(name.into(), model::loaders::gltf::load(name, device));
        }
    }*/

    pub fn get(&mut self, name: &str) -> &model::Model {
        self.storage.get(name).unwrap()
    }
}
