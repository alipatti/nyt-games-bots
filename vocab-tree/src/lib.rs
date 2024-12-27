use std::borrow::{Borrow, BorrowMut};

pub struct Vocabulary<'a, T>
where
    T: Clone + Eq,
{
    root: Node<'a, T>,
}

enum Node<'a, T> {
    Root {
        children: Vec<Box<Self>>,
    },
    Internal {
        letter: T,
        children: Vec<Box<Self>>,
        parent: &'a Self,
    },
    Leaf,
}

type Word<T> = [T];
type WordFragment<T> = [Option<T>];

pub struct SearchResult<'a, 'b, T: Clone + Eq> {
    word_fragment: &'b WordFragment<T>,
    remaining_nodes: std::slice::Iter<'a, Box<Node<T>>>,
    current_iterator: Option<Box<Self>>,
}

impl<T: Clone + Eq> Node<T> {
    pub fn insert(&mut self, word: &Word<T>) {
        let next_letter = &word[0];
        let rest_of_word = &word[1..];

        if let Node::Internal { children, .. } = self {
            // find matching child
            let child = children
                .iter_mut()
                .filter(|n| match &***n {
                    Node::Internal { letter, .. }
                        if { letter == next_letter } =>
                    {
                        true
                    }
                    _ => false,
                })
                .next();

            // add child if there is no matching child
            let child = match child {
                Some(node) => node,
                None => {
                    let new_child = Box::new(Node::Internal {
                        letter: next_letter.clone(),
                        children: Vec::new(),
                    });
                    children.push(new_child);

                    children.last_mut().unwrap()
                }
            };

            child.insert(rest_of_word);
        }
    }
}

impl<T: Clone + Eq> Vocabulary<T> {
    /// Inserts a word into the vocabulary.
    pub fn insert(&mut self, word: &Word<T>) {
        self.root.insert(word);
    }

    /// Checks whether or not a word is contained in the vocabulary.
    pub fn contains(&self, word: &Word<T>) -> bool {
        // convert word into word fragment, then use the `matching` function to check if it's in
        // the tree
        let word_as_fragment: Vec<_> =
            word.iter().map(|x| Some(x.to_owned())).collect();

        match self.matching(&word_as_fragment).into_iter().next() {
            Some(_) => true,
            None => false,
        }
    }

    /// Returns an iterator of words matching the word pattern
    pub fn matching<'a>(
        &'a self,
        word_fragment: &'a WordFragment<T>,
    ) -> SearchResult<T> {
        SearchResult::new(&self.root, word_fragment)
            .expect("Can't call matching on a leaf")
    }
}

impl<'a, T: Clone + Eq> SearchResult<'a, 'a, T> {
    fn new(
        root: &'a Node<T>,
        word_fragment: &'a WordFragment<T>,
    ) -> Option<Self> {
        let remaining_nodes = match root {
            Node::Internal { children, .. } => children.iter(),
            Node::Leaf => [].iter(),
        };

        Some(Self {
            word_fragment,
            remaining_nodes,
            current_iterator: todo!(),
        })
    }
}

impl<'a, T: Clone + Eq> Iterator for SearchResult<'a, 'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_iterator = self.current_iterator.as_deref_mut().unwrap();

        // if the pattern is empty and this node has a terminal child, we're done.

        // if the current iterator has items, return those
        if let Some(result) = current_iterator.next() {
            // TODO: return the maching word
            return Some(result);
        }

        // if the current iterator is empty, get a new one from the next child.
        // if there is no matching next child, return None (i.e., we've explored the whole tree)
        let next_node = self
            .remaining_nodes
            .borrow_mut()
            .filter(|n| match (&self.word_fragment[0], &***n) {
                (Some(letter_to_match), Node::Internal { letter, .. })
                    if { letter != letter_to_match } =>
                {
                    false
                }
                _ => true,
            })
            .next();

        // if the child is a leaf, return
        // if internal, update the current iterator
        match next_node {
            Some(node) => match &**node {
                Node::Internal { .. } => todo!(),
                Node::Leaf => {
                    let word = vec![];
                    return Some(word);
                }
            },
            None => return None,
        }

        // yield from child until it's empty

        // entire tree explored
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_word() {}
}
