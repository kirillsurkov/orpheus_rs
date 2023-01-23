use super::*;

pub struct Transform {
    transform: [[f32; 4]; 4],
}

impl Transform {
    pub fn new(transform: [[f32; 4]; 4]) -> Self {
        Self { transform }
    }

    pub fn transform(&self) -> &[[f32; 4]; 4] {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut [[f32; 4]; 4] {
        &mut self.transform
    }
}

impl Component for Transform {
}