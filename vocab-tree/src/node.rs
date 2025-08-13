use std::fmt::Debug;

use crate::traversals::{DfsTraversal, Query};

#[derive(Debug)]
pub(crate) struct Node<K> {
    pub(crate) contents: Key<K>,
    pub(crate) children: Children<K>,
    min_subtree_cost: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum Key<K> {
    Start,
    Internal(K),
    End,
}

#[derive(Debug)]
pub(crate) struct Children<K>(Vec<Node<K>>);

impl<K: Debug + Clone + Ord> Node<K> {
    /// Create a root node with no children and maximum cost.
    pub(crate) fn root() -> Self {
        Self::with_contents(Key::Start)
    }

    fn with_contents(contents: Key<K>) -> Self {
        Self {
            min_subtree_cost: usize::MAX,
            contents,
            children: Children(Vec::new()),
        }
    }

    pub(crate) fn is_terminal(&self) -> bool {
        match &self.contents {
            Key::End => true,
            _ => false,
        }
    }

    pub(crate) fn key(&self) -> Option<&K> {
        return match &self.contents {
            Key::Internal(k) => Some(k),
            _ => None,
        };
    }

    /// Returns Some(cost) if node is terminal, `None` otherwise.
    pub(crate) fn cost(&self) -> Option<usize> {
        match self.contents {
            Key::End => Some(self.min_subtree_cost),
            _ => None,
        }
    }

    pub(crate) fn iter_children(&self) -> impl Iterator<Item = &'_ Node<K>> {
        self.children.into_iter()
    }

    pub(crate) fn iter_descendents<'a>(
        &'a self,
        pattern: Option<Vec<Query<K>>>,
    ) -> impl Iterator<Item = (Vec<Option<&'a K>>, &'a Node<K>)> {
        DfsTraversal::new(&self, pattern)
    }

    /// Adds a node at the given suffix with the given cost.
    pub(crate) fn push(&mut self, suffix: &[Key<K>], cost: usize) -> &Self {
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

impl<'a, K> IntoIterator for &'a Children<K> {
    type Item = &'a Node<K>;

    type IntoIter = <&'a Vec<Node<K>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<K: Ord + Clone + Debug> Children<K> {
    /// Gets the child if it exists.
    pub(crate) fn get(&self, key: &Key<K>) -> Option<&Node<K>> {
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
