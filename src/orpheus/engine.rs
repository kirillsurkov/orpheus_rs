use super::*;

pub struct Engine {
    ecs: ecs::ECS,
    scene: scene::Scene,
}

impl Engine {
    pub fn new(render: &mut render::Render) -> Self {
        let ecs = ecs::ECS::new();
        let mut scene = scene::Scene::new();
        scene.add_model(render, "res/models/monkey.glb");
        scene.camera_mut().look_at([0.0, 0.0, 5.0], [0.0, 0.0, 0.0]);

        Self { ecs, scene }
    }

    pub fn update(&mut self, delta: f32, render: &mut render::Render) {
        self.scene
            .camera_mut()
            .set_aspect(render.width() as f32 / render.height() as f32);
        self.scene.update(delta);
    }

    pub fn scene(&self) -> &scene::Scene {
        &self.scene
    }
}
