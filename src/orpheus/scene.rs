use super::*;

pub struct Scene {
    camera: render::camera::Camera,
    models: Vec<render::model::Model>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: render::camera::Camera::new(),
            models: Vec::new(),
        }
    }

    pub fn models(&self) -> &Vec<render::model::Model> {
        &self.models
    }

    pub fn add_model(&mut self, render: &mut render::Render, file_name: &str) {
        self.models
            .push(render::model::loaders::gltf::load(file_name, render.device()).unwrap())
    }

    pub fn camera(&self) -> &render::camera::Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut render::camera::Camera {
        &mut self.camera
    }

    pub fn update(&mut self, delta: f32) {
        self.camera.update(delta);
    }
}
