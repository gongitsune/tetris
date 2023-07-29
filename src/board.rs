use crate::{mino::Mino, vector::Int2, BOARD_COL, BOARD_ROW};
use anyhow::{Ok, Result};
use std::io::Write;
use termion::{clear, cursor};

pub struct Board {
    pub board: [[u32; BOARD_COL]; BOARD_ROW],
    active_mino: Option<Mino>,
    pub mino_x_dir: i32,
}

impl Board {
    pub fn new() -> Result<Self> {
        let mut board = [[1; BOARD_COL]; BOARD_ROW];

        // Fill
        for y in 0..BOARD_ROW - 1 {
            for x in 1..BOARD_COL - 1 {
                board[y][x] = 0;
            }
        }

        Ok(Self {
            board,
            active_mino: None,
            mino_x_dir: 0,
        })
    }

    pub fn put_mino(&mut self, mut mino: Mino) {
        mino.pos = Int2::new(BOARD_COL as i32 / 2, 0);
        self.active_mino = Some(mino);
    }

    pub fn update(&mut self) {
        if let Some(ref mut mino) = self.active_mino {
            {
                mino.pos.y += 1;
                for y in 0..BOARD_ROW {
                    for x in 0..BOARD_COL {
                        let mino_pixel = mino.get_pixel(Int2::new(x as i32, y as i32));
                        if mino_pixel != 0 && self.board[y][x] != 0 {
                            mino.pos.y -= 1;
                            mino.apply(&mut self.board);
                            self.active_mino = None;
                            return;
                        }
                    }
                }
            }

            {
                mino.pos.x += self.mino_x_dir;
                for y in 0..BOARD_ROW {
                    for x in 0..BOARD_COL {
                        let mino_pixel = mino.get_pixel(Int2::new(x as i32, y as i32));
                        if mino_pixel != 0 && self.board[y][x] != 0 {
                            mino.pos.x -= self.mino_x_dir;
                            return;
                        }
                    }
                }
                self.mino_x_dir = 0;
            }
        }
    }

    pub fn draw<W: Write>(&mut self, out: &mut W) -> Result<()> {
        write!(out, "{}", clear::All)?;
        write!(out, "{}", cursor::Goto(1, 1))?;

        for (y, row) in self.board.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let pixel = if let Some(ref mino) = self.active_mino {
                    let pixel = mino.get_pixel(Int2::new(x as i32, y as i32));
                    if pixel == 0 {
                        *col
                    } else {
                        pixel
                    }
                } else {
                    *col
                };
                let pixel = if pixel == 0 { ' ' } else { '#' };

                write!(out, "{}", pixel)?;
            }
            write!(out, "\r\n")?;
        }

        out.flush()?;

        Ok(())
    }
}
