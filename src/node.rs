use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::LeafNode;
use crate::InternalNode;




#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Internal(Rc<RefCell<InternalNode>>),
    Leaf(Rc<RefCell<LeafNode>>),
}