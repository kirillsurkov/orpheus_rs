#[derive(Debug)]
pub enum Event {
    Redraw(),
    Resize(u32, u32),
    CloseRequested(),
}
