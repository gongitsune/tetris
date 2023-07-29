use anyhow::{Ok, Result};
use board::Board;
use mino::Mino;
use std::{
    collections::VecDeque,
    io::{stdin, stdout},
    sync::{Arc, Mutex},
    time::Duration,
};
use termion::{
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
    screen::IntoAlternateScreen,
};
use tokio_util::sync::CancellationToken;

mod board;
mod mino;
mod vector;

pub const BOARD_ROW: usize = 10;
pub const BOARD_COL: usize = 10;

#[tokio::main]
async fn main() -> Result<()> {
    let minos = [Mino::new([
        [1, 1, 0, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ])];

    let mut board = Board::new()?;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?.into_alternate_screen()?;

    let event_queue = Arc::new(Mutex::new(VecDeque::new()));
    // key loop
    let event_queue_clone = event_queue.clone();
    let handle = tokio::spawn(async move {
        for ele in stdin.events() {
            let ele = ele.unwrap();

            if ele == Event::Key(Key::Ctrl('c')) {
                break;
            } else {
                let mut lock = event_queue_clone.lock().unwrap();
                (*lock).push_front(ele);
            }
        }
    });

    // main loop
    board.put_mino(minos[0].clone());
    board.draw(&mut stdout)?;

    loop {
        if handle.is_finished() {
            return Ok(());
        }

        {
            let mut events = event_queue.lock().unwrap();
            while let Some(Event::Key(key)) = events.pop_back() {
                match key {
                    Key::Right => {
                        board.mino_x_dir = 1;
                    }
                    Key::Left => {
                        board.mino_x_dir = -1;
                    }
                    _ => {}
                }
            }
        }

        board.update();
        board.draw(&mut stdout).unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
