use crate::Position;
use derivative::Derivative;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Direction {
    Across,
    Down,
}

#[derive(Debug, Derivative)]
#[derivative(PartialEq, Eq)]
pub(crate) struct Clue {
    pub(crate) direction: Direction,
    pub(crate) start: Position,
    pub(crate) length: usize,
}

impl Clue {
    pub(super) fn len(&self) -> usize {
        self.length
    }

    /// Returns an interator of the positions of the squares covered by this clue..
    pub(super) fn positions(&'_ self) -> impl Iterator<Item = Position> + '_ {
        let positions = (0..self.length).map(|i| Position {
            row: match self.direction {
                Direction::Across => self.start.row,
                Direction::Down => self.start.row + i,
            },
            col: match self.direction {
                Direction::Across => self.start.col + i,
                Direction::Down => self.start.col,
            },
        });

        positions
    }

    pub(crate) fn is_filled(&self) -> bool {
        todo!()
    }

    pub(crate) fn is_empty(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::Position;

    use super::{Clue, Direction::*};

    #[test]
    fn clue_positions_work() {
        let clue = Clue {
            direction: Down,
            start: Position { row: 0, col: 1 },
            length: 4,
        };

        assert_eq!(
            clue.positions().collect_vec(),
            vec![
                Position { row: 0, col: 1 },
                Position { row: 1, col: 1 },
                Position { row: 2, col: 1 },
                Position { row: 3, col: 1 },
            ]
        );

        // ----

        let clue = Clue {
            direction: Across,
            start: Position { row: 3, col: 7 },
            length: 4,
        };

        assert_eq!(
            clue.positions().collect_vec(),
            vec![
                Position { row: 3, col: 7 },
                Position { row: 3, col: 8 },
                Position { row: 3, col: 9 },
                Position { row: 3, col: 10 },
            ]
        );
    }
}
