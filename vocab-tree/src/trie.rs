use std::fmt::Debug;

use crate::node::{Key, Node};

#[derive(Debug)]
pub struct Trie<K>(Node<K>);

impl<K> Trie<K>
where
    K: Ord + Clone + Debug,
{
    pub fn new() -> Self {
        Self(Node::root())
    }

    pub fn push(&mut self, value: impl IntoIterator<Item = K>, cost: usize) {
        self.0.push(&Self::make_query(value), cost);
    }

    pub fn cost(
        &mut self,
        value: impl IntoIterator<Item = K>,
    ) -> Option<usize> {
        self.0
            .find_descendent(&Self::make_query(value))
            .map(|n| n.cost().expect("node must be terminal"))
    }

    fn make_query(value: impl IntoIterator<Item = K>) -> Vec<Key<K>> {
        value
            .into_iter()
            .map(|v| Key::Internal(v))
            .chain(std::iter::once(Key::End))
            .collect()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Vec<K>> + 'a {
        self.0.iter_descendents(None).filter_map(|(a, _)| {
            if let [None, middle @ .., None] = &a[..] {
                Some(
                    middle
                        .iter()
                        .filter_map(|x| x.cloned())
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_trie_does_not_contain_anything() {
        let mut trie: Trie<char> = Trie::new();
        assert!(trie.cost("a".chars()).is_none());
        assert!(trie.cost("test".chars()).is_none());
    }

    #[test]
    fn test_insert_and_query_single_word() {
        let mut trie: Trie<char> = Trie::new();
        trie.push("hello".chars(), 1);
        assert_eq!(trie.cost("hello".chars()), Some(1));
        assert!(trie.cost("hell".chars()).is_none());
        assert!(trie.cost("helloo".chars()).is_none());
    }

    #[test]
    fn test_insert_multiple_words() {
        let mut trie: Trie<char> = Trie::new();
        trie.push("foo".chars(), 1);
        trie.push("bar".chars(), 2);
        trie.push("baz".chars(), 3);

        assert_eq!(trie.cost("foo".chars()), Some(1));
        assert_eq!(trie.cost("bar".chars()), Some(2));
        assert_eq!(trie.cost("baz".chars()), Some(3));

        assert!(trie.cost("ba".chars()).is_none());
        assert!(trie.cost("foobar".chars()).is_none());
    }

    #[test]
    fn test_iter() {
        let mut trie: Trie<char> = Trie::new();
        trie.push("car".chars(), 5);
        trie.push("cap".chars(), 6);

        // trie.push("carthage".chars(), 1);
        // trie.push("captive".chars(), 2);

        let x: Vec<_> = trie.iter().collect();

        dbg!(&x);
        // dbg!(&trie);
    }
}
