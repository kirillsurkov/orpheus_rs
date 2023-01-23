use super::*;

pub struct Clear {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl Clear {
    pub fn new() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub fn set_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a;
    }
}

impl RenderPass for Clear {
    type ExecDescriptor<'a> = &'a wgpu::TextureView;

    fn exec<'a>(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: Self::ExecDescriptor<'a>,
    ) {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: self.r,
                        g: self.g,
                        b: self.b,
                        a: self.a,
                    }),
                    store: true,
                },
                resolve_target: None,
            })],
            depth_stencil_attachment: None,
        });
    }
}
