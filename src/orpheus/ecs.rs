use specs::prelude::*;

pub struct ECS {
    world: World,
    //dispatcher: Dispatcher,
}

impl ECS {
    pub fn new() -> Self {
        let world = World::new();
        //let Dispatcher = DispatcherBuilder::
        Self {
            world,
        }
    }
}