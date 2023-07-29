use crate::{vector::Int2, BOARD_COL, BOARD_ROW};
use std::usize;

#[derive(Clone)]
pub struct Mino {
    pub shape: [[u32; 4]; 4],
    pub pos: Int2,
}

impl Mino {
    pub fn new(shape: [[u32; 4]; 4]) -> Self {
        Self {
            shape,
            pos: Int2::default(),
        }
    }

    fn rotate(&self, dir: i32) {}

    pub fn apply(&self, board: &mut [[u32; BOARD_COL]; BOARD_ROW]) {
        for y in 0..4 {
            for x in 0..4 {
                let pixel = self.shape[y as usize][x as usize];
                if pixel == 0 {
                    continue;
                }

                let (board_y, board_x) = (self.pos.y - 3 + y, self.pos.x + x);
                board[board_y as usize][board_x as usize] = pixel;
            }
        }
    }

    pub fn get_pixel(&self, pos: Int2) -> u32 {
        let (y, x) = (3 - self.pos.y + pos.y, pos.x - self.pos.x);

        if y > 3 || y < 0 || x > 3 || x < 0 {
            0
        } else {
            self.shape[y as usize][x as usize]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Int2;

    use super::Mino;

    #[test]
    fn get_pixel_test() {
        let mut mino = Mino::new([[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);

        mino.pos.x = 0;
        mino.pos.y = 3;

        assert_eq!(mino.get_pixel(Int2::new(0, 0)), 1);
    }
}
