mod ecs;
pub mod engine;
pub mod game;
mod render;
mod scene;
mod stopwatch;
mod window;

pub fn run<T: 'static + game::Game>() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let window = window::Window::new();
    let mut render = rt.block_on(render::Render::new(&window));
    let mut stopwatch = stopwatch::Stopwatch::new();
    let mut engine = engine::Engine::new(&mut render);
    let mut game = T::new(&mut engine);
    window.run(move |window, event| match event {
        window::event::Event::Redraw() => {
            let delta = stopwatch.split();
            game.update(delta);
            engine.update(delta, &mut render);
            match render.render(engine.scene()) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => window.resize(window.width(), window.height()),
                Err(wgpu::SurfaceError::OutOfMemory) => window.exit(),
                Err(e) => eprintln!("{:?}", e),
            }
        }
        window::event::Event::Resize(width, height) => {
            render.resize(width, height);
        }
        window::event::Event::CloseRequested() => window.exit(),
    });
}
