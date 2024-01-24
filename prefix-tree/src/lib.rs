pub struct PrefixTree<K: Eq, V>(Node<K, V>);

/// The result of querying a prefix tree
#[derive(Debug, PartialEq, Eq)]
pub enum QueryResult<V> {
    /// The key is in the tree.
    Value(V),
    /// The key is not in the tree.
    NotFound,
    /// The key itself is not in the tree, but it is the prefix for some other stored key.
    Prefix,
}

impl<K: Eq, V> PrefixTree<K, V> {
    pub fn empty() -> Self {
        Self(Node::new(None, None))
    }

    pub fn get<T>(&self, key: T) -> QueryResult<&V>
    where
        T: IntoIterator<Item = K>,
    {
        self.0.get(key.into_iter())
    }

    pub fn set<T>(&mut self, key: T, value: V)
    where
        T: IntoIterator<Item = K>,
    {
        self.0.set(key.into_iter(), value)
    }
}

struct Node<K, V>
where
    K: Eq,
{
    /// None only at the root
    prefix_component: Option<K>,
    value: Option<V>,
    children: Vec<Node<K, V>>,
}

impl<K: Eq, V> Node<K, V> {
    fn new(prefix: Option<K>, value: Option<V>) -> Self {
        Self {
            prefix_component: prefix,
            value,
            children: Vec::new(),
        }
    }

    fn set<T>(&mut self, mut key: T, value: V)
    where
        T: Iterator<Item = K>,
    {
        match key.next() {
            None => {
                // store value at this node
                self.value = Some(value)
            }
            Some(next) => {
                // descend tree, if possible
                let possible_child = self
                    .children
                    .iter_mut()
                    .filter(|child| match &child.prefix_component {
                        Some(value) => value == &next,
                        _ => false,
                    })
                    .next();

                match possible_child {
                    None => {
                        let mut new_child = Self::new(Some(next), None);
                        new_child.set(key, value);
                        self.children.push(new_child);
                    }
                    Some(child) => child.set(key, value),
                }
            }
        }
    }

    fn get<T>(&self, mut key: T) -> QueryResult<&V>
    where
        T: Iterator<Item = K>,
    {
        return match key.next() {
            None => {
                // return value stored at this node, if it exists
                match &self.value {
                    Some(value) => QueryResult::Value(value),
                    None => QueryResult::Prefix,
                }
            }
            Some(next) => {
                // descend tree, if possible
                match self
                    .children
                    .iter()
                    .filter(|child| match &child.prefix_component {
                        Some(value) => value == &next,
                        _ => false,
                    })
                    .next()
                {
                    Some(child) => child.get(key),
                    None => QueryResult::NotFound,
                }
            }
        };
    }
}

pub struct StringPrefixTree(PrefixTree<char, ()>);

impl StringPrefixTree {
    pub fn empty() -> Self {
        Self(PrefixTree::empty())
    }

    pub fn contains(&self, key: &str) -> bool {
        match self.0.get(key.chars()) {
            QueryResult::Value(()) => true,
            _ => false,
        }
    }

    pub fn contains_prefix(&self, key: &str) -> bool {
        match self.0.get(key.chars()) {
            QueryResult::NotFound => false,
            _ => true,
        }
    }

    pub fn add(&mut self, key: &str) {
        self.0.set(key.chars(), ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_tree() {
        let mut trie: PrefixTree<char, String> = PrefixTree::empty();

        let value = "world".to_string();

        // hello -> world
        trie.set("hello".chars(), value.clone());

        assert_eq!(trie.get("hello".chars()), QueryResult::Value(&value));
        assert_eq!(trie.get("hel".chars()), QueryResult::Prefix);
        assert_eq!(trie.get("asdf".chars()), QueryResult::NotFound);
        assert_eq!(trie.get("helloworld".chars()), QueryResult::NotFound);
    }

    #[test]
    fn test_string_prefix_tree() {
        let mut trie = StringPrefixTree::empty();

        trie.add("hello");
        trie.add("world");

        assert!(trie.contains("hello"));
        assert!(trie.contains("world"));

        assert!(!trie.contains(""));
        assert!(!trie.contains("adf"));
        assert!(!trie.contains("worl"));
        assert!(!trie.contains("helloo"));

        assert!(trie.contains_prefix("hello"));
        assert!(trie.contains_prefix("world"));
        assert!(trie.contains_prefix("hel"));
        assert!(trie.contains_prefix("worl"));
        assert!(trie.contains_prefix("w"));
        assert!(trie.contains_prefix(""));

        assert!(!trie.contains_prefix("woooorld"));
        assert!(!trie.contains_prefix("asdlfa"));
    }
}
