use std::fmt::{Debug, Write};

/// A word represented as an array of uppercase ascii bytes
#[derive(Clone)]
pub(super) struct Word {
    pub(super) chars: Box<[u8]>,
}

impl Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.chars.iter() {
            f.write_char(*c as char)?
        }

        Ok(())
    }
}

impl Word {
    pub(crate) fn len(&self) -> usize {
        self.chars.len()
    }
}

impl TryFrom<&[u8]> for Word {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let is_valid = value
            .into_iter()
            .all(|&byte| (byte as char).is_alphabetic());

        if is_valid {
            Ok(Self {
                chars: value.to_ascii_uppercase().into_boxed_slice(),
            })
        } else {
            Err("Not valid ASCII.")
        }
    }
}

impl TryFrom<&str> for Word {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes())
    }
}

impl TryFrom<String> for Word {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_word_from_string() {
        let valid = vec![
            "lowercase",
            "UPPERCASE",
            "miXedCaSE",
            "short",
            "reallyreallyreallylong",
        ];
        let invalid = vec!["this has spaces", "not-ascii", "huh?"];

        for w in valid {
            Word::try_from(w).expect("This should work");
        }

        for w in invalid {
            Word::try_from(w).expect_err("This shouldn't work");
        }
    }

    #[test]
    fn test_word_from_bytes() {}
}
// TODO: add tests
