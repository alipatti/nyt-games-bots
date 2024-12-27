use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs};

use itertools::Itertools;

use crate::{square::Square, word::Word};

const SERDE_VOCAB_PATH: &'static str = "./crossword-vocab.serde";
const TXT_VOCAB_PATH: &'static str = "../word_list.txt";

#[derive(Serialize, Deserialize)]
pub(crate) struct Vocab {
    vocab: HashMap<Vec<Square>, Vec<Word>>,
}

pub(crate) struct _Vocab<'a> {
    partial_map: HashMap<Vec<Square>, Vec<&'a Word>>,
    word_list: Vec<Word>,
}

// impl<'a> Vocab<'a> {
impl Vocab {
    pub(crate) fn new<'a, T>(word_list: T) -> Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        let mut vocab = HashMap::new();

        // convert strings to `Words`
        let word_list = word_list
            .into_iter()
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

    pub(crate) fn matches(&'_ self, squares: Vec<Square>) -> &'_ [Word] {
        let matching_words = self
            .vocab
            .get(&squares)
            .map(|v| v.as_slice()) // convert vec to slice
            .unwrap_or(&[]); // return empty slice if there aren't matches

        matching_words
    }
}

pub(crate) fn load_cached_vocab() -> Result<Vocab, Box<dyn Error>> {
    println!("Loading vocabulary...");

    let load_vocab = || -> Result<Vocab, Box<dyn Error>> {
        let f = fs::File::open(SERDE_VOCAB_PATH)?;
        let vocab: Vocab = bincode::deserialize_from(f)?;
        Ok(vocab)
    };

    match load_vocab() {
        Ok(vocab) => Ok(vocab),
        Err(err) => {
            dbg!(err);
            println!("Failed to load vocab file. Creating a new one...");

            let word_list = fs::read_to_string(TXT_VOCAB_PATH)?;
            let vocab = Vocab::new(word_list.split_whitespace());

            println!("Vocab created. Writing to disc...");
            let f = fs::File::create(SERDE_VOCAB_PATH)?;
            bincode::serialize_into(f, &vocab)?;

            Ok(vocab)
        }
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
