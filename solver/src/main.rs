use derivative::Derivative;
use std::{
    ops::{BitOr, Index, IndexMut},
    vec,
};

use itertools::Itertools;

use pathfinding::directed::{
    dfs::dfs_reach, dijkstra::dijkstra_reach, yen::yen,
};
use prefix_tree::{PrefixTree, QueryResult};

const N_PER_SIDE: usize = 3;
const N_SIDES: usize = 4;

const N_SOLUTIONS: usize = 1;

const WORD_LIST: &str = include_str!("../../vocab.txt");
const VOCAB_SIZE: usize = 50_000;
const WORD_LEN_THRESHOLD: usize = 3;

pub type Letter = char;

/// word -> cost
type Vocabulary = PrefixTree<Letter, usize>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Position(usize, usize);

#[derive(Default, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Board<T>([[T; N_PER_SIDE]; N_SIDES]);

/// Let's us index boards like `board[pos]`
impl<T> Index<Position> for Board<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<T> IndexMut<Position> for Board<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl BitOr for &Board<bool> {
    type Output = Board<bool>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut output = Board::default();

        for (i, j) in (0..N_SIDES).cartesian_product(0..N_PER_SIDE) {
            output.0[i][j] = self.0[i][j] | rhs.0[i][j];
        }
        output
    }
}

impl Board<bool> {
    fn all_true(&self) -> bool {
        self.0.into_iter().flatten().all(|x| x)
    }

    fn all_false(&self) -> bool {
        self.0.into_iter().flatten().all(|x| !x)
    }
}

// #[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
#[derive(Derivative, Clone, Default)]
#[derivative(PartialEq, Eq, Hash)]
pub struct GameState {
    pos: Position,
    letters_used: Board<bool>,

    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    moves: Vec<String>,
}

pub struct LettersBoxedGame {
    letters: Board<Letter>,
    vocabulary: Vocabulary,
}

impl LettersBoxedGame {
    pub fn new(letters: Board<Letter>) -> Self {
        Self {
            letters,
            vocabulary: Self::load_vocabulary(),
        }
    }

    fn load_vocabulary() -> Vocabulary {
        let mut vocab: Vocabulary = PrefixTree::empty();

        for (word, popularity) in WORD_LIST
            .split("\n")
            .map(|line| {
                let mut line = line.split_whitespace();
                let word = line.next().unwrap();
                let popularity: usize = line.next().unwrap().parse().unwrap();

                (word, popularity)
            })
            .filter(|(word, _)| word.len() >= WORD_LEN_THRESHOLD)
            .take(VOCAB_SIZE)
        {
            vocab.set(word.chars(), popularity);
        }

        vocab
    }

    pub fn solve(&self) -> impl IntoIterator<Item = Vec<String>> {
        let legal_words = self.all_possible_words();

        // words so far, letters so far, current pos
        // `None` to start
        let start: GameState = Default::default();

        // iterator to generate all valid sequences of guesses
        // e.g. cat -> target -> thespian -> ...
        // we will filter this later to only extract the terminal states
        let reachable_states = dijkstra_reach(
            // -- START STATE --
            &start,
            // -- NEIGHBORS --
            move |GameState {
                      pos: current_pos,
                      moves,
                      letters_used: current_letters,
                  },
                  _cost| {
                if current_letters.all_false() {
                    // first move, can play anywhere
                    legal_words
                        .0
                        .iter()
                        .flatten()
                        .flatten()
                        .map(|(pos, word, lets)| {
                            (
                                GameState {
                                    pos: pos.clone(),
                                    moves: vec![word.clone()],
                                    letters_used: lets.clone(),
                                },
                                1_000 - word.len(),
                            )
                        })
                        .collect_vec()
                } else {
                    // not the first move, can only play from the end of the previous word
                    legal_words[*current_pos]
                        .iter()
                        .map(|(next_pos, next_word, next_letters)| {
                            let mut new_moves = moves.clone();
                            new_moves.push(next_word.clone());

                            let next_state = GameState {
                                pos: next_pos.clone(),
                                moves: new_moves,
                                letters_used: current_letters | next_letters,
                            };
                            (next_state, 1_000 - next_word.len())
                        })
                        .collect_vec()
                }
            },
        );

        reachable_states
            // we want only the winning states
            .filter(|state| state.node.letters_used.all_true())
            // and only the move sequence
            .map(|state| state.node.moves)
    }

    #[inline]
    fn all_moves() -> impl Iterator<Item = Position> {
        (0..N_SIDES)
            .cartesian_product(0..N_PER_SIDE)
            .map(|(side, index)| Position(side, index))
    }

    /// Returns all legal moves from a given position,
    #[inline]
    fn valid_moves(pos: Position) -> impl Iterator<Item = Position> {
        Self::all_moves().filter(move |Position(side, _)| side != &pos.0)
    }

    /// Returns all possible words and their ending locations from a given starting point.
    fn all_possible_words(
        &self,
    ) -> Board<Vec<(Position, String, Board<bool>)>> {
        let mut words: Board<Vec<(Position, String, Board<bool>)>> =
            Default::default();

        for starting_pos in Self::all_moves() {
            let starting_word = self.letters[starting_pos].to_string();

            let mut starting_letters: Board<bool> = Default::default();
            starting_letters[starting_pos] = true;

            // generate all reachable positions
            let reachable = dfs_reach(
                // initial state
                (starting_pos, starting_word, starting_letters),
                // closure to generate possible next letters
                move |(current_pos, current_word, current_letters)| {
                    Self::valid_moves(current_pos.clone())
                        .filter_map(|next_pos| {
                            let next_char = self.letters[next_pos];
                            let next_word_chars = current_word
                                .chars()
                                .chain(std::iter::once(next_char));

                            match self.vocabulary.get(next_word_chars) {
                                prefix_tree::QueryResult::NotFound => None,
                                _ => {
                                    // update the word
                                    let mut next_word = current_word.clone();
                                    next_word.push(next_char);

                                    // update the letters used
                                    let mut next_letters =
                                        current_letters.clone();
                                    next_letters[next_pos] = true;

                                    Some((next_pos, next_word, next_letters))
                                }
                            }
                        })
                        .collect_vec()
                },
            );

            // keep only those which are valid words
            let all_reachable_words = reachable
                .filter(|(_end_pos, word, _letters)| {
                    match self.vocabulary.get(word.chars()) {
                        QueryResult::Value(_) => true,
                        _ => false,
                    }
                })
                .collect();

            words[starting_pos] = all_reachable_words;
        }

        words
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn game() -> LettersBoxedGame {
        LettersBoxedGame::new(Board([
            ['l', 'c', 'v'],
            ['r', 'w', 'a'],
            ['e', 'n', 'g'],
            ['t', 'i', 'o'],
        ]))
    }

    #[test]
    fn test_reachable_words() {
        let words = game().all_possible_words();
        println!("{:#?}", words.0[0][0]);
    }

    // TODO: add more tests
}

fn main() {
    // 1/22/24
    // let game = LettersBoxedGame::new(Board([
    //     ['t', 'c', 'p'],
    //     ['y', 'i', 'r'],
    //     ['d', 'h', 'a'],
    //     ['o', 'n', 'l'],
    // ]));

    // 1/23/24
    // let game = LettersBoxedGame::new(Board([
    //     ['l', 'c', 'v'],
    //     ['r', 'w', 'a'],
    //     ['e', 'n', 'g'],
    //     ['t', 'i', 'o'],
    // ]));

    // 1/24/24
    // flashbacks, samurai
    let game = LettersBoxedGame::new(Board([
        ['c', 'r', 'm'],
        ['k', 'b', 'l'],
        ['a', 'u', 'h'],
        ['i', 's', 'f'],
    ]));

    // 1/25/24
    // let game = LettersBoxedGame::new(Board([
    //     ['n', 'l', 'a'],
    //     ['i', 'g', 'c'],
    //     ['r', 'u', 'p'],
    //     ['q', 'k', 'o'],
    // ]));

    for moves in game.solve().into_iter().take(10) {
        println!("{:?}", moves);
    }
}
