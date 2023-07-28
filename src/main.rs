use std::io::{stdin, stdout};

use anyhow::{Ok, Result};
use board::Board;
use termion::{
    event::{Event, Key},
    input::TermReadEventsAndRaw,
    raw::IntoRawMode,
    screen::IntoAlternateScreen,
};

mod board;
mod mino;
mod vector;

pub const BOARD_ROW: usize = 10;
pub const BOARD_COL: usize = 10;

fn main() -> Result<()> {
    // let minos = [Mino::new([
    //     [1, 1, 0, 0],
    //     [1, 1, 0, 0],
    //     [0, 0, 0, 0],
    //     [0, 0, 0, 0],
    // ])];

    let mut board = Board::new()?;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?.into_alternate_screen()?;

    // main loop
    board.draw(&mut stdout)?;
    for ele in stdin.events_and_raw() {
        let (ev, _) = ele?;

        if ev == Event::Key(Key::Ctrl('c')) {
            return Ok(());
        }

        board.draw(&mut stdout)?;
    }

    Ok(())
}
