use std::{error::Error, io::stdin};

use board::Board;
use puzzle::Puzzle;

mod board;
mod clue;
mod puzzle;
mod square;
mod vocab;
mod word;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub(crate) fn right(&self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }

    pub(crate) fn left(&self) -> Self {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }

    pub(crate) fn up(&self) -> Self {
        Self {
            row: self.row - 1,
            col: self.col,
        }
    }

    pub(crate) fn down(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let board = Board::try_from("GAMMA\nALOOF\nSLANT\n#   #\n#   #")
        .expect("Should be a valid board.");

    let puzzle =
        Puzzle::try_from(board.clone()).expect("Should be a valid puzzle.");
    let mut s = String::new();

    println!("{:?}", board);

    for fill in puzzle.valid_fills() {
        println!("{:?}", fill);
        let _ = stdin().read_line(&mut s)?;
    }

    Ok(())
}
