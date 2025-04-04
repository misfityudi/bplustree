// Ensure the `node` module is declared or imported correctly
mod internal_node;
mod leaf_node;
mod node;

use internal_node::InternalNode;
use leaf_node::LeafNode;
use node::Node;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct BPlusTree {
    pub order: usize,
    pub root: Option<Rc<RefCell<Node>>>,
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
                let leaf = LeafNode::new(enteries, Some(self.order)).unwrap();

                self.root = Some(Rc::new(RefCell::new(Node::Leaf(leaf))));
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
                    let leaf = LeafNode::new(enteries, Some(self.order)).unwrap();

                    self.root = Some(Rc::new(RefCell::new(Node::Leaf(leaf))));
                } else {
                    println!("split and enter")
                }
            }
            Some(Node) => println!("bulk insert into either and internal node or a leaf node"),
        }
    }

    pub fn search(&self, key: i32) -> Option<String> {
        match &self.root {
            None => None,
            Some(node) => self.search_node(node, key),
        }
    }

    pub fn search_node(&self, node: &Rc<RefCell<Node>>, key: i32) -> Option<String> {
        match &*node.borrow() {
            Node::Leaf(leaf) => {
                return leaf.enteries.get(&key).cloned();
            }
            Node::Internal(internal) => {
                let mut child_iter = internal.enteries.range(..=key);

                // Try to find the largest key ≤ `key`
                if let Some((_child_key, child_node)) = child_iter.next_back() {
                    return self.search_node(child_node, key);
                }

                // If no such key exists, use the rightmost child
                if let Some((_child_key, last_child)) = internal.enteries.iter().next_back() {
                    return self.search_node(last_child, key);
                }

                return None;
            }
        }
    }

    pub fn update() {}

    pub fn range_search() {}

    pub fn min() {}

    pub fn max() {}

    pub fn delete() {}

    pub fn save_to_disk() {}

    pub fn load_from_disk() {}

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

        if let Some(node) = tree.root {
            match &*node.borrow() {
                Node::Leaf(leaf) => {
                    assert_eq!(leaf.enteries.get(&5), Some(&"Five".to_string()));
                    assert!(leaf.next.is_none());
                    assert!(leaf.prev.is_none());
                }
                Node::Internal(_) => panic!("Expected Leaf node, got Internal node"),
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
            match &*node.borrow() {
                Node::Leaf(ref leaf) => {
                    if keys.len() > tree.order {
                    } else {
                        for (k, v) in keys.iter().zip(values.iter()) {
                            assert_eq!(leaf.enteries.get(k), Some(v));
                        }

                        let prev_key: Option<i32> = None;

                        for k in leaf.enteries.keys() {
                            if let Some(prev) = prev_key {
                                assert!(prev < *k);
                            }
                        }

                        assert!(leaf.next.is_none());
                        assert!(leaf.prev.is_none());
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
    fn test_merge_nodes() {}

    #[test]
    fn test_borrow_from_sibling() {}

    #[test]
    fn test_traverse() {}
}
