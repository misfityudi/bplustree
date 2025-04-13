use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct LeafNode {
    pub enteries: BTreeMap<i32, String>,
    pub next: Option<Rc<RefCell<LeafNode>>>,
    pub prev: Option<Rc<RefCell<LeafNode>>>,
}

impl LeafNode {
    pub fn new(enteries: BTreeMap<i32, String>, order: Option<usize>) -> Option<Self> {
        let order = order.unwrap_or(32);
        match enteries.len() {
            len if len < order => Some(Self {
                enteries,
                next: None,
                prev: None,
            }),
            _ => None,
        }
    }

    pub fn split(&mut self, order: Option<usize>) -> Option<(i32, Rc<RefCell<LeafNode>>)> {
        let enteries_len = self.enteries.len();
        let order = order.unwrap_or(32);
        if enteries_len < order {
            return None;
        } else {
            let split_position = order / 2;
            let split_key = self.enteries.keys().nth(split_position).copied().unwrap();

            let right_enteries = self.enteries.split_off(&split_key);

            let mut right_leaf = LeafNode::new(right_enteries, Some(order)).unwrap();
            right_leaf.next = self.next.take();
            right_leaf.prev = Some(Rc::new(RefCell::new(self.clone())));

            return Some((split_key, Rc::new(RefCell::new(right_leaf))));
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_leaf_node() {
        let no_enteries = BTreeMap::new();
        let new_leaf = LeafNode::new(no_enteries, Some(3));

        assert_eq!(
            new_leaf.as_ref().unwrap().enteries.len() as i32,
            0,
            "New leaf node should not have any enteries"
        );
        assert!(
            new_leaf.as_ref().unwrap().next.is_none(),
            "New leaf node should not have next"
        );
        assert!(
            new_leaf.as_ref().unwrap().prev.is_none(),
            "New leaf node should not have prev"
        );

        let extra_keys = vec![5, 3, 7, 1, 9];
        let extra_values = vec![
            "Five".to_string(),
            "Three".to_string(),
            "Seven".to_string(),
            "One".to_string(),
            "Nine".to_string(),
        ];

        let extra_enteries: BTreeMap<i32, String> = extra_keys
            .into_iter()
            .zip(extra_values.into_iter())
            .collect();
        let leaf_with_extra_enteries = LeafNode::new(extra_enteries, Some(3));

        assert!(
            leaf_with_extra_enteries.is_none(),
            "Leaf node can not have more enteries than order - 1"
        );

        let keys = vec![5, 3];
        let values = vec!["Five".to_string(), "Three".to_string()];

        let enteries: BTreeMap<i32, String> = keys.into_iter().zip(values.into_iter()).collect();
        let leaf = LeafNode::new(enteries, Some(3));

        assert_eq!(
            leaf.as_ref().unwrap().enteries.len() as i32,
            2,
            "New leaf node should not all the enteries"
        );
        assert!(
            leaf.as_ref().unwrap().next.is_none(),
            "New leaf node should not have next"
        );
        assert!(
            leaf.as_ref().unwrap().prev.is_none(),
            "New leaf node should not have prev"
        );
    }

    #[test]
    fn test_split_leafnode() {}
}
