mod board;
mod clue;
mod puzzle;
mod square;
mod word;

const VOCAB_SIZE: usize = 10_000;
const VOCAB_PATH: &'static str = "../../vocab.txt";

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

fn main() {
    // puzzle::Puzzle::from_rows(["   ", "    ", "    ", "    "]);
}
