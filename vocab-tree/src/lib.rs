pub struct Trie<K>(Node<K>);

#[derive(Debug)]
struct Node<K> {
    contents: Key<K>,
    min_subtree_cost: usize,
    children: Children<K>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Key<K> {
    Start,
    Internal(K),
    End,
}

enum Query<K> {
    Matches(Key<K>),
    AnySingle,
    // AnyMultiple,
}

#[derive(Debug)]
struct Children<K>(Vec<Node<K>>);

impl<K> Trie<K>
where
    K: Ord + Clone,
{
    pub fn new() -> Self {
        Self(Node::empty())
    }

    pub fn push(&mut self, value: impl IntoIterator<Item = K>, cost: usize) {
        self.0.push(&Self::make_query(value), cost);
    }

    pub fn find(
        &mut self,
        value: impl IntoIterator<Item = K>,
    ) -> Option<usize> {
        self.0
            .find(&Self::make_query(value))
            .map(|n| n.min_subtree_cost)
    }

    fn make_query(value: impl IntoIterator<Item = K>) -> Vec<Key<K>> {
        value
            .into_iter()
            .map(|v| Key::Internal(v))
            .chain(std::iter::once(Key::End))
            .collect()
    }
}

impl<K> Node<K>
where
    K: Ord + Clone,
{
    fn empty() -> Self {
        Self {
            min_subtree_cost: usize::MAX,
            contents: Key::Start,
            children: Children(Vec::new()),
        }
    }

    fn with_contents(contents: Key<K>) -> Self {
        Self {
            min_subtree_cost: usize::MAX,
            contents,
            children: Children(Vec::new()),
        }
    }

    fn push(&mut self, suffix: &[Key<K>], cost: usize) -> &Self {
        // overwrite with new cost if it's lower
        self.min_subtree_cost = self.min_subtree_cost.min(cost);

        if let [first, rest @ ..] = suffix {
            // get child and recurse
            self.children.get_or_create(first).push(rest, cost)
        } else {
            // we're done (suffix is empty slice)
            self
        }
    }

    fn find(&self, suffix: &[Key<K>]) -> Option<&Self> {
        if let [first, rest @ ..] = suffix {
            self.children.get(first).and_then(|n| n.find(rest))
        } else {
            Some(self) // found it!
        }
    }
}

impl<K> IntoIterator for Children<K> {
    type Item = Node<K>;

    type IntoIter = <Vec<Node<K>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K: Ord + Clone> Children<K> {
    /// Gets the child if it exists.
    fn get(&self, key: &Key<K>) -> Option<&Node<K>> {
        match self.0.binary_search_by_key(
            key,
            |n| n.contents.clone(), // PERF: clone here isn't ideal
        ) {
            Ok(index) => self.0.get(index),
            Err(_) => None,
        }
    }

    /// Gets the child if it exists. Creates it if not.
    fn get_or_create(&mut self, key: &Key<K>) -> &mut Node<K> {
        // PERF: linear search may honestly be better here
        // (or hash map? should we let the user choose?)
        match self.0.binary_search_by_key(
            key,
            |n| n.contents.clone(), // PERF: clone here isn't ideal
        ) {
            // exists, so return it
            Ok(index) => self.0.get_mut(index),
            // doesn't exist, so create and return it
            Err(index) => {
                let child = Node::with_contents(key.clone());
                self.0.insert(index, child);
                self.0.get_mut(index)
            }
        }
        .expect("entry is guaranteed to exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_trie_does_not_contain_anything() {
        let mut trie: Trie<char> = Trie::new();
        assert!(trie.find("a".chars()).is_none());
        assert!(trie.find("test".chars()).is_none());
    }

    #[test]
    fn test_insert_and_query_single_word() {
        let mut trie: Trie<char> = Trie::new();
        trie.push("hello".chars(), 1);
        assert_eq!(trie.find("hello".chars()), Some(1));
        assert!(trie.find("hell".chars()).is_none());
        assert!(trie.find("helloo".chars()).is_none());
    }

    #[test]
    fn test_insert_multiple_words() {
        let mut trie: Trie<char> = Trie::new();
        trie.push("foo".chars(), 1);
        trie.push("bar".chars(), 2);
        trie.push("baz".chars(), 3);

        assert_eq!(trie.find("foo".chars()), Some(1));
        assert_eq!(trie.find("bar".chars()), Some(2));
        assert_eq!(trie.find("baz".chars()), Some(3));

        assert!(trie.find("ba".chars()).is_none());
        assert!(trie.find("foobar".chars()).is_none());
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
