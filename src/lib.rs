pub struct BplusTree {
    pub items: u8,
    pub children: Vec<BplusTree>,
    pub parent: Vec<BplusTree>,
}

impl BplusTree {
    pub fn new(items: Option<u8>) -> Self {
        Self {
            items: items.unwrap_or(4),
            children: Vec::new(),
            parent: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bplustree() {
        let bplustree = BplusTree::new(None);
        assert_eq!(bplustree.items, 4);
        assert_eq!(bplustree.children.len(), 0);
        assert_eq!(bplustree.parent.len(), 0);
    }
}
