use std::collections::HashMap;

use itertools::Itertools;

use crate::{square::Square, word::Word};

pub(crate) struct Vocab {
    vocab: HashMap<Vec<Square>, Vec<Word>>,
    // TODO: add word list that this data strcture referenes
    // vocab: HashMap<Vec<Square>, Vec<&'a Word>>,
}

// impl<'a> Vocab<'a> {
impl Vocab {
    pub(crate) fn new(word_list: &[&str]) -> Self {
        let mut vocab = HashMap::new();

        // convert strings to `Words`
        let word_list = word_list
            .iter()
            .map(|w| Word::try_from(w.as_bytes()))
            .filter_map(|w| w.ok()); // silently drop failed conversions

        for word in word_list {
            // TODO: handle errors

            let partial_words = word
                .chars
                .iter()
                .map(|c| Square::try_from(*c).unwrap()) // convert char to square
                .map(|s| [s, Square::EMPTY]) // each square can possibly be empty
                .multi_cartesian_product();

            for partial_word in partial_words {
                if !vocab.contains_key(&partial_word) {
                    vocab.insert(partial_word.clone(), Vec::new());
                }

                vocab.get_mut(&partial_word).unwrap().push(word.clone())
            }
        }

        Self { vocab }
    }

    pub(crate) fn matches<T>(&'_ self, squares: T) -> &'_ [Word]
    where
        T: IntoIterator<Item = Square>,
    {
        let squares = squares.into_iter().collect_vec();

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

        let vocab = Vocab::new(&words);

        // check that all the partial words are in the vocab
        assert!(partial_words
            .iter()
            .all(|p| vocab.matches(p.to_owned()).len() > 0));

        assert!(vocab.matches(partial_words[0].to_owned()).len() == 3);
        assert!(vocab.matches(partial_words[1].to_owned()).len() == 2);
    }
}
