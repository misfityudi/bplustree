#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Internal {
        keys: Vec<i32>,
        children: Vec<Box<Node>>,
    },
    Leaf {
        keys: Vec<i32>,
        values: Vec<String>,
        next: Option<Box<Node>>,
    },
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
            Some(node) => Self::insert_into_node(node, key, value, self.order),
            None => {
                self.root = Some(Box::new(Node::Leaf {
                    keys: vec![key],
                    values: vec![value],
                    next: None,
                }))
            }
        }
    }

    fn insert_into_node(node: &mut Node, key: i32, value: String, order: usize) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bplustree() {
        let bplustree = BPlusTree::new(None);
        assert_eq!(bplustree.order, 32);
    }

    #[test]
    fn insert_into_root() {
        let mut bplustree = BPlusTree::new(None);
        assert_eq!(bplustree.order, 32);

        bplustree.insert(10, "Ten".to_string());
        assert_ne!(bplustree.root, None);
    }
}
