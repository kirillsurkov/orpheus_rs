use super::*;

pub mod clear;
pub mod diffuse;

pub trait RenderPass {
    type ExecDescriptor<'a>;
    fn exec<'a>(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        descriptor: Self::ExecDescriptor<'a>,
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
