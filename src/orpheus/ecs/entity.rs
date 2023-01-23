use super::*;

pub trait Entity {
    fn entity(&self) -> &entity_handle::EntityHandle;
}