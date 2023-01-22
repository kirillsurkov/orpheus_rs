use super::*;

pub trait Game {
    fn new(engine: &mut engine::Engine) -> Self;
    fn update(&mut self, delta: f32);
}