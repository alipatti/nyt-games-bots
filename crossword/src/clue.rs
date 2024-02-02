use crate::Position;

pub(super) enum Direction {
    Across,
    Down,
}

pub(super) struct Clue {
    direction: Direction,
    start: Position,
    length: usize,
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
                Direction::Down => self.start.col + i,
            },
            col: match self.direction {
                Direction::Across => self.start.row + i,
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
