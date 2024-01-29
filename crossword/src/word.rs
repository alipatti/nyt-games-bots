/// A word represented as an array of uppercase ascii bytes
pub(super) struct Word {
    chars: Box<[u8]>,
}

impl Word {
    pub(crate) fn chars(&self) -> impl Iterator<Item = &u8> {
        self.chars.into_iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.chars.len()
    }
}

impl TryFrom<&[u8]> for Word {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let is_valid =
            value
                .to_ascii_uppercase()
                .into_iter()
                .all(|byte| match byte {
                    41..=90 => true, // ASCII uppercase range
                    _ => false,
                });

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
// TODO: add tests

