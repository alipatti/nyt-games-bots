use std::fmt::Debug;

use crate::traversals::{DfsTraversal, Query};

#[derive(Debug)]
pub(crate) struct Node<K, V> {
    pub(crate) contents: Key<K>,
    pub(crate) children: Children<K, V>,
    /// INVARIANT: `None` iff trie is empty
    min_descendent: Option<V>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum Key<K> {
    Start,
    Internal(K),
    End,
}

#[derive(Debug)]
pub(crate) struct Children<K, V>(Vec<Node<K, V>>);

impl<K: Debug + Clone + Ord, V: Debug + Ord + Clone> Node<K, V> {
    pub(crate) fn new() -> Self {
        Self::with_contents(Key::Start)
    }

    fn with_contents(contents: Key<K>) -> Self {
        Self {
            min_descendent: None,
            contents,
            children: Children(Vec::new()),
        }
    }

    pub(crate) fn key(&self) -> Option<&K> {
        match &self.contents {
            Key::Internal(k) => Some(k),
            _ => None,
        }
    }

    /// Returns Some(v) if node is terminal, `None` otherwise.
    pub(crate) fn value(&self) -> Option<&V> {
        match &self.contents {
            Key::End => self.min_descendent.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn iter_children(&self) -> impl Iterator<Item = &'_ Node<K, V>> {
        self.children.into_iter()
    }

    pub(crate) fn iter_descendents(
        &self,
        pattern: Option<Vec<Query<K>>>,
    ) -> impl Iterator<Item = (Vec<Option<&K>>, &Node<K, V>)> {
        DfsTraversal::new(self, pattern)
    }

    /// Adds a node at the given suffix with the given value
    pub(crate) fn push(&mut self, suffix: &[Key<K>], value: V) -> &Self {
        if self.min_descendent.is_none() {
            // we're at the root
            self.min_descendent = Some(value.clone());
        } else if value < *self.min_descendent.as_ref().unwrap() {
            // we're inserting a smaller value
            self.min_descendent = Some(value.clone())
        }

        if let [first, rest @ ..] = suffix {
            // get child and recurse
            self.children.get_or_create(first).push(rest, value)
        } else {
            // we're done (suffix is empty slice)
            self
        }
    }

    /// Returns `Some(node)` if `self` contains a descendent at the address `suffix`.
    /// Returns `None` if not.
    pub(crate) fn find_descendent(&self, suffix: &[Key<K>]) -> Option<&Self> {
        if let [first, rest @ ..] = suffix {
            self.children
                .get(first)
                .and_then(|n| n.find_descendent(rest))
        } else {
            Some(self) // found it!
        }
    }
}

impl<'a, K, V> IntoIterator for &'a Children<K, V> {
    type Item = &'a Node<K, V>;

    type IntoIter = <&'a Vec<Node<K, V>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<K: Ord + Clone + Debug, V: Debug + Ord + Clone> Children<K, V> {
    /// Gets the child if it exists.
    pub(crate) fn get(&self, key: &Key<K>) -> Option<&Node<K, V>> {
        match self.0.binary_search_by_key(
            key,
            |n| n.contents.clone(), // PERF: clone here isn't ideal
        ) {
            Ok(index) => self.0.get(index),
            Err(_) => None,
        }
    }

    /// Gets the child if it exists. Creates it if not.
    fn get_or_create(&mut self, key: &Key<K>) -> &mut Node<K, V> {
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
