use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Clone)]
pub struct LeafNode {
    pub enteries: BTreeMap<i32, String>,
    pub next: Option<Rc<RefCell<LeafNode>>>,
    pub prev: Option<Rc<RefCell<LeafNode>>>,
}

impl LeafNode {
    pub fn new(enteries: BTreeMap<i32, String>, next: Option<Rc<RefCell<LeafNode>>>, prev: Option<Rc<RefCell<LeafNode>>>) -> Self {
        Self { enteries, next, prev }
    }
}