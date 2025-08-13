use crate::node::{Key, Node};
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) enum Query<K> {
    Matches(Key<K>),
    Any,
}

pub(crate) struct DfsTraversal<'a, K, V> {
    // PERF: storing the whole prefix is not optimal.
    // is there a way to have nodes store pointers to their parents?
    stack: Vec<Vec<&'a Node<K, V>>>,
    pattern: Option<Vec<Query<K>>>,
}

impl<'a, K, V> DfsTraversal<'a, K, V> {
    pub(crate) fn new(
        root: &'a Node<K, V>,
        pattern: Option<Vec<Query<K>>>,
    ) -> Self {
        Self {
            pattern,
            stack: vec![vec![&root]],
        }
    }
}

impl<'a, K: Debug + Ord + Clone, V: Debug + Clone + Ord> Iterator
    for DfsTraversal<'a, K, V>
{
    type Item = (Vec<Option<&'a K>>, &'a Node<K, V>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prefix) = self.stack.pop() {
            let node = prefix.last().expect("guaranteed to be non-empty");

            let remainig_pattern =
                self.pattern.as_ref().map(|p| &p[prefix.len()..]);

            match remainig_pattern {
                // no pattern OR next pattern is Query::Any
                None | Some([Query::Any, ..]) => {
                    self.stack.extend(node.iter_children().map(|child| {
                        let mut v = prefix.clone();
                        v.push(child);
                        v
                    }))
                }
                // next pattern must match
                Some([Query::Matches(k), ..]) => {
                    if let Some(child) = node.children.get(k) {
                        let mut v = prefix.clone();
                        v.push(child);
                        self.stack.push(v);
                    }
                }
                // end of pattern. don't descend further
                Some([]) => {}
            }

            Some((prefix.iter().map(|n| n.key()).collect(), node))
        } else {
            // exhausted tree
            None
        }
    }
}
