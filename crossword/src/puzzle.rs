use std::error::Error;
use std::iter;

use pathfinding::directed::dfs::dfs_reach;

use crate::board::Board;
use crate::clue::Clue;
use crate::vocab::{load_cached_vocab, Vocab};

pub struct Puzzle {
    board: Board,
    vocab: Vocab,
    clues: Vec<Clue>,
}

impl TryFrom<Board> for Puzzle {
    type Error = Box<dyn Error>;

    fn try_from(board: Board) -> Result<Self, Self::Error> {
        let clues = board.clues();

        if clues.iter().any(|c| c.len() < 3) {
            return Err("Clues must have length at least 3.".into());
        }

        let vocab = load_cached_vocab()?;

        Ok(Self {
            board,
            vocab,
            clues,
        })
    }
}

impl Puzzle {
    fn fill_generator(&'_ self) -> impl Iterator<Item = Board> + '_ {
        dfs_reach(self.board.clone(), |board| self.next_moves(board.clone()))
    }

    pub fn valid_fills(&'_ self) -> impl Iterator<Item = Board> + '_ {
        self.fill_generator().filter(|b| b.is_filled())
    }

    fn next_moves(&'_ self, board: Board) -> impl Iterator<Item = Board> + '_ {
        let board2 = board.clone();

        let words_and_clues = self.clues.iter().flat_map(move |clue| {
            let squares = board.clue_squares(clue);
            iter::repeat(clue).zip(self.vocab.matches(squares))
        });

        let all_next_moves =
            words_and_clues.map(move |(clue, word)| board2.insert(word, clue));

        let good_next_moves = all_next_moves.filter(|new_board| {
            self.clues.iter().all(|clue| {
                !self.vocab.matches(new_board.clue_squares(clue)).is_empty()
            })
        });

        good_next_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_works_small() {
        let board = Board::try_from("    \n    \n    \n    ")
            .expect("Should be a valid board.");
        let puzzle =
            Puzzle::try_from(board).expect("Should be a valid puzzle.");

        let _ = puzzle.fill_generator().take(10);
    }
}
