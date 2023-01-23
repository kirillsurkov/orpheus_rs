pub struct EntityHandle {
    id: u32,
}

impl EntityHandle {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
