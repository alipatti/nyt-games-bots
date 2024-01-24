use ordered_float::NotNan;
use pathfinding::directed::dijkstra::dijkstra;
use prefix_tree::StringPrefixTree;

const N_PER_SIDE: usize = 3;
const N_SIDES: usize = 4;
const EPSILON: f32 = 0.00001;
const WORD_LIST: &str = include_str!("../../vocab.txt");

pub struct LettersBoxedGame {
    letters: [[char; N_PER_SIDE]; N_SIDES],
    vocabulary: StringPrefixTree,
}

#[derive(Eq, PartialEq, Clone, Hash, Default, Debug)]
pub struct LettersBoxedState {
    used: [[bool; N_PER_SIDE]; N_SIDES],
    current_pos: Option<Position>,
    current_word: String,
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
struct Position {
    side: usize,
    index: usize,
}

impl LettersBoxedGame {
    pub fn new(letters: [[char; N_PER_SIDE]; N_SIDES]) -> Self {
        Self {
            letters,
            vocabulary: Self::load_vocabulary(),
        }
    }

    fn load_vocabulary() -> StringPrefixTree {
        let mut vocab = StringPrefixTree::empty();

        for w in WORD_LIST.split("\n").filter(|w| !w.is_empty()) {
            vocab.add(w)
        }

        vocab
    }

    pub fn solve(&self) -> Option<(Vec<LettersBoxedState>, usize)> {
        let (result, cost) = dijkstra(
            &LettersBoxedState::default(),
            |state| state.successors(self),
            |state| state.finished(),
        )?;

        Some((result, cost.floor() as usize))
    }
}

impl LettersBoxedState {
    fn successors(&self, game: &LettersBoxedGame) -> Vec<(Self, NotNan<f32>)> {
        let possible_letters = game
            .letters
            .iter()
            // add location to the letters
            .enumerate()
            .map(|(side, letters)| {
                letters
                    .iter()
                    .enumerate()
                    .map(move |(index, c)| (Position { side, index }, c))
            })
            .flatten()
            // filter out letters on the same side
            .filter(|(next_pos, _)| {
                match &self.current_pos {
                    None => true, // first move. can play anywhere
                    Some(current_pos) => current_pos.side != next_pos.side,
                }
            });

        let valid_next_moves =
            possible_letters.filter_map(|(next_pos, next_letter)| {
                let mut next_word = self.current_word.clone();
                next_word.push(*next_letter);
                if game.vocabulary.contains_prefix(&next_word) {
                    let mut used = self.used.clone();
                    used[next_pos.side][next_pos.index] = true;

                    let new_state = Self {
                        used,
                        current_pos: Some(next_pos),
                        current_word: next_word,
                    };
                    Some((new_state, NotNan::new(EPSILON).unwrap()))
                } else {
                    None
                }
            });

        if game.vocabulary.contains(&self.current_word) {
            valid_next_moves
                .chain(std::iter::once((
                    LettersBoxedState {
                        used: self.used,
                        current_pos: self.current_pos,
                        current_word: self
                            .current_word
                            .chars()
                            .next_back()
                            .unwrap()
                            .to_string(),
                    },
                    NotNan::new(1.).unwrap(),
                )))
                .collect()
        } else {
            valid_next_moves.collect()
        }
    }

    fn finished(&self) -> bool {
        (self.used.into_iter().flatten().all(|b| b))
            && self.current_word.len() == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn game() -> LettersBoxedGame {
        LettersBoxedGame::new([
            ['l', 'c', 'v'],
            ['r', 'w', 'a'],
            ['e', 'n', 'g'],
            ['t', 'i', 'o'],
        ])
    }

    #[test]
    fn test_first_move() {
        let game = game();
        let start = LettersBoxedState::default();

        assert_eq!(start.successors(&game).len(), N_SIDES * N_PER_SIDE)
    }

    #[test]
    fn test_solve() {}
}

fn main() {
    // 1/23/24
    let game = LettersBoxedGame::new([
        ['t', 'c', 'p'],
        ['y', 'i', 'r'],
        ['d', 'h', 'a'],
        ['o', 'n', 'l'],
    ]);

    // 1/24/24
    // let game = LettersBoxedGame::new([
    //     ['l', 'c', 'v'],
    //     ['r', 'w', 'a'],
    //     ['e', 'n', 'g'],
    //     ['t', 'i', 'o'],
    // ]);

    if let Some((solution, cost)) = game.solve() {
        println!(
            "{:#?}",
            solution
                .into_iter()
                .map(|state| state.current_word)
                .collect::<Vec<_>>()
        );
    }
}
