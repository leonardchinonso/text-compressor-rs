use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

/// Node is the trait for node behaviours in the huffman tree
pub trait Node {
    fn get_frequency(&self) -> i32;
    fn get_character(&self) -> Option<char>;
    fn get_left(&self) -> Link;
    fn get_right(&self) -> Link;
    fn is_leaf(&self) -> bool;
}

/// BoxNode contains and sizes the Node trait in compile time
pub type BoxNode = Box<dyn Node>;

impl Eq for BoxNode {}

impl PartialEq<Self> for Box<dyn Node> {
    fn eq(&self, other: &Self) -> bool {
        self.get_frequency() == other.get_frequency()
    }
}

impl PartialOrd<Self> for BoxNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BoxNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_frequency().cmp(&other.get_frequency())
    }
}

/// Link represents a connection between nodes
pub type Link = Option<Rc<RefCell<BoxNode>>>;

/// HuffmanLeafNode represents a LEAF node in the huffman tree
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct HuffmanLeaf {
    character: char,
    node: HuffmanNode,
}

impl HuffmanLeaf {
    pub fn new(character: char, frequency: i32) -> Self {
        let mut node = HuffmanNode::new(None, None);
        node.frequency = frequency;
        Self { character, node }
    }

    /// get_character returns the character of the leaf node
    pub fn get_character(&self) -> char {
        self.character
    }
}

impl Node for HuffmanLeaf {
    fn get_frequency(&self) -> i32 {
        self.node.frequency
    }

    fn get_character(&self) -> Option<char> {
        Some(self.character)
    }

    fn get_left(&self) -> Link {
        self.node.left.clone()
    }

    fn get_right(&self) -> Link {
        self.node.right.clone()
    }

    fn is_leaf(&self) -> bool {
        true
    }
}

/// HuffmanNode represents a NON-LEAF node in the huffman tree
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct HuffmanNode {
    frequency: i32,
    left: Link,
    right: Link,
}

impl HuffmanNode {
    pub fn new(left_node: Link, right_node: Link) -> Self {
        match left_node.is_none() && right_node.is_none() {
            true => Self {
                frequency: 0,
                left: None,
                right: None,
            },
            false => {
                let left_node = left_node.unwrap();
                let right_node = right_node.unwrap();
                let node = Self {
                    frequency: left_node.borrow().get_frequency()
                        + right_node.borrow().get_frequency(),
                    left: Some(Rc::clone(&left_node)),
                    right: Some(Rc::clone(&right_node)),
                };
                node
            }
        }
    }
}

impl Node for HuffmanNode {
    fn get_frequency(&self) -> i32 {
        self.frequency
    }

    fn get_character(&self) -> Option<char> {
        None
    }

    fn get_left(&self) -> Link {
        self.left.clone()
    }

    fn get_right(&self) -> Link {
        self.right.clone()
    }

    fn is_leaf(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn huffman_leaf_works() {
        let leaf = HuffmanLeaf::new('a', 2);
        assert_eq!(leaf.get_character(), 'a');
        assert_eq!(leaf.get_frequency(), 2);
    }

    #[test]
    fn huffman_node_works() {
        let mut node = HuffmanNode::new(None, None);
        node.frequency = 1;
        assert_eq!(node.get_frequency(), 1);
        assert!(node.left.is_none());
        assert!(node.right.is_none());

        let node1 = node;
        let mut node2 = HuffmanNode::new(None, None);
        node2.frequency = 2;
        let node3 = HuffmanNode::new(
            Some(Rc::new(RefCell::new(Box::new(node1)))),
            Some(Rc::new(RefCell::new(Box::new(node2)))),
        );
        assert_eq!(node3.frequency, 3);
    }

    #[test]
    fn huffman_leaf_huffman_node_works() {
        let leaf_a = HuffmanLeaf::new('a', 2);
        let leaf_b = HuffmanLeaf::new('b', 3);
        let leaf_c = HuffmanLeaf::new('c', 1);
        let leaf_d = HuffmanLeaf::new('d', 5);

        let node_ab = HuffmanNode::new(
            Some(Rc::new(RefCell::new(Box::new(leaf_a)))),
            Some(Rc::new(RefCell::new(Box::new(leaf_b)))),
        );
        assert_eq!(node_ab.get_frequency(), 5);

        let node_cd = HuffmanNode::new(
            Some(Rc::new(RefCell::new(Box::new(leaf_c)))),
            Some(Rc::new(RefCell::new(Box::new(leaf_d)))),
        );
        assert_eq!(node_cd.get_frequency(), 6);

        let node_abcd = HuffmanNode::new(
            Some(Rc::new(RefCell::new(Box::new(node_ab)))),
            Some(Rc::new(RefCell::new(Box::new(node_cd)))),
        );
        assert_eq!(node_abcd.get_frequency(), 11);
    }
}
