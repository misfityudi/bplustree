use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct InternalNode {
    pub enteries: BTreeMap<i32, Rc<Node>>,
}

impl InternalNode {
    pub fn new(enteries: BTreeMap<i32, Rc<Node>>, order: Option<usize>) -> Option<Self> {
        let order = order.unwrap_or(32);
        match enteries.len() {
            0 => None,
            len if len > 0 && len < order => Some(Self { enteries }),
            _ => None,
        }
    }

    pub fn split(&mut self, order: Option<usize>) -> Option<(i32, Rc<RefCell<InternalNode>>)> {
        let enteries_len = self.enteries.len();
        let order = order.unwrap_or(32);
        if enteries_len < order {
            return None;
        } else {
            let split_position = order / 2;
            let split_key = self.enteries.keys().nth(split_position).copied().unwrap();

            let right_enteries = self.enteries.split_off(&split_key);

            let right_internal = InternalNode::new(right_enteries, Some(order)).unwrap();

            return Some((split_key, Rc::new(RefCell::new(right_internal))));
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_new_internal_node() {
        let no_enteries = BTreeMap::new();
        let new_internal_node = InternalNode::new(no_enteries, Some(3));

        assert!(
            new_internal_node.as_ref().is_none(),
            "New internal node should not created_without any enteries"
        );
    }
}
