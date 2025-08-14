use std::fmt::Debug;

use crate::trie::{Key, Node, Trie};

#[derive(Debug)]
pub(crate) struct TrieDfsTraversal<'a, K, V> {
    trie: &'a Trie<K, V>,
    stack: Vec<(usize, usize)>, // index, depth
    pattern: Option<Pattern<K>>,
}

pub(crate) type Pattern<K> = Vec<Query<K>>;
pub(crate) type Query<K> = Option<Key<K>>;

impl<'a, K, V> TrieDfsTraversal<'a, K, V> {
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
        Self {
            trie,
            stack: vec![(start_index, 0)],
            pattern,
        }
    }
}

impl<'a, K, V> Iterator for TrieDfsTraversal<'a, K, V>
where
    K: PartialEq,
{
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, depth)) = self.stack.pop() {
            let remainig_pattern =
                self.pattern.as_ref().map(|p| &p[depth + 1..]);

            // push the approiate children based on the remaining pattern
            match remainig_pattern {
                // no pattern OR next part of pattern is `None`:
                // push all children
                None | Some([None, ..]) => self.stack.extend(
                    self.trie.child_indices(index).iter().map(|i| (*i, depth + 1)),
                ),
                // next part of pattern is `Some`:
                // push the matching child (if it exists)
                Some([Some(k), ..]) => {
                    if let Some(child_index) =
                        self.trie.get_child_index(index, k)
                    {
                        self.stack.push((child_index, depth + 1));
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

/// unordered iteration over nodes
impl<K, V> Trie<K, V>
where
    K: PartialEq,
{
    pub fn iter_values_unordered(
        &self,
        pattern: Option<Pattern<K>>,
    ) -> impl Iterator<Item = (impl Iterator<Item = &K>, &V)> {
        self.iter_nodes_unordered(pattern)
            .filter_map(|node| match node.key() {
                Key::End => Some((self.path_to_root(node), node.value())),
                _ => None,
            })
    }

    fn iter_nodes_unordered(
        &self,
        pattern: Option<Pattern<K>>,
    ) -> impl Iterator<Item = &Node<K, V>> {
        TrieDfsTraversal::from_root(self, pattern)
    }

    fn path_to_root<'a>(
        &'a self,
        node: &'a Node<K, V>,
    ) -> impl Iterator<Item = &'a K> {
        let mut current = node;

        std::iter::from_fn(move || match current.parent(self) {
            Some(parent) => {
                current = parent;

                match current.key() {
                    Key::Internal(k) => Some(k),
                    _ => None,
                }
            }
            None => None, // at the root
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_no_pattern() {
        let mut trie = Trie::new();
        trie.push("car".chars(), ());
        trie.push("cat".chars(), ());
        trie.push("carp".chars(), ());

        assert_eq!(TrieDfsTraversal::from_root(&trie, None).count(), 9);
    }

    #[test]
    fn test_dfs_with_pattern() {
        let mut trie = Trie::new();

        // 'ca.'
        let pattern = vec![
            Some(Key::Start),
            Some(Key::Internal('c')),
            Some(Key::Internal('a')),
            None,
            Some(Key::End),
        ];

        trie.push("car".chars(), ());
        trie.push("cat".chars(), ());

        let n =
            TrieDfsTraversal::from_root(&trie, Some(pattern.clone())).count();

        // push a bunch of words that don't match the pattern,
        // so the number of nodes explored shouldn't change
        trie.push("carp".chars(), ());
        trie.push("carpet".chars(), ());
        trie.push("bazinga!".chars(), ());

        assert_eq!(
            TrieDfsTraversal::from_root(&trie, Some(pattern)).count(),
            n
        );
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
