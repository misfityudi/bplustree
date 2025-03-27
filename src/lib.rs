use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Internal(InternalNode),
    Leaf(LeafNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InternalNode {
    enteries: BTreeMap<i32, Box<Node>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LeafNode {
    enteries: BTreeMap<i32, String>,
    next: Option<Box<LeafNode>>,
    prev: Option<Box<LeafNode>>,
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
            None => self.insert_into_root(key, value),
            Some(Node) => println!("insert into either an internal node or a leaf node"),
        }
    }

    pub fn bulk_insert(&mut self, keys: Vec<i32>, values: Vec<String>) {
        match &mut self.root {
            None => self.bulk_insert_into_root(keys, values),
            Some(Node) => println!("bulk insert into either and internal node or a leaf node"),
        }
    }

    pub fn search() {}

    pub fn update() {}

    pub fn range_search() {}

    pub fn min() {}

    pub fn max() {}

    pub fn delete() {}

    pub fn save_to_disk() {}

    pub fn load_from_disk() {}

    fn insert_into_root(&mut self, key: i32, value: String) {
        let mut enteries = BTreeMap::new();
        enteries.insert(key, value);
        let leaf = LeafNode {
            enteries,
            next: None,
            prev: None,
        };
        self.root = Some(Box::new(Node::Leaf(leaf)));
    }

    fn insert_into_internal_node(node: &mut Node, key: i32, value: String, order: usize) {}

    fn insert_into_leaf_node(node: &mut Node, key: i32, value: String, order: usize) {}

    fn bulk_insert_into_root(&mut self, keys: Vec<i32>, values: Vec<String>) {
        let mut enteries = BTreeMap::new();
        if keys.len() > self.order {
        } else {
            keys.into_iter().zip(values).for_each(|(k, v)| {
                enteries.insert(k, v);
            });

            let leaf = LeafNode {
                enteries,
                next: None,
                prev: None,
            };

            self.root = Some(Box::new(Node::Leaf(leaf)));
        }
    }

    fn bulk_insert_into_leaf_node() {}

    fn bulk_insert_into_internal_node() {}

    fn split_node() {}

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
    fn test_insert_into_root() {
        let mut tree = BPlusTree::new(None);
        tree.insert(5, "Five".to_string());

        if let Some(node) = &tree.root {
            match node.as_ref() {
                Node::Leaf(leaf) => {
                    assert_eq!(leaf.enteries.get(&5), Some(&"Five".to_string()));
                    assert!(leaf.next.is_none());
                    assert!(leaf.prev.is_none());
                }
                Node::Internal(_) => panic!("Expected Lead node, got Internal node"),
            }
        } else {
            panic!("Root node should not be None after insertion");
        }
    }

    #[test]
    fn test_insert_into_internal_node() {}

    #[test]
    fn test_insert_into_leaf_node() {}

    #[test]
    fn test_bulk_insert_into_root() {
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
    fn test_bulk_insert_into_internal_node() {}

    #[test]
    fn test_bulk_insert_into_leaf_node() {}

    #[test]
    fn test_bulk_insert() {}

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
    fn test_split_node() {}

    #[test]
    fn test_merge_nodes() {}

    #[test]
    fn test_borrow_from_sibling() {}

    #[test]
    fn test_traverse() {}
}
