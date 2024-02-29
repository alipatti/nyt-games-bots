use std::error::Error;
use std::iter;

use itertools::Itertools;
use pathfinding::directed::dfs::dfs_reach;

use crate::board::Board;
use crate::clue::Clue;
use crate::vocab::Vocab;

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

        let word_list = include_str!("../../word_list.txt").split_whitespace();
        let vocab = Vocab::new(word_list);

        Ok(Self {
            board,
            vocab,
            clues,
        })
    }
}

impl Puzzle {
    pub fn possbile_fills(&'_ self) -> impl Iterator<Item = Board> + '_ {
        dfs_reach(self.board.clone(), |board| self.next_moves(board.clone()))
        // .filter(|b| b.is_filled())
    }

    fn next_moves(&'_ self, board: Board) -> impl Iterator<Item = Board> + '_ {
        let board2 = board.clone();

        let words_and_clues = self.clues.iter().flat_map(move |clue| {
            let squares = board.clue_squares(clue);
            iter::repeat(clue).zip(self.vocab.matches(squares))
        });

        words_and_clues.map(move |(clue, word)| board2.insert(word, clue))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver_works_small() {
        let board = Board::try_from("   \n   \n   \n   ")
            .expect("Should be a valid board.");
        let puzzle =
            Puzzle::try_from(board).expect("Should be a valid puzzle.");

        println!("{:#?}", puzzle.clues);

        for board in puzzle.possbile_fills().take(10) {
            println!("{:?}", board);
        }

        panic!()
    }
}
