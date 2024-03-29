use std::fmt::{Debug, Write};
use std::ops::{Index, IndexMut};

use itertools::Itertools;

use crate::clue::Clue;
use crate::clue::Direction::{Across, Down};
use crate::square::Square;
use crate::word::Word;
use crate::Position;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Board {
    squares: Vec<Square>,
    n_rows: usize,
    n_cols: usize,
}

impl Index<Position> for Board {
    type Output = Square;

    fn index(&self, index: Position) -> &Self::Output {
        let i = self.n_cols * index.row + index.col;
        &self.squares[i]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        let i = self.n_cols * index.row + index.col;
        &mut self.squares[i]
    }
}

impl Board {
    pub(crate) fn is_filled(&self) -> bool {
        self.squares.iter().all(|square| !square.is_empty())
    }

    pub(crate) fn rows(
        &self,
    ) -> impl IntoIterator<Item = impl IntoIterator<Item = &Square>> {
        self.squares.chunks(self.n_cols)
    }

    pub(crate) fn cols(
        &self,
    ) -> impl IntoIterator<Item = impl Iterator<Item = &Square>> {
        (0..self.n_cols).map(move |col| {
            (0..self.n_rows)
                .map(move |row| &self.squares[self.n_cols * row + col])
        })
    }

    pub(crate) fn positions(&self) -> impl Iterator<Item = Position> {
        (0..self.n_rows)
            .cartesian_product(0..self.n_cols)
            .map(|(row, col)| Position { row, col })
    }

    pub(crate) fn clue_squares(&self, clue: &Clue) -> Vec<Square> {
        clue.positions().map(|p| self[p].clone()).collect_vec()
    }

    pub(crate) fn clues(&self) -> Vec<Clue> {
        let mut clues = Vec::new();

        for pos in self.positions() {
            if self[pos].is_blocked() {
                continue; // blocked, can't be clue here
            }

            // check for across clue
            if pos.col == 0 || self[pos.left()].is_blocked() {
                let length = {
                    let mut current = pos;
                    let mut i = 0;
                    while current.col < self.n_cols
                        && !self[current].is_blocked()
                    {
                        current = current.right();
                        i += 1;
                    }
                    i
                };
                clues.push(Clue {
                    direction: Across,
                    start: pos,
                    length,
                })
            }

            // check for down clue
            if pos.row == 0 || self[pos.up()].is_blocked() {
                let length = {
                    let mut current = pos;
                    let mut i = 0;
                    while current.row < self.n_rows
                        && !self[current].is_blocked()
                    {
                        current = current.down();
                        i += 1;
                    }
                    i
                };
                clues.push(Clue {
                    direction: Down,
                    start: pos,
                    length,
                });
            }
        }

        clues
    }

    pub(crate) fn insert(&self, word: &Word, clue: &Clue) -> Self {
        let mut board = self.clone();
        for (p, c) in clue.positions().zip(word.chars.into_iter()) {
            board[p] = Square::try_from(*c).unwrap();
        }

        board
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                let pos = Position { row: i, col: j };
                self[pos].fmt(f)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl TryFrom<&str> for Board {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines = value.split("\n").collect_vec();

        let rows = lines.len();
        let cols = lines[0].len();

        if lines.iter().any(|row| row.len() != cols) {
            return Err("Rows have different lengths.");
        }

        let squares = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(|c| Square::try_from(c))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            squares,
            n_rows: rows,
            n_cols: cols,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn board_from_string() {
        Board::try_from("asdf\nasdf\nasdf\nasdf")
            .expect("Should be a valid board.");

        Board::try_from("#   \n#  #\n#  #\n   #")
            .expect("Should be a valid board.");

        Board::try_from("asdf\nas").expect_err("Should be an invalid board.");
    }

    #[test]
    fn correct_board_size() {
        let board = Board::try_from("    #\n     \n     \n     \n#    ")
            .expect("Should be a valid board.");

        assert_eq!(board.n_rows, 5);
        assert_eq!(board.n_cols, 5);
    }

    #[test]
    fn correct_positions() {
        let board = Board::try_from("    #\n     \n     \n     \n#    ")
            .expect("Should be a valid board.");

        assert_eq!(
            board.positions().collect_vec().len(),
            board.n_rows * board.n_cols
        );
    }

    #[test]
    fn clues_from_board() {
        let board = Board::try_from("    #\n     \n     \n     \n#    ")
            .expect("Should be a valid board.");
        let clues = board.clues();

        println!("{:#?}", clues);

        assert!(clues.contains(&Clue {
            direction: Across,
            start: Position { row: 0, col: 0 },
            length: 4,
        }));

        assert!(clues.contains(&Clue {
            direction: Down,
            start: Position { row: 1, col: 4 },
            length: 4,
        }));

        assert!(clues.contains(&Clue {
            direction: Down,
            start: Position { row: 0, col: 2 },
            length: 5,
        }));

        assert!(!clues.contains(&Clue {
            direction: Down,
            start: Position { row: 2, col: 2 },
            length: 3,
        }));
    }
}
