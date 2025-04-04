use crate::InternalNode;
use crate::LeafNode;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Internal(InternalNode),
    Leaf(LeafNode),
}
