use crate::{mino::Mino, vector::Int2, BOARD_COL, BOARD_ROW};
use anyhow::{Ok, Result};
use std::{char, io::Write};
use termion::cursor;

pub struct Board {
    pub board: [[u32; BOARD_COL]; BOARD_ROW],
    pub pre_rendered: [[char; BOARD_COL]; BOARD_ROW],
    active_mino: Option<Mino>,
    pub mino_x_dir: i32,
    pub rotate_dir: i32,
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
            pre_rendered: [[' '; BOARD_COL]; BOARD_ROW],
            active_mino: None,
            mino_x_dir: 0,
            rotate_dir: 0,
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

            if self.mino_x_dir != 0 {
                mino.pos.x += self.mino_x_dir;
                'loop_y: for y in 0..BOARD_ROW {
                    for x in 0..BOARD_COL {
                        let mino_pixel = mino.get_pixel(Int2::new(x as i32, y as i32));
                        if mino_pixel != 0 && self.board[y][x] != 0 {
                            mino.pos.x -= self.mino_x_dir;
                            break 'loop_y;
                        }
                    }
                }
                self.mino_x_dir = 0;
            }

            if self.rotate_dir != 0 {
                mino.rotate(self.rotate_dir);
                'loop_y: for y in 0..BOARD_ROW {
                    for x in 0..BOARD_COL {
                        let mino_pixel = mino.get_pixel(Int2::new(x as i32, y as i32));
                        if mino_pixel != 0 && self.board[y][x] != 0 {
                            mino.rotate(-self.rotate_dir);
                            break 'loop_y;
                        }
                    }
                }
                self.rotate_dir = 0;
            }
        }
    }

    pub fn draw<W: Write>(&mut self, out: &mut W) -> Result<()> {
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

                if self.pre_rendered[y][x] != pixel {
                    write!(out, "{}{}", cursor::Goto(x as u16 + 1, y as u16 + 1), pixel)?;
                    self.pre_rendered[y][x] = pixel;
                }
            }
            write!(
                out,
                "{}\r\n",
                cursor::Goto(BOARD_COL as u16 + 1, y as u16 + 1)
            )?;
        }

        out.flush()?;

        Ok(())
    }
}
