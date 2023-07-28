pub struct Mino {
    pub shape: [[u32; 4]; 4],
}

impl Mino {
    pub fn new(shape: [[u32; 4]; 4]) -> Self {
        Self { shape }
    }

    fn rotate(&self, dir: i32) {}
}
