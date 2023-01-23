use hecs::QueryMut;

pub mod component;
pub mod entity;
pub mod entity_handle;

pub struct ECS {
    world: hecs::World,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            world: hecs::World::new(),
        }
    }

    pub fn spawn(&mut self, components: impl hecs::DynamicBundle) -> entity_handle::EntityHandle {
        entity_handle::EntityHandle::new(self.world.spawn(components).id())
    }

    pub fn query<Q: hecs::Query>(&mut self) -> QueryMut<'_, Q> {
        self.world.query_mut::<Q>()
    }
}