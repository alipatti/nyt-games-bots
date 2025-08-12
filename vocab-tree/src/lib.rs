#[derive(Debug)]
struct TrieNode<K, V> {
    /// `None` if root
    key: Option<K>,
    value: Option<V>,
    children: Vec<TrieNode<K, V>>,
}

impl<K, V> TrieNode<K, V>
where
    K: PartialEq + Clone,
{
    /// Returns the old value if it exists
    fn insert(
        &mut self,
        mut key: impl Iterator<Item = K>,
        value: V,
    ) -> Option<V> {
        match key.next() {
            // end of key, so insert value here
            None => self.value.replace(value),
            // recurse on the correct child
            Some(k) => {
                match self.children.iter_mut().find(|child| {
                    *child
                        .key
                        .as_ref()
                        .expect("guaranteed to be Some for non-root")
                        == k
                }) {
                    Some(child) => child.insert(key, value),
                    None => {
                        let mut new_node = TrieNode {
                            key: Some(k.clone()),
                            value: None,
                            children: vec![],
                        };
                        let result = new_node.insert(key, value);
                        self.children.push(new_node);
                        result
                    }
                }
            }
        }
    }

    // TODO: can we just use search for this?
    fn remove(&mut self, mut key: impl Iterator<Item = K>) -> Option<V> {
        match key.next() {
            None => self.value.take(),
            Some(k) => {
                if let Some(pos) = self.children.iter().position(|child| {
                    *child
                        .key
                        .as_ref()
                        .expect("guaranteed to be Some for non-root")
                        == k
                }) {
                    let child = &mut self.children[pos];
                    let removed = child.remove(key);

                    if child.value.is_none() && child.children.is_empty() {
                        self.children.remove(pos);
                    }

                    removed
                } else {
                    None
                }
            }
        }
    }

    /// Recursively searches and finds the descendent that matches a particular query.
    fn search(&self, mut query: impl Iterator<Item = K>) -> Option<&Self> {
        match query.next() {
            Some(k) => self
                .children
                .iter()
                .find(|child| {
                    *child
                        .key
                        .as_ref()
                        .expect("guaranteed to be Some for non-root")
                        == k
                })
                .and_then(|child| child.search(query)),
            None => Some(self),
        }
    }

    /// Recursively searches and finds the descendent that matches a particular query.
    /// None values in the query indicate that the query should match all children.
    /// PERF: can we get rid of the dynamic dispach?
    fn search_pattern<'a>(
        &'a self,
        mut query: impl Iterator<Item = Option<K>> + Clone + 'a,
    ) -> Box<dyn Iterator<Item = &'a Self> + 'a> {
        match query.next() {
            // key given
            Some(maybe_key) => Box::new(
                self.children
                    .iter()
                    .filter(move |child| {
                        maybe_key.as_ref().map_or(true, |key| {
                            key == child
                                .key
                                .as_ref()
                                .expect("guaranteed to be Some for non-root")
                        })
                    })
                    .flat_map(move |child| child.search_pattern(query.clone())),
            ),
            None => Box::new(std::iter::once(self)),
        }
    }

    // fn traceback(&self) -> impl Iterator<Item = K> {
    //
    // }
}

pub struct Trie<K, V> {
    root: TrieNode<K, V>,
}

impl<K: Clone + PartialEq, V> Trie<K, V> {
    pub fn empty() -> Self {
        Self {
            root: TrieNode {
                key: None,
                value: None,
                children: vec![],
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty()
    }

    /// Returns the old value if it exists
    pub fn insert(
        &mut self,
        key: impl IntoIterator<Item = K>,
        value: V,
    ) -> Option<V> {
        self.root.insert(key.into_iter(), value)
    }

    /// Returns the old value if it exists
    pub fn remove(&mut self, key: impl IntoIterator<Item = K>) -> Option<V> {
        self.root.remove(key.into_iter())
    }

    pub fn new<T>(
        key_value_pairs: impl IntoIterator<Item = (impl IntoIterator<Item = K>, V)>,
    ) -> Self {
        let mut trie = Trie::empty();

        for (key, value) in key_value_pairs {
            trie.insert(key, value);
        }

        trie
    }

    pub fn get(&mut self, query: impl IntoIterator<Item = K>) -> Option<&V> {
        self.root
            .search(query.into_iter())
            .and_then(|n| n.value.as_ref())
    }

    pub fn contains_prefix(
        &mut self,
        query: impl IntoIterator<Item = K>,
    ) -> bool {
        self.root.search(query.into_iter()).is_some()
    }

    // pub fn get_matches<Q, T>(
    //     &mut self,
    //     query: Q,
    // ) -> impl Iterator<Item = (T, &V)>
    // where
    //     Q: IntoIterator<Item = Option<K>>,
    //     <Q as IntoIterator>::IntoIter: Clone,
    //     T: Iterator<Item = K>
    // {
    //     let x = self.root.search_pattern(query.into_iter());
    //
    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_trie() {
        let trie: Trie<char, &str> = Trie::empty();
        assert!(trie.is_empty());
    }

    #[test]
    fn test_insert_and_get() {
        let mut trie = Trie::empty();
        trie.insert("cat".chars(), "meow");
        assert_eq!(trie.get("cat".chars()), Some(&"meow"));
    }

    #[test]
    fn test_insert_two_remove_one() {
        let mut trie = Trie::empty();
        trie.insert("cat".chars(), "meow");
        trie.insert("dog".chars(), "woof");

        trie.remove("cat".chars());

        assert_eq!(trie.get("cat".chars()), None);
        assert_eq!(trie.get("dog".chars()), Some(&"woof"));
    }

    #[test]
    fn test_insert_two_remove_two() {
        let mut trie = Trie::empty();
        trie.insert("cat".chars(), "meow");
        trie.insert("dog".chars(), "woof");

        trie.remove("cat".chars());
        trie.remove("dog".chars());

        assert!(trie.is_empty());
    }
}
