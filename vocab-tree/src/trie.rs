use std::fmt::Debug;

use crate::{
    node::{Key, Node},
    traversals::Query,
};

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

    pub fn iter<'a>(
        &'a self,
        pattern: Option<&'a [Option<K>]>,
    ) -> impl Iterator<Item = Vec<K>> + 'a {
        let pattern = pattern.map(|p| {
            std::iter::once(Query::Matches(Key::Start))
                .chain(p.iter().map(|maybe_key| match maybe_key {
                    Some(k) => Query::Matches(Key::Internal(k.clone())),
                    None => Query::Any,
                }))
                .chain(std::iter::once(Query::Matches(Key::End)))
                .collect::<Vec<_>>()
        });

        self.0.iter_descendents(pattern).filter_map(|(a, _)| {
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
        let mut words: Vec<Vec<char>> =
            vec!["car", "cap", "carthage", "captive"]
                .iter()
                .map(|x| x.chars().collect())
                .collect();

        let mut trie: Trie<char> = Trie::new();
        for (i, w) in words.iter().enumerate() {
            trie.push(w.clone(), i);
        }

        let mut matches: Vec<Vec<char>> = trie.iter(None).collect();

        words.sort();
        matches.sort();

        assert_eq!(matches, words);
    }

    #[test]
    fn test_iter_pattern() {
        let words: Vec<Vec<char>> =
            vec!["car", "cap", "cop", "carthage", "captive"]
                .iter()
                .map(|x| x.chars().collect())
                .collect();

        let mut trie: Trie<char> = Trie::new();
        for (i, w) in words.iter().enumerate() {
            trie.push(w.clone(), i);
        }

        let pattern = vec![Some('c'), None, Some('p')];

        let mut matches: Vec<Vec<char>> = trie.iter(Some(&pattern)).collect();
        matches.sort();

        assert_eq!(matches, vec![vec!['c', 'a', 'p'], vec!['c', 'o', 'p']]);
    }
}
