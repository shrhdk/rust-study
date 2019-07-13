use std::cmp::Ordering::{Equal, Greater, Less};

pub struct BTreeSet<T> {
    root: Option<Box<BTreeNode<T>>>,
    len: usize,
}

struct BTreeNode<T> {
    value: T,
    left: Option<Box<BTreeNode<T>>>,
    right: Option<Box<BTreeNode<T>>>,
}

impl<T: Ord> BTreeSet<T> {
    pub fn new() -> BTreeSet<T> {
        BTreeSet { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn contains(&self, value: &T) -> bool {
        self.get(value).is_some()
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        let mut opt_node = &self.root;
        while let Some(node) = opt_node {
            match value.cmp(&node.value) {
                Less => opt_node = &node.left,
                Greater => opt_node = &node.right,
                Equal => return Some(&node.value),
            }
        }
        None
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut opt_node = &mut self.root;
        while let Some(node) = opt_node {
            match value.cmp(&node.value) {
                Less => opt_node = &mut node.left,
                Greater => opt_node = &mut node.right,
                Equal => return false,
            }
        }
        let new_node = BTreeNode {
            value,
            left: None,
            right: None,
        };
        std::mem::replace(opt_node, Some(Box::new(new_node)));
        self.len += 1;
        true
    }

    pub fn clear(&mut self) {
        self.root.take();
        self.len = 0;
    }
}
