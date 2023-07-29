use crate::{mino::Mino, vector::Int2, BOARD_COL, BOARD_ROW};
use anyhow::{Ok, Result};
use std::io::Write;
use termion::{clear, cursor};

pub struct Board {
    pub board: [[u32; BOARD_COL]; BOARD_ROW],
    active_mino: Option<Mino>,
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
        })
    }

    pub fn put_mino(&mut self, mut mino: Mino) {
        mino.pos = Int2::new(BOARD_COL as i32 / 2, 0);
        self.active_mino = Some(mino);
    }

    pub fn update(&mut self) {
        if let Some(ref mut mino) = self.active_mino {
            mino.pos.y += 1;
        }
    }

    pub fn draw<W: Write>(&mut self, out: &mut W) -> Result<()> {
        write!(out, "{}", clear::All)?;
        write!(out, "{}", cursor::Goto(1, 1))?;

        for (y, row) in self.board.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let Some(ref mino) = self.active_mino {
                    write!(
                        out,
                        "{}",
                        if mino.get_pixel(Int2::new(x as i32, y as i32)) != 0 || *col != 0 {
                            '#'
                        } else {
                            ' '
                        }
                    )?;
                }
            }
            write!(out, "\r\n")?;
        }

        out.flush()?;

        Ok(())
    }
}
