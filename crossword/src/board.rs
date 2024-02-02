use std::fmt::{Debug, Write};
use std::ops::Index;

use crate::square::Square;
use crate::Position;

#[derive(Clone)]
pub(super) struct Board {
    squares: Vec<Square>,
    rows: usize,
    cols: usize,
}

impl Index<Position> for Board {
    type Output = Square;

    fn index(&self, index: Position) -> &Self::Output {
        let i = self.cols * index.row + index.col;
        &self.squares[i]
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let pos = Position { row: i, col: j };
                self[pos].fmt(f)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Board {
    fn is_filled(&self) -> bool {
        self.squares.iter().all(|square| !square.is_empty())
    }
}
