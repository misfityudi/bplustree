use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::InternalNode;
use crate::LeafNode;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Internal(Rc<RefCell<InternalNode>>),
    Leaf(Rc<RefCell<LeafNode>>),
}
