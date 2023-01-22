#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Data {
    pub camera: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
}