use specs::prelude::*;

mod window;
mod render;

struct App {
    window_data: window::WindowData,
    render: render::Render,
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}
impl App {
    async fn new() -> Self {
        let window_data = window::WindowData::new();
        let render = render::Render::new(&window_data.window).await;

        let mut world = World::new();
        let mut dispatcher = DispatcherBuilder::new().build();
        dispatcher.setup(&mut world);

        Self {
            window_data,
            render,
            world,
            dispatcher
        }
    }
}
impl window::Window for App {
    fn get_data(&mut self) -> &mut window::WindowData {
        &mut self.window_data
    }

    fn on_update(&mut self) {
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }

    fn on_resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        println!("Resized {} {}", new_size.width, new_size.height);
        self.render.resize(new_size);
    }

    fn on_redraw(&mut self) {
        use crate::window::WindowBase;
        match self.render.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => self.resize(self.window_data.size),
            Err(wgpu::SurfaceError::OutOfMemory) => self.window_data.exit = true,
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

pub async fn run() {
    use crate::window::WindowBase;
    let app = App::new().await;
    app.run();
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(run());
}
