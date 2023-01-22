pub mod event;

pub struct Window {
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    window: winit::window::Window,
    exit: bool,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        Self {
            event_loop: Some(event_loop),
            window,
            exit: false,
        }
    }

    pub fn raw_window(
        &self,
    ) -> &(impl raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle)
    {
        &self.window
    }

    pub fn run<T: 'static + FnMut(&mut Self, event::Event)>(mut self, mut callback: T) {
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| match event {
                winit::event::Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        callback(&mut self, event::Event::CloseRequested())
                    }
                    winit::event::WindowEvent::Resized(physical_size) => {
                        callback(
                            &mut self,
                            event::Event::Resize(physical_size.width, physical_size.height),
                        );
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        callback(
                            &mut self,
                            event::Event::Resize(new_inner_size.width, new_inner_size.height),
                        );
                    }
                    _ => (),
                },
                winit::event::Event::RedrawRequested(window_id)
                    if window_id == self.window.id() =>
                {
                    callback(&mut self, event::Event::Redraw())
                }
                winit::event::Event::MainEventsCleared => {
                    if self.exit {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    } else {
                        self.window.request_redraw();
                    }
                }
                _ => (),
            });
    }

    pub fn width(&self) -> u32 {
        self.window.inner_size().width
    }

    pub fn height(&self) -> u32 {
        self.window.inner_size().height
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.window
            .set_inner_size(winit::dpi::PhysicalSize::new(width, height));
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
