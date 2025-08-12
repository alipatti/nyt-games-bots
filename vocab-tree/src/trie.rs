use crate::node::{Key, Node};

pub struct Trie<K>(Node<K>);

enum Query<K> {
    Matches(Key<K>),
    AnySingle,
    // AnyMultiple,
}

impl<K> Trie<K>
where
    K: Ord + Clone,
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

    // #[test]
    // fn test_shared_prefixes() {
    //     let mut trie: Trie<char> = Trie::new();
    //     trie.push("car".chars(), 1);
    //     trie.push("cart".chars(), 2);
    //
    //     assert!(trie.find("car".chars()));
    //     assert!(trie.find("cart".chars()));
    //     assert!(!trie.find("ca".chars()));
    //     assert!(!trie.find("cars".chars()));
    // }
}
