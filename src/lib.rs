#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Internal(InternalNode),
    Leaf(LeafNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InternalNode {
    keys: Vec<i32>,
    children: Vec<Box<Node>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LeafNode {
    keys: Vec<i32>,
    values: Vec<String>,
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

    pub fn bulk_insert() {}

    pub fn search() {}

    pub fn update() {}

    pub fn range_search() {}

    pub fn min() {}

    pub fn max() {}

    pub fn delete() {}

    pub fn save_to_disk() {}

    pub fn load_from_disk() {}

    fn insert_into_root(&mut self, key: i32, value: String) {
        let leaf = LeafNode {
            keys: vec![key],
            values: vec![value],
            next: None,
            prev: None,
        };
        self.root = Some(Box::new(Node::Leaf(leaf)));
    }

    fn insert_into_internal_node(node: &mut Node, key: i32, value: String, order: usize) {}

    fn insert_into_leaf_node(node: &mut Node, key: i32, value: String, order: usize) {}

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
                    assert_eq!(leaf.keys, vec![5]);
                    assert_eq!(leaf.values, vec!["Five".to_string()]);
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
