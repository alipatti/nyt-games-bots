use std::fmt::Debug;

use crate::traversal::{Pattern, TrieDfsTraversal};

#[derive(Debug)]
pub struct Trie<K, V> {
    nodes: Vec<Node<K, V>>,
}

#[derive(Debug)]
pub(crate) struct Node<K, V> {
    key: Key<K>,
    // TODO: remove the option. this would require us to fix empty trie creation
    // (can we defer pushing the root until we add the first element?)
    min_descendent: Option<V>,
    children: Vec<usize>,
    /// INVARIANT `None` iff trie is empty
    parent: Option<usize>,
}

impl<K, V> Node<K, V> {
    fn new(parent: Option<usize>, key: Key<K>) -> Self {
        Self {
            children: Vec::new(),
            min_descendent: None,
            parent,
            key,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum Key<K> {
    Start,
    Internal(K),
    End,
}

impl<K, V> Default for Trie<K, V>
where
    K: Debug + Clone + Eq,
    V: Debug + Clone + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Trie<K, V>
where
    // TODO: split up impls don't really need ord here.
    K: Debug + Clone + Eq,
    V: Debug + Clone + Ord,
{
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn push(&mut self, keys: impl IntoIterator<Item = K>, value: V) {
        // push root if this the first element
        if self.nodes.is_empty() {
            self.nodes.push(Node::new(None, Key::Start));
        }

        // get the root
        let mut current_index = 0;

        for key in keys {
            // PERF: unnecessary clone here
            // change the function to take Option<&K>?
            current_index = self.get_or_create_child_index(
                current_index,
                Key::Internal(key.clone()),
            );
            self.update_min_descendent(current_index, &value);
        }

        current_index = self.get_or_create_child_index(current_index, Key::End);
        self.update_min_descendent(current_index, &value);
    }

    pub fn get(
        &self,
        keys: impl IntoIterator<Item = impl Into<Key<K>>>,
    ) -> Option<&V> {
        let keys = keys
            .into_iter()
            .map(|k| k.into())
            .chain(std::iter::once(Key::End));

        self.get_node_index(keys)
            .map(|i| self.nodes[i].min_descendent.as_ref().expect("guaranteed"))
    }

    /// Returns the index of a child, creating the child if it doesn't exist.
    fn get_or_create_child_index(
        &mut self,
        parent_index: usize,
        child_key: Key<K>,
    ) -> usize {
        // get the index of the child if it exists
        let maybe_index = self.nodes[parent_index]
            .children
            .iter()
            .find(|&i| child_key == self.nodes[*i].key);

        match maybe_index {
            Some(index) => *index,
            None => {
                let child = Node::new(Some(parent_index), child_key);
                self.nodes.push(child);

                let child_index = self.nodes.len() - 1;
                self.nodes[parent_index].children.push(child_index);

                child_index
            }
        }
    }

    fn update_min_descendent(&mut self, index_to_update: usize, value: &V) {
        let node = self.nodes.get_mut(index_to_update).unwrap();

        match &node.min_descendent {
            None => node.min_descendent = Some(value.clone()),
            Some(current_value) if current_value < value => {
                node.min_descendent = Some(value.clone())
            }
            _ => {} // current value is smaller
        }
    }

    /// Returns the index of a child
    pub(crate) fn get_child_index(
        &self,
        parent_index: usize,
        child_key: &Key<K>,
    ) -> Option<usize> {
        // get the index of the child if it exists
        self.nodes[parent_index]
            .children
            .iter()
            .find(|&i| *child_key == self.nodes[*i].key)
            .copied()
    }

    pub(crate) fn children(&self, parent_index: usize) -> &[usize] {
        &self.nodes[parent_index].children
    }

    pub(crate) fn get_node_index(
        &self,
        keys: impl IntoIterator<Item = Key<K>>,
    ) -> Option<usize> {
        let mut current_index = 0;

        for k in keys {
            match self.get_child_index(dbg!(current_index), &k) {
                Some(child_index) => {
                    current_index = child_index;
                }
                None => return None,
            }
        }

        Some(current_index)
    }

    pub fn iter_values_unordered(
        &self,
        pattern: Option<Pattern<K>>,
    ) -> impl Iterator<Item = (impl Iterator<Item = &K>, &V)> {
        self.iter_nodes_unordered(pattern)
            .filter_map(|node| match node.key {
                Key::End => Some((
                    self.path_to_root(node),
                    node.min_descendent.as_ref().expect("guaranteed to exist"),
                )),
                _ => None,
            })
    }

    fn iter_nodes_unordered(
        &self,
        pattern: Option<Pattern<K>>,
    ) -> impl Iterator<Item = &Node<K, V>> {
        TrieDfsTraversal::from_root(self, pattern).map(|i| &self.nodes[i])
    }

    fn path_to_root<'a>(
        &'a self,
        node: &'a Node<K, V>,
    ) -> impl Iterator<Item = &'a K> {
        let mut current = node;

        std::iter::from_fn(move || match current.parent {
            Some(parent_index) => {
                current = &self.nodes[parent_index]; // go to parent

                match &current.key {
                    Key::Internal(k) => Some(k),
                    _ => None,
                }
            }
            None => None, // at the root
        })
    }
}

impl<K> From<K> for Key<K> {
    fn from(value: K) -> Self {
        Key::Internal(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_trie() {
        let trie: Trie<char, ()> = Trie::new();

        assert_eq!(
            trie.nodes.iter().map(|n| n.key.clone()).collect::<Vec<_>>(),
            vec![]
        );
    }

    #[test]
    fn test_single_string() {
        let mut trie = Trie::new();
        trie.push("hi!".chars(), ());

        assert_eq!(
            trie.nodes.iter().map(|n| n.key.clone()).collect::<Vec<_>>(),
            vec![
                Key::Start,
                Key::Internal('h'),
                Key::Internal('i'),
                Key::Internal('!'),
                Key::End,
            ]
        );
    }

    #[test]
    fn test_multiple_strings() {
        let mut trie = Trie::new();
        trie.push("car".chars(), 1);
        trie.push("cat".chars(), 2);

        assert_eq!(trie.nodes.len(), 1 + (3 + 1) + (1 + 1));

        assert_eq!(trie.get("car".chars()), Some(&1));
        assert_eq!(trie.get("cat".chars()), Some(&2));

        assert!(trie.get("dog".chars()).is_none());

        assert!(trie.get("c".chars()).is_none());
        assert!(trie.get("ca".chars()).is_none());
    }

    #[test]
    fn test_iter_unordered() {
        let mut trie = Trie::new();
        let words = vec!["antidisestablishmentarianism", "cab", "car", "cat"];

        for word in &words {
            trie.push(word.chars(), ());
        }

        // awkward reversal so that the prefix is in the right order
        let mut should_be_words = trie
            .iter_values_unordered(None)
            .map(|(prefix, _)| {
                prefix
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        should_be_words.sort();

        assert_eq!(words, should_be_words);
    }
}
