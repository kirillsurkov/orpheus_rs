pub mod vertex;
pub mod loaders;
mod mesh;

pub struct Model {
    meshes: Vec<mesh::Mesh>,
    transform: [[f32; 4]; 4],
}

impl Model {
    pub fn meshes(&self) -> &Vec<mesh::Mesh> {
        &self.meshes
    }
}

impl Model {
    pub fn transform(&self) -> &[[f32; 4]; 4] {
        &self.transform
    }
}