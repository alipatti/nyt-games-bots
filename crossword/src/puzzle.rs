use std::{collections::HashMap, error::Error};

use crate::board::Board;
use crate::clue::Clue;
use crate::word::Word;

use crate::{VOCAB_PATH, VOCAB_SIZE};

pub struct Puzzle {
    board: Board,
    clues: Vec<Clue>,
    vocab: HashMap<usize, Vec<Word>>,
}

impl TryFrom<Board> for Puzzle {
    type Error = Box<dyn Error>;

    fn try_from(board: Board) -> Result<Self, Self::Error> {
        let clues = board.clues();

        if clues.iter().any(|c| c.len() < 3) {
            return Err("Clues must have length at least 3.".into());
        }

        Ok(Self {
            board,
            vocab: load_vocabulary()?,
            clues,
        })
    }
}

// impl<T> TryFrom<T> for Puzzle
// where
//     T: TryInto<Board>,
// {
//     type Error = Box<dyn Error>;
//
//     fn try_from(value: T) -> Result<Self, Self::Error> {
//         let board = Board::try_from(value)?;
//         Self::try_from(board)
//     }
// }

impl Puzzle {
    /// Returns the possbile fills of this [`Puzzle`].
    pub fn possbile_fills(&self) -> impl IntoIterator<Item = Board> {
        todo!();
        Vec::new()
    }

    /// Returns up to `k` possible next moves, ordered from most to least promising.
    fn possible_next_moves(&self, board: Board, k: usize) -> Vec<Board> {
        todo!()
    }

    /// Calculates how many possible words can be added to a given board.
    /// Used to explore more promising portions of the tree first.
    fn n_moves(&self, board: &Board) -> usize {
        self.clues
            .iter()
            .map(|clue| self.clue_n_fits(clue))
            .sum::<usize>()
    }

    /// Returns the number of words in the vocabulary that would fit this clue.
    fn clue_n_fits(&self, clue: &Clue) -> usize {
        let words = self
            .vocab
            .get(&clue.len())
            .expect("No words of length {clue.len()}");

        if self.clue_filled(clue) {
            1 // filled, so there's one match
        } else if self.clue_empty(clue) {
            words.len() // empty, so any word will fit
        } else {
            // count the number of words that fit
            // PERF: is there a better (sub-linear) data structure for this?
            words
                .iter()
                .map(|word| self.clue_fits(word, clue) as usize)
                .sum::<usize>()
        }
    }

    fn next_unfilled_clue(&'_ self) -> Option<&'_ Clue> {
        self.clues.iter().filter(|clue| !clue.is_filled()).next()
    }

    fn clue_fits(&self, word: &Word, clue: &Clue) -> bool {
        clue.positions()
            .zip(word.chars())
            .all(|(pos, &char)| self.board[pos].matches(char))
    }

    fn clue_empty(&self, clue: &Clue) -> bool {
        clue.positions().all(|pos| self.board[pos].is_empty())
    }

    fn clue_filled(&self, clue: &Clue) -> bool {
        clue.positions().all(|pos| !self.board[pos].is_empty())
    }
}

fn load_vocabulary() -> Result<HashMap<usize, Vec<Word>>, Box<dyn Error>> {
    let words = include_str!("../../vocab.txt")
        .split("\n")
        .take(VOCAB_SIZE)
        .map(|line| {
            let mut cols = line.split_whitespace();

            let word: Word = cols
                .next()
                .ok_or(format!("Line contains no word: {line}"))?
                .try_into()?;

            let popularity: usize = cols
                .next()
                .ok_or(format!("Line contains no popularity score: {line}"))?
                .parse()?;

            Ok::<_, Box<dyn Error>>((word, popularity))
        });

    let mut out: HashMap<usize, Vec<Word>> = HashMap::new();

    for result in words {
        let (word, _popularity) = result?;
        let len = word.len();

        if !out.contains_key(&len) {
            out.insert(len, Vec::new());
        }

        out.get_mut(&len).unwrap().push(word);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vocab_loads_properly() {
        load_vocabulary().expect("Should generate vocabulary");
    }

    #[test]
    fn all_words_loaded() {
        let vocab = load_vocabulary().unwrap();

        assert_eq!(
            vocab.values().map(|words| words.len()).sum::<usize>(),
            VOCAB_SIZE
        );
    }

    #[test]
    fn lengths_sorted_properly() {
        let vocab = load_vocabulary().unwrap();

        assert!(vocab
            .iter()
            .all(|(&len, words)| words.iter().all(|w| w.len() == len)))
    }
}
