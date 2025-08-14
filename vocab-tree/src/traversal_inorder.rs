use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;

use crate::traversal::Pattern;
use crate::trie::{Key, Node, Trie};

#[derive(Debug)]
pub(crate) struct DijkstraTraversal<'a, K, V> {
    trie: &'a Trie<K, V>,
    heap: BinaryHeap<HeapItem<&'a V>>, // cost, index, depth
    pattern: Option<Pattern<K>>,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct HeapItem<V> {
    value: Reverse<V>,
    index: usize,
    depth: usize,
}

impl<V> HeapItem<V> {
    fn new(value: V, index: usize, depth: usize) -> Self {
        Self {
            value: Reverse(value),
            index,
            depth,
        }
    }
}

impl<'a, K, V: Ord> DijkstraTraversal<'a, K, V> {
    pub(crate) fn from_root(
        trie: &'a Trie<K, V>,
        pattern: Option<Pattern<K>>,
    ) -> Self {
        Self::from_index(trie, 0, pattern)
    }

    pub(crate) fn from_index(
        trie: &'a Trie<K, V>,
        start_index: usize,
        pattern: Option<Pattern<K>>,
    ) -> Self {
        let mut heap = BinaryHeap::new();
        if let Some(root) = trie.root() {
            heap.push(HeapItem::new(root.value(), start_index, 0));
        }

        Self {
            trie,
            heap,
            pattern,
        }
    }
}

impl<'a, K, V> Iterator for DijkstraTraversal<'a, K, V>
where
    K: PartialEq,
    V: Debug + Ord,
{
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(HeapItem { index, depth, .. }) = self.heap.pop() {
            let remainig_pattern =
                self.pattern.as_ref().map(|p| &p[depth + 1..]);

            // push the approiate children based on the remaining pattern
            match remainig_pattern {
                // no pattern OR next part of pattern is `None`:
                // push all children
                None | Some([None, ..]) => self.heap.extend(
                    self.trie.child_indices(index).iter().map(|&child_index| {
                        HeapItem::new(
                            self.trie.node(child_index).value(),
                            child_index,
                            depth + 1,
                        )
                    }),
                ),
                // next part of pattern is `Some`:
                // push the matching child (if it exists)
                Some([Some(k), ..]) => {
                    if let Some(child_index) =
                        self.trie.get_child_index(index, k)
                    {
                        self.heap.push(HeapItem::new(
                            self.trie.node(child_index).value(),
                            child_index,
                            depth + 1,
                        ));
                    }
                }
                // end of pattern. don't descend further
                Some([]) => {}
            }

            // return the current index
            Some(self.trie.node(index))
        } else {
            // exhausted tree
            None
        }
    }
}

/// ordered iteration over nodes
impl<K, V> Trie<K, V>
where
    K: PartialEq,
    V: Debug + Ord,
{
    pub fn iter_values_ordered(
        &self,
        pattern: Option<Pattern<K>>,
    ) -> impl Iterator<Item = (impl Iterator<Item = &K>, &V)> {
        DijkstraTraversal::from_root(self, pattern).filter_map(
            |node| match node.key() {
                Key::End => Some((self.path_to_root(node), node.value())),
                _ => None,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_iter_ordered() {
        let mut trie = Trie::new();

        let words = vec!["bad", "shazam", "baz", "bat", "bark", "zebra"];
        let sorted_by_last_letter =
            vec!["zebra", "bad", "bark", "shazam", "bat", "baz"];

        for word in &words {
            trie.push(word.chars(), word.chars().last().unwrap());
        }

        // awkward reversal so that the prefix is in the right order
        let returned_by_iterator = trie
            .iter_values_ordered(None)
            .map(|(prefix, _)| {
                prefix
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        assert_eq!(sorted_by_last_letter, returned_by_iterator);
    }

    /// check if the iterator is sorted
    #[quickcheck]
    fn test_iter_inorder_2(unique_keys: HashSet<Vec<u8>>) {
        let mut trie = Trie::new();
        for key in &unique_keys {
            trie.push(key.clone(), key.last());
        }

        let iterator_sorted =
            trie.iter_values_ordered(None).is_sorted_by_key(|(_, v)| v);

        assert!(iterator_sorted);
    }
}
