use super::*;

pub struct Model {
    name: String,
}

impl Model {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Component for Model {}
