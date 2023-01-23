use super::*;

pub mod camera;
mod misc;
pub mod model;
mod model_cache;
mod render_pass;

struct RenderPasses {
    clear: render_pass::clear::Clear,
    diffuse: render_pass::diffuse::Diffuse,
}

pub struct Render {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_passes: RenderPasses,
    model_cache: model_cache::ModelCache,
}

impl Render {
    pub async fn new(window: &window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window.raw_window()) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        let formats = surface.get_supported_formats(&adapter);
        println!("{:?}", formats);
        let config = wgpu::SurfaceConfiguration {
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            format: *formats
                .iter()
                .find(|e| e.describe().srgb)
                .unwrap_or(&formats[0]),
            width: 1,
            height: 1,
            present_mode: wgpu::PresentMode::Fifo,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        };

        let mut render_passes = RenderPasses {
            clear: render_pass::clear::Clear::new(),
            diffuse: render_pass::diffuse::Diffuse::new(&device, config.format),
        };
        render_passes.clear.set_color(0.2, 0.1, 0.2, 1.0);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            render_passes,
            model_cache: model_cache::ModelCache::new(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(
        &mut self,
        ecs: &mut ecs::ECS,
        scene: &scene::Scene,
    ) -> Result<(), wgpu::SurfaceError> {
        use render_pass::RenderPass;

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command encoder"),
            });

        self.render_passes.clear.exec(&mut encoder, &view);
        self.render_passes.diffuse.exec(
            &mut encoder,
            (
                &mut self.device,
                &mut self.queue,
                &mut self.model_cache,
                ecs,
                scene,
                &view,
            ),
        );

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn device(&mut self) -> &mut wgpu::Device {
        &mut self.device
    }

    pub fn width(&self) -> u32 {
        self.config.width
    }

    pub fn height(&self) -> u32 {
        self.config.height
    }
}
