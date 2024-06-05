//! Mapping queue that is used for conversions.

use std::borrow::Cow;

/// Queue of mappings from a pattern to a constant replacement string.
/// Mappings are applied in FIFO order to the input string.
///
/// Applying a mapping (pattern → replacement) to an input string means
/// to replace all occurrences of the pattern with the replacement string.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct MappingQueue<'a> {
    /// Queue of mappings.
    queue: Vec<(&'a str, &'a str)>,
}

impl<'a> MappingQueue<'a> {
    /// Create an empty mapping queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a mapping to the back of the queue.
    pub fn push(mut self, from: &'a str, to: &'a str) -> Self {
        self.queue.push((from, to));
        self
    }

    /// Extend the back of the queue with an iterator.
    pub fn extend<I: Iterator<Item = (&'a str, &'a str)>>(mut self, it: I) -> Self {
        self.queue.extend(it);
        self
    }

    /// Apply each queued mapping in FIFO order.
    pub fn apply(&self, input: &str) -> String {
        let mut output = Cow::Borrowed(input);
        for (from, to) in self.queue.iter() {
            output = Cow::Owned(output.replace(from, to));
        }
        output.into_owned()
    }
}

/// Iterate over each character of the string as a `&str` slice.
fn char_slices(s: &str) -> impl Iterator<Item = &str> {
    s.char_indices().map(|(i, c)| &s[i..i + c.len_utf8()])
}

/// Create mappings (pattern → replacement) from the characters of two strings.
///
/// The first string lists pattern characters.
/// The second string lists replacement characters.
/// The first pattern character is translated to the first replacement character, and so on.
///
/// ## Panics
///
/// The two strings are not of the same length.
pub fn char_mappings<'a>(from: &'a str, to: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> {
    assert_eq!(
        from.chars().count(),
        to.chars().count(),
        "There must be as many pattern characters as replacement characters"
    );
    char_slices(from).zip(char_slices(to))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "There must be as many pattern characters as replacement characters")]
    fn char_mappings_unequal_len() {
        let _ = char_mappings("", "a");
    }
}
