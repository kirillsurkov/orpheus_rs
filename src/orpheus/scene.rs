use super::*;

pub struct Scene {
    camera: render::camera::Camera,
    entities: Vec<Box<dyn ecs::entity::Entity>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: render::camera::Camera::new(),
            entities: Vec::new(),
        }
    }

    pub fn add_entity<T: 'static + ecs::entity::Entity>(&mut self, entity: T) {
        self.entities.push(Box::new(entity));
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
