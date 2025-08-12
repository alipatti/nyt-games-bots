use std::{collections::HashMap, fmt::Debug};

use crate::node::{Key, Node};

#[derive(Debug)]
pub(crate) enum Query<K> {
    Matches(K),
    Any,
}

pub(crate) struct DfsTraversal<'a, 'b, K> {
    stack: Vec<Vec<&'a Node<K>>>,
    _pattern: Option<&'b [Query<K>]>,
}

impl<'a, 'b, K> DfsTraversal<'a, 'b, K> {
    pub(crate) fn new(
        root: &'a Node<K>,
        pattern: Option<&'b [Query<K>]>,
    ) -> Self {
        Self {
            _pattern: pattern,
            stack: vec![vec![&root]],
        }
    }
}

impl<'a, 'b, K: Debug> Iterator for DfsTraversal<'a, 'b, K> {
    type Item = (Vec<Option<&'a K>>, &'a Node<K>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prefix) = self.stack.pop() {
            let node = prefix.last().expect("guaranteed to be non-empty");

            if true {
                // push children
                self.stack.extend(node.iter_children().map(|child| {
                    let mut v = prefix.clone();
                    v.push(child);
                    v
                }));
            } else {
                let child = todo!();
                self.stack.push(child);
            }

            Some((prefix.iter().map(|n| n.key()).collect(), node))
        } else {
            // exhausted tree
            None
        }
    }
}
