use std::fmt::{Debug, Write};

const EMPTY_CHAR: char = ' ';
const BLOCKED_CHAR: char = '#';

/// A square in a crossword puzzle. Guaranteed to contain 0, 1, or a
/// valid ASCII uppercase character.
#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) struct Square(u8);

impl Square {
    pub(super) const EMPTY: Self = Self(0);
    pub(super)const BLOCKED: Self = Self(1);
    const EMPTY_CHAR: char = ' ';
    const BLOCKED_CHAR: char = '#';

    pub(crate) fn is_empty(&self) -> bool {
        *self == Self::EMPTY
    }

    pub(crate) fn is_blocked(&self) -> bool {
        *self == Self::BLOCKED
    }

    pub(crate) fn matches(&self, char: u8) -> bool {
        self.is_empty() || self.0 == char
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(if self.is_empty() {
            EMPTY_CHAR
        } else if self.is_blocked() {
            BLOCKED_CHAR
        } else {
            self.0 as char
        })
    }
}

impl TryFrom<u8> for Square {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=1 => Ok(Self(value)), // empty or blocked
            _ => {
                if value.is_ascii_uppercase() {
                    Ok(Self(value))
                } else {
                    Err("Not a valid uppercase ASCII char.")
                }
            }
        }
    }
}

impl TryFrom<char> for Square {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == Self::EMPTY_CHAR {
            Ok(Self(0)) // empty
        } else if value == Self::BLOCKED_CHAR {
            Ok(Self(1))
        } else if value.is_ascii_uppercase() {
            Ok(Self(value as u8))
        } else if value.is_ascii_lowercase() {
            Ok(Self(value.to_ascii_uppercase() as u8))
        } else {
            Err("Not a valid uppercase ASCII char.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_from_char() {
        Square::try_from('a').expect("Should be an a.");
        Square::try_from('X').expect("Should be a X.");
        Square::try_from(' ').expect("Should be an empty square.");
        Square::try_from('#').expect("Should be a blocked square.");
        
        Square::try_from('*').expect_err("Should be an invalid square.");
        Square::try_from('_').expect_err("Should be an invalid square.");
        Square::try_from('$').expect_err("Should be an invalid square.");
        Square::try_from('/').expect_err("Should be an invalid square.");

    }
}
