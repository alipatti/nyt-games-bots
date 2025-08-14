use std::fmt::Debug;

#[derive(Debug)]
pub struct Trie<K, V> {
    nodes: Vec<Node<K, V>>,
}

#[derive(Debug)]
pub(crate) struct Node<K, V> {
    key: Key<K>,
    min_descendent: V,
    children: Vec<usize>,
    /// INVARIANT `None` if root
    parent: Option<usize>,
}

impl<K, V> Node<K, V> {
    fn new(parent: Option<usize>, key: Key<K>, value: V) -> Self {
        Self {
            children: Vec::new(),
            min_descendent: value,
            parent,
            key,
        }
    }

    fn root(value: V) -> Self {
        Self::new(None, Key::Start, value)
    }

    pub(crate) fn key(&self) -> &Key<K> {
        &self.key
    }

    pub(crate) fn value(&self) -> &V {
        &self.min_descendent
    }

    pub(crate) fn parent<'a>(
        &'a self,
        trie: &'a Trie<K, V>,
    ) -> Option<&'a Node<K, V>> {
        self.parent.map(move |i| trie.node(i))
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

impl<K, V> Trie<K, V> {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub(crate) fn node(&self, index: usize) -> &Node<K, V> {
        &self.nodes[index]
    }

    /// used in the traversals
    pub(crate) fn child_indices(&self, parent_index: usize) -> &[usize] {
        &self.node(parent_index).children
    }
}

impl<K, V> Trie<K, V>
where
    K: PartialEq,
{
    /// retrieve the value of a particular key sequence
    pub fn get(
        &self,
        keys: impl IntoIterator<Item = impl Into<Key<K>>>,
    ) -> Option<&V> {
        let keys = keys
            .into_iter()
            .map(|k| k.into())
            .chain(std::iter::once(Key::End));

        self.get_node_index(keys)
            .map(|i| &self.node(i).min_descendent)
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

    /// Returns the index of a child
    pub(crate) fn get_child_index(
        &self,
        parent_index: usize,
        child_key: &Key<K>,
    ) -> Option<usize> {
        // get the index of the child if it exists
        self.node(parent_index)
            .children
            .iter()
            .find(|&i| *child_key == self.node(*i).key)
            .copied()
    }
}

impl<K, V> Trie<K, V>
where
    K: Clone + PartialEq,
    V: Clone + Ord,
{
    pub fn push(&mut self, keys: impl IntoIterator<Item = K>, value: V) {
        // push root if this the first element
        if self.nodes.is_empty() {
            self.nodes.push(Node::root(value.clone()));
        }

        // get the root
        let mut current_index = 0;

        for key in keys {
            // PERF: unnecessary clone here
            // change the function to take Option<&K>?
            current_index = self.descend_to_child(
                current_index,
                Key::Internal(key.clone()),
                &value,
            );
        }

        // add a special `Key::End` value to indicate that this is the end of a word
        self.descend_to_child(current_index, Key::End, &value);
    }

    /// Returns the index of a child, creating the child if it doesn't exist.
    /// Updates `min_descendent` of the child.
    fn descend_to_child(
        &mut self,
        parent_index: usize,
        child_key: Key<K>,
        child_value: &V,
    ) -> usize {
        // get the index of the child if it exists
        let maybe_index = self.nodes[parent_index]
            .children
            .iter()
            .find(|&i| child_key == self.nodes[*i].key);

        let child_index = match maybe_index {
            Some(index) => *index,
            None => {
                let child = Node::new(
                    Some(parent_index),
                    child_key,
                    child_value.clone(),
                );
                self.nodes.push(child);

                let child_index = self.nodes.len() - 1;
                self.nodes[parent_index].children.push(child_index);

                child_index
            }
        };

        let node = self.nodes.get_mut(child_index).unwrap();

        if node.min_descendent < *child_value {
            node.min_descendent = child_value.clone();
        }

        child_index
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
}
