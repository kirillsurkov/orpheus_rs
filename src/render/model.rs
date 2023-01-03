use super::misc;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 4],
    pub uv: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32x2];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl Mesh {
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

pub struct Model {
    meshes: Vec<Mesh>,
    transform: misc::UniformBuffer<misc::Transform>,
}

impl Model {
    pub fn from_gltf<P: AsRef<std::path::Path>>(
        device: &wgpu::Device,
        file_name: P,
    ) -> Result<Self, ()> {
        use cgmath::SquareMatrix;
        use wgpu::util::DeviceExt;

        let transform = misc::UniformBuffer::new(
            device,
            format!("model({})", file_name.as_ref().to_str().unwrap()).as_str(),
            misc::Transform {
                matrix: cgmath::Matrix4::identity().into(),
            },
        );

        let mut result = Self {
            meshes: Vec::new(),
            transform,
        };

        let mut gltf = gltf::Gltf::open(file_name).unwrap();
        let mut buffer_data = Vec::with_capacity(gltf.buffers().len());
        let mut blob_index = None;
        for buffer in gltf.buffers() {
            buffer_data.push(match buffer.source() {
                gltf::buffer::Source::Bin => {
                    blob_index = Some(buffer.index());
                    Vec::new()
                }
                gltf::buffer::Source::Uri(_) => todo!(),
            })
        }
        if let Some(blob_index) = blob_index {
            buffer_data[blob_index] = gltf.blob.take().unwrap();
        }

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                let mesh = match node.mesh() {
                    Some(mesh) => mesh,
                    None => continue,
                };
                for primitive in mesh.primitives() {
                    let mut vertices = Vec::new();
                    let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));
                    if let Some(positions) = reader.read_positions() {
                        for (i, pos) in positions.enumerate() {
                            vertices.push(Vertex::default());
                            vertices[i].pos = pos;
                        }
                    }
                    if let Some(normals) = reader.read_normals() {
                        for (i, normal) in normals.enumerate() {
                            vertices[i].normal = normal;
                        }
                    }
                    if let Some(tangents) = reader.read_tangents() {
                        for (i, tangent) in tangents.enumerate() {
                            vertices[i].tangent = tangent;
                        }
                    }
                    if let Some(uvs) = reader.read_tex_coords(0) {
                        for (i, uv) in uvs.into_f32().enumerate() {
                            vertices[i].uv = uv;
                        }
                    }

                    let mut indices = Vec::new();
                    match reader.read_indices() {
                        Some(_indices) => {
                            indices = _indices.into_u32().collect::<Vec<u32>>();
                        }
                        None => {}
                    }

                    let vertex_buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Vertex Buffer"),
                            contents: bytemuck::cast_slice(&vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                    let index_buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Index Buffer"),
                            contents: bytemuck::cast_slice(&indices),
                            usage: wgpu::BufferUsages::INDEX,
                        });

                    result.meshes.push(Mesh {
                        index_buffer,
                        vertex_buffer,
                        index_count: indices.len() as u32,
                    })
                }
            }
        }

        Ok(result)
    }

    pub fn meshes(&self) -> &Vec<Mesh> {
        &self.meshes
    }

    pub fn transform(&self) -> &misc::UniformBuffer<misc::Transform> {
        &self.transform
    }

    pub fn update(&mut self) {
        let mat = cgmath::Matrix4::from(self.transform.data().matrix);
        let rotation = cgmath::Matrix4::from_angle_y(cgmath::Deg(90.0 / 60.0));
        self.transform.data().matrix = (mat * rotation).into();
    }
}
