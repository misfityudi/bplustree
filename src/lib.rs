use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Internal(Rc<RefCell<InternalNode>>),
    Leaf(Rc<RefCell<LeafNode>>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InternalNode {
    enteries: BTreeMap<i32, Rc<Node>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LeafNode {
    enteries: BTreeMap<i32, String>,
    next: Option<Rc<RefCell<LeafNode>>>,
    prev: Option<Rc<RefCell<LeafNode>>>,
}

#[derive(Debug)]
pub struct BPlusTree {
    pub order: usize,
    pub root: Option<Box<Node>>,
}

impl BPlusTree {
    pub fn new(order: Option<usize>) -> Self {
        Self {
            order: order.unwrap_or(32),
            root: None,
        }
    }

    pub fn insert(&mut self, key: i32, value: String) {
        match &mut self.root {
            None => {
                let mut enteries = BTreeMap::new();
                enteries.insert(key, value);
                let leaf = LeafNode {
                    enteries,
                    next: None,
                    prev: None,
                };

                self.root = Some(Box::new(Node::Leaf(Rc::new(RefCell::new(leaf)))));
            }
            Some(Node) => println!("insert into either an internal node or a leaf node"),
        }
    }

    pub fn bulk_insert(&mut self, keys: Vec<i32>, values: Vec<String>) {
        match &mut self.root {
            None => {
                if keys.len() <= self.order {
                    let enteries: BTreeMap<i32, String> =
                        keys.into_iter().zip(values.into_iter()).collect();
                    let leaf = LeafNode {
                        enteries,
                        next: None,
                        prev: None,
                    };

                    self.root = Some(Box::new(Node::Leaf(Rc::new(RefCell::new(leaf)))));
                } else {
                    println!("split and enter")
                }
            }
            Some(Node) => println!("bulk insert into either and internal node or a leaf node"),
        }
    }

    pub fn search(self, key: i32) -> Option<String> {
        match self.root {
            None => None,
            Some(node) => match *node {
                Node::Leaf(leaf) => {
                    let value = leaf.borrow().enteries.get(&key).unwrap().clone();
                    return Some(value);
                }
                Node::Internal(internal) => {
                    println!("search in internal node");
                    return None;
                }
            },
        }
    }

    pub fn update() {}

    pub fn range_search() {}

    pub fn min() {}

    pub fn max() {}

    pub fn delete() {}

    pub fn save_to_disk() {}

    pub fn load_from_disk() {}

    fn split_node(&mut self, node: &mut Node) -> Option<(i32, Box<Node>)> {
        let enteries_len = match node {
            Node::Leaf(leaf) => leaf.borrow().enteries.len(),
            Node::Internal(internal) => internal.borrow().enteries.len(),
        };

        if enteries_len <= self.order {
            return None;
        }

        let split_point = self.order / 2;
        match node {
            Node::Leaf(leaf) => {
                let split_key = *leaf.borrow().enteries.keys().nth(split_point).unwrap();

                let right_enteries = leaf.borrow_mut().enteries.split_off(&split_key);

                let right_leaf = LeafNode {
                    enteries: right_enteries,
                    next: leaf.borrow_mut().next.take(),
                    prev: Some(Rc::clone(leaf)),
                };

                let right_leaf_rc = Rc::new(RefCell::new(right_leaf));
                leaf.borrow_mut().next = Some(Rc::clone(&right_leaf_rc));

                return Some((split_key, Box::new(Node::Leaf(Rc::clone(&right_leaf_rc)))));
            }
            Node::Internal(internal) => {
                let split_key = *internal.borrow().enteries.keys().nth(split_point).unwrap();

                let mut internal_borrow = internal.borrow_mut();
                let right_enteries = internal_borrow.enteries.split_off(&split_key);
                let right_internal = InternalNode {
                    enteries: right_enteries,
                };

                return Some((
                    split_key,
                    Box::new(Node::Internal(Rc::new(RefCell::new(right_internal)))),
                ));
            }
        }
    }

    fn merge_nodes() {}

    fn borrow_from_sibiling() {}

    fn traverse() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bplustree() {
        let tree = BPlusTree::new(None);
        assert_eq!(tree.order, 32);
    }

    #[test]
    fn test_insert() {
        let mut tree = BPlusTree::new(None);
        tree.insert(5, "Five".to_string());

        if let Some(node) = &tree.root {
            match node.as_ref() {
                Node::Leaf(leaf) => {
                    assert_eq!(leaf.borrow().enteries.get(&5), Some(&"Five".to_string()));
                    assert!(leaf.borrow().next.is_none());
                    assert!(leaf.borrow().prev.is_none());
                }
                Node::Internal(_) => panic!("Expected Lead node, got Internal node"),
            }
        } else {
            panic!("Root node should not be None after insertion");
        }
    }

    #[test]
    fn test_bulk_insert() {
        let mut tree = BPlusTree::new(None);
        let keys = vec![5, 3, 7, 1, 9];
        let values = vec![
            "Five".to_string(),
            "Three".to_string(),
            "Seven".to_string(),
            "One".to_string(),
            "Nine".to_string(),
        ];

        tree.bulk_insert(keys.clone(), values.clone());

        if let Some(node) = &tree.root {
            match node.as_ref() {
                Node::Leaf(leaf) => {
                    if keys.len() > tree.order {
                    } else {
                        for (k, v) in keys.iter().zip(values.iter()) {
                            assert_eq!(leaf.borrow().enteries.get(k), Some(v));
                        }

                        let prev_key: Option<i32> = None;

                        for k in leaf.borrow().enteries.keys() {
                            if let Some(prev) = prev_key {
                                assert!(prev < *k);
                            }
                        }

                        assert!(leaf.borrow().next.is_none());
                        assert!(leaf.borrow().prev.is_none());
                    }
                }
                Node::Internal(_) => panic!("Expected Leaf node, got internal node"),
            }
        } else {
            panic!("Root node should not be None after bulk insertion");
        }
    }

    #[test]
    fn test_search() {}

    #[test]
    fn test_update() {}

    #[test]
    fn test_range_search() {}

    #[test]
    fn test_min() {}

    #[test]
    fn test_max() {}

    #[test]
    fn test_delete() {}

    #[test]
    fn test_save_to_disk() {}

    #[test]
    fn test_load_from_disk() {}

    #[test]
    fn test_split_node() {
        let mut tree = BPlusTree::new(Some(3));

        let mut small_enteries = BTreeMap::new();
        for i in 1..=2 {
            small_enteries.insert(i, i.to_string());
        }

        let small_leaf = LeafNode {
            enteries: small_enteries,
            next: None,
            prev: None,
        };

        let mut small_node = Node::Leaf(Rc::new(RefCell::new(small_leaf)));
        assert!(tree.split_node(&mut small_node).is_none());

        let mut large_entries = BTreeMap::new();
        for i in 1..=10 {
            // 4 entries > order of 3
            large_entries.insert(i, i.to_string());
        }

        let large_leaf = LeafNode {
            enteries: large_entries.clone(),
            next: None,
            prev: None,
        };

        let mut node = Node::Leaf(Rc::new(RefCell::new(large_leaf)));

        let split_result = tree.split_node(&mut node);
        assert!(
            split_result.is_some(),
            "Node should split when enteries > order"
        );

        if let Some((split_key, right_node)) = split_result {
            match node {
                Node::Leaf(left_leaf) => {
                    // Verify left node has at most order/2 entries
                    assert!(
                        left_leaf.borrow().enteries.len() <= tree.order / 2 + 1,
                        "Left node size should be <= order/2 +1"
                    );

                    assert!(
                        left_leaf.borrow().enteries.keys().all(|k| *k < split_key),
                        "All keys in left node should be less than split key"
                    );

                    match *right_node {
                        Node::Leaf(right_leaf) => {
                            // Verify total entries after split still fit in tree
                            let total_entries = left_leaf.borrow().enteries.len()
                                + right_leaf.borrow().enteries.len();
                            assert!(
                                total_entries == large_entries.len(),
                                "No entries should be lost during split"
                            );
                            assert!(
                                right_leaf.borrow().enteries.keys().all(|k| *k >= split_key),
                                "All keys in right node should be >= split key"
                            );

                            assert!(
                                right_leaf.borrow().next.is_none(),
                                "Right node's next should be None"
                            );
                            assert!(
                                right_leaf.borrow().prev.is_some(),
                                "Right node's prev should be Some"
                            );
                            assert!(
                                left_leaf.borrow().next.is_some(),
                                "Left node's next should be Some"
                            );
                            assert!(
                                left_leaf.borrow().prev.is_none(),
                                "Left node's prev should be None"
                            );

                            // Verify split key is the smallest key in right node
                            assert_eq!(
                                split_key,
                                *right_leaf.borrow().enteries.keys().next().unwrap(),
                                "Split key should be smallest key in right node"
                            );
                        }
                        _ => panic!("Expected right node to be a leaf"),
                    }
                }
                _ => panic!("Expected left node to be a leaf"),
            }
        } else {
            panic!("Expected left node to be a leaf");
        }
    }

    #[test]
    fn test_merge_nodes() {}

    #[test]
    fn test_borrow_from_sibling() {}

    #[test]
    fn test_traverse() {}
}
