use winit::event::Event;
use winit::event_loop::ControlFlow;

pub struct WindowData {
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    pub window: winit::window::Window,
    pub exit: bool,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl WindowData {
    pub fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        let size = window.inner_size();

        Self {
            event_loop: Some(event_loop),
            window,
            exit: false,
            size
        }
    }
}

pub trait WindowBase {
    fn run(self);
    fn process_events(&mut self, event: Event<()>, control_flow: &mut ControlFlow);
    fn input(&self, event: &winit::event::WindowEvent) -> bool;
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
}

pub trait Window {
    fn get_data(&mut self) -> &mut WindowData;
    fn on_update(&mut self);
    fn on_redraw(&mut self);
    fn on_resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
}

impl<T: 'static + Window> WindowBase for T {
    fn run(mut self) {
        let event_loop = self.get_data().event_loop.take().unwrap();
        event_loop.run(move |event, _, control_flow| self.process_events(event, control_flow));
    }

    fn process_events(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        if self.get_data().exit {
            *control_flow = winit::event_loop::ControlFlow::Exit;
        }
        match event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.get_data().window.id() => {
                if !self.input(event) {
                    match event {
                        winit::event::WindowEvent::CloseRequested
                        | winit::event::WindowEvent::KeyboardInput {
                            input:
                                winit::event::KeyboardInput {
                                    state: winit::event::ElementState::Pressed,
                                    virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => {
                            self.get_data().exit = true;
                        }
                        winit::event::WindowEvent::Resized(physical_size) => {
                            self.resize(*physical_size);
                        }
                        winit::event::WindowEvent::ScaleFactorChanged {
                            new_inner_size, ..
                        } => {
                            self.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            winit::event::Event::RedrawRequested(window_id)
                if window_id == self.get_data().window.id() =>
            {
                self.on_update();
                self.on_redraw();
            }
            winit::event::Event::MainEventsCleared => {
                self.get_data().window.request_redraw();
            }
            _ => {}
        };
    }

    fn input(&self, event: &winit::event::WindowEvent) -> bool {
        false
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.get_data().size = new_size;
        self.on_resize(new_size);
    }
}