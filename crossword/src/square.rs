use std::fmt::{Debug, Write};

const EMPTY_CHAR: char = ' ';
const BLOCKED_CHAR: char = '#';

/// A square in a crossword puzzle. Guaranteed to contain 0, 1, or a
/// valid ASCII uppercase character.
#[derive(Clone, PartialEq, Eq)]
pub(super) struct Square(u8);

impl Square {
    const EMPTY: Self = Self(0);
    const BLOCKED: Self = Self(1);

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
            0..=1 => Ok(Self(value)),
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
        if value == ' ' {
            Ok(Self(0)) // empty
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
    fn test_square_from_u8() {
        todo!()
    }

    #[test]
    fn test_square_from_char() {
        todo!()
    }
}
