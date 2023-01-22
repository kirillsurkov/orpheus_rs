use super::*;

pub mod clear;
pub mod diffuse;

pub trait RenderPass {
    fn exec(
        &mut self,
        scene: &scene::Scene,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        queue: &mut wgpu::Queue,
    );
}

fn buffer_label<T>() -> String {
    format!("{} buffer", std::any::type_name::<T>())
}

fn bind_group_layout_label<T>() -> String {
    format!("{} bind group layout", std::any::type_name::<T>())
}

fn bind_group_label<T>() -> String {
    format!("{} bind group", std::any::type_name::<T>())
}

fn render_pipeline_layout_label<T>() -> String {
    format!("{} render pipeline layout", std::any::type_name::<T>())
}

fn render_pipeline_label<T>() -> String {
    format!("{} render pipeline", std::any::type_name::<T>())
}
