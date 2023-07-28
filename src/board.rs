use std::io::Write;

use anyhow::{Ok, Result};
use termion::{clear, cursor};

use crate::{BOARD_COL, BOARD_ROW};

pub struct Board {
    pub board: [[u32; BOARD_COL]; BOARD_ROW],
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

        Ok(Self { board })
    }

    pub fn draw<W: Write>(&mut self, out: &mut W) -> Result<()> {
        write!(out, "{}", clear::All)?;
        write!(out, "{}", cursor::Goto(1, 1))?;

        for row in self.board {
            for col in row {
                write!(out, "{}", if col != 0 { '#' } else { ' ' })?;
            }
            write!(out, "\r\n")?;
        }

        out.flush()?;

        Ok(())
    }
}
