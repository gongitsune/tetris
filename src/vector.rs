#[derive(Debug, Default, Clone, Copy)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

impl Int2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
