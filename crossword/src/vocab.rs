use std::{collections::HashMap, error::Error, fs};

use itertools::Itertools;

use crate::{square::Square, word::Word};

const SERDE_VOCAB_PATH: &'static str = "./crossword-vocab.serde";
const TXT_VOCAB_PATH: &'static str = "../word_list.txt";

type Score = usize;

/// Thin wrapper around a ternary search tree
pub(crate) struct Vocab(tst::TSTMap<Score>);

impl Vocab {
    pub(crate) fn new<'a, T>(word_list: T) -> Self
    where
        T: IntoIterator<Item = (&'a str, Score)>,
    {
        let mut tree = tst::TSTMap::new();

        for (word, score) in word_list {
            tree.insert(word, score);
        }

        Vocab(tree)
    }

    pub(crate) fn matches(&'_ self, squares: Vec<Square>) -> &'_ [Word] {
        let matching_words = self
            .vocab
            .get(&squares)
            .map(|v| v.as_slice()) // convert vec to slice
            .unwrap_or(&[]); // return empty slice if there aren't matches

        matching_words
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::square::Square;

    use super::Vocab;

    #[test]
    fn vocab_data_struct_works() {
        let words = ["hello", "world", "jello"];

        let partial_words =
            ["     ", " ello", "h ll ", "    o", "worl ", "w  ld"]
                .iter()
                .map(|p| {
                    p.chars()
                        .map(|c| Square::try_from(c).unwrap())
                        .collect_vec()
                })
                .collect_vec();

        let vocab = Vocab::new(words);

        // check that all the partial words are in the vocab
        assert!(partial_words
            .iter()
            .all(|p| vocab.matches(p.to_owned()).len() > 0));

        assert!(vocab.matches(partial_words[0].to_owned()).len() == 3);
        assert!(vocab.matches(partial_words[1].to_owned()).len() == 2);
    }
}
