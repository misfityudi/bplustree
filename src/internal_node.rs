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
}

