use super::*;

mod data;

pub struct Diffuse {
    buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    data: data::Data,
}

impl Diffuse {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        use bytemuck::Zeroable;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(buffer_label::<Self>().as_str()),
            size: std::mem::size_of::<data::Data>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group_layout =
            &device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(bind_group_layout_label::<Self>().as_str()),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(bind_group_label::<Self>().as_str()),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            layout: bind_group_layout,
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/diffuse.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(render_pipeline_layout_label::<Self>().as_str()),
                bind_group_layouts: &[bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(render_pipeline_label::<Self>().as_str()),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[model::vertex::Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            buffer,
            render_pipeline,
            bind_group,
            data: data::Data::zeroed(),
        }
    }
}

impl RenderPass for Diffuse {
    type ExecDescriptor<'a> = (
        &'a mut wgpu::Device,
        &'a mut wgpu::Queue,
        &'a mut render::model_cache::ModelCache,
        &'a mut ecs::ECS,
        &'a scene::Scene,
        &'a wgpu::TextureView,
    );
    fn exec<'a>(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        (device, queue, model_cache, ecs, scene, view): Self::ExecDescriptor<'a>,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
                resolve_target: None,
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        self.data.camera = *scene.camera().transform();

        for (id, (model, transform)) in ecs.query::<(
            &ecs::component::model::Model,
            &mut ecs::component::transform::Transform,
        )>() {
            self.data.model = *transform.transform();
            let model = model_cache.get(model.name());
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.data]));
            for mesh in model.meshes() {
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
                /*render_pass
                    .set_index_buffer(mesh.index_buffer().slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..mesh.index_count(), 0, 0..1);*/
            }
        }
    }
}
