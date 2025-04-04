use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct InternalNode {
    pub enteries: BTreeMap<i32, Rc<Node>>,
}

impl InternalNode {
    pub fn new(enteries: BTreeMap<i32, Rc<Node>>) -> Self {
        Self { enteries }
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

            let right_internal = InternalNode::new(right_enteries);

            return Some((split_key, Rc::new(RefCell::new(right_internal))));
        }
    }
}
