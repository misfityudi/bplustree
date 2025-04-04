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
        if enteries.len() < order{
            return Some(Self {
            enteries,
            next: None,
            prev: None,
        })
        } else {
            return None;
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

    pub fn merge() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_leafnode(){

    }

    #[test]
    fn test_split_leafnode(){}
}