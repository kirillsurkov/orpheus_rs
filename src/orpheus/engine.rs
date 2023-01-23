use super::*;

struct Monkey {
    entity: ecs::entity_handle::EntityHandle,
}

impl Monkey {
    fn new(ecs: &mut ecs::ECS) -> Self {
        use cgmath::SquareMatrix;

        Self {
            entity: ecs.spawn((
                ecs::component::model::Model::new("res/models/monkey.glb"),
                ecs::component::transform::Transform::new(cgmath::Matrix4::identity().into()),
                ecs::component::monkey::Monkey::new(),
            )),
        }
    }
}

impl ecs::entity::Entity for Monkey {
    fn entity(&self) -> &ecs::entity_handle::EntityHandle {
        &self.entity
    }
}

pub struct Engine {
    scene: scene::Scene,
}

impl Engine {
    pub fn new(render: &mut render::Render, ecs: &mut ecs::ECS) -> Self {
        let mut scene = scene::Scene::new();
        scene.camera_mut().look_at([0.0, 0.0, 5.0], [0.0, 0.0, 0.0]);

        scene.add_entity(Monkey::new(ecs));

        Self { scene }
    }

    pub fn update(&mut self, delta: f32, render: &mut render::Render, ecs: &mut ecs::ECS) {
        self.scene
            .camera_mut()
            .set_aspect(render.width() as f32 / render.height() as f32);
        self.scene.update(delta);

        for (id, (model, transform)) in ecs.query::<(
            &ecs::component::model::Model,
            &mut ecs::component::transform::Transform,
        )>() {
            let mat = cgmath::Matrix4::from(*transform.transform());
            let rotation = cgmath::Matrix4::from_angle_y(cgmath::Deg(delta * 30.0));
            *transform.transform_mut() = (mat * rotation).into();
        }
    }

    pub fn scene(&self) -> &scene::Scene {
        &self.scene
    }
}
