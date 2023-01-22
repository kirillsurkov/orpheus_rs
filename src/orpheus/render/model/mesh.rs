pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl Mesh {
    pub fn new(vertex_buffer: wgpu::Buffer, index_buffer: wgpu::Buffer, index_count: u32) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }
}
