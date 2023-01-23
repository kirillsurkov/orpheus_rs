use super::*;

pub fn load(file_name: &str, device: &wgpu::Device) -> Model {
    use cgmath::SquareMatrix;
    use wgpu::util::DeviceExt;

    let mut result = Model {
        meshes: Vec::new(),
        transform: cgmath::Matrix4::identity().into(),
    };

    let mut gltf = ::gltf::Gltf::open(file_name).unwrap();
    let mut buffer_data = Vec::with_capacity(gltf.buffers().len());
    let mut blob_index = None;
    for buffer in gltf.buffers() {
        buffer_data.push(match buffer.source() {
            ::gltf::buffer::Source::Bin => {
                blob_index = Some(buffer.index());
                Vec::new()
            }
            ::gltf::buffer::Source::Uri(_) => todo!(),
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
                        vertices.push(vertex::Vertex::default());
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

                result.meshes.push(mesh::Mesh::new(
                    vertex_buffer,
                    index_buffer,
                    indices.len() as u32,
                ))
            }
        }
    }

    result
}