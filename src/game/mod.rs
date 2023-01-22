use super::*;

pub struct App {}

impl orpheus::game::Game for App {
    fn new(engine: &mut orpheus::engine::Engine) -> Self {
        Self {}
    }

    fn update(&mut self, delta: f32) {

    }
}