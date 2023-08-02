use crate::{mino::Mino, vector::Int2, BOARD_COL, BOARD_ROW};
use anyhow::{Ok, Result};
use std::{char, io::Write};
use termion::cursor;

pub struct Board {
    pub board: [[u32; BOARD_COL]; BOARD_ROW],
    pub pre_rendered: [[char; BOARD_COL]; BOARD_ROW],
    pub minos: Vec<Mino>,
    pub exist_active_mino: bool,
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
            minos: vec![],
            exist_active_mino: false,
            mino_x_dir: 0,
            rotate_dir: 0,
        })
    }

    pub fn put_mino(&mut self, mut mino: Mino) {
        mino.pos = Int2::new(BOARD_COL as i32 / 2, 0);
        self.minos.push(mino);
        self.exist_active_mino = true;
    }

    pub fn update(&mut self) {
        _ = self.delete_line();

        let length = self.minos.len();
        for (idx, mino) in self.minos.iter_mut().enumerate() {
            mino.pos.y += 1;
            'loop_y: for y in 0..BOARD_ROW {
                for x in 0..BOARD_COL {
                    let mino_pixel = mino.get_pixel(Int2::new(x as i32, y as i32));
                    if mino_pixel != 0 && self.board[y][x] != 0 {
                        mino.pos.y -= 1;
                        mino.apply(&mut self.board);

                        if idx == length - 1 {
                            self.exist_active_mino = false;
                        }
                        break 'loop_y;
                    }
                }
            }
        }

        if self.exist_active_mino {
            let mino = self.minos.last_mut().unwrap();

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

    fn delete_line(&mut self) -> bool {
        let mut is_deleted = false;
        for y in (0..BOARD_ROW - 1).rev() {
            let mut is_line = true;
            for x in 1..BOARD_COL - 1 {
                if self.board[y][x] == 0 {
                    is_line = false;
                    break;
                }
            }
            if is_line {
                is_deleted = true;
                for x in 1..BOARD_COL - 1 {
                    self.board[y][x] = 0;

                    for mino in self.minos.iter_mut() {
                        mino.set_pixel(Int2::new(x as i32, y as i32), 0);
                    }
                }
            }
        }

        is_deleted
    }

    pub fn draw<W: Write>(&mut self, out: &mut W) -> Result<()> {
        for (y, row) in self.board.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let mino_pixel = if self.exist_active_mino {
                    Some(
                        self.minos
                            .last()
                            .unwrap()
                            .get_pixel(Int2::new(x as i32, y as i32)),
                    )
                } else {
                    None
                };
                let pixel = if let Some(pixel) = mino_pixel {
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
