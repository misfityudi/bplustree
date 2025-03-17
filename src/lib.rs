pub enum BplusTreeNode {
    Internal {
        keys: Vec<i32>,
        children: Vec<Box<BplusTreeNode>>
    },
    Leaf {
        keys: Vec<i32>,
        values: Vec<String>,
        next: Option<Box<BplusTreeNode>>
    }
}

pub struct BplusTree {
    pub order: usize,
    pub root: Option<Box<BplusTreeNode>>
}

impl BplusTree {
    pub fn new(order: Option<usize>) -> Self {
        Self {
            order: order.unwrap_or(4),
            root: None,
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bplustree() {
        let bplustree = BplusTree::new(None);
        assert_eq!(bplustree.order, 4);
    }
}
