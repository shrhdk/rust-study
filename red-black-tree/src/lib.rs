use std::cmp::Ordering::*;

use Color::*;

pub struct RBTreeSet<T> {
    root: Option<Box<RBTreeNode<T>>>,
    len: usize,
}

struct RBTreeNode<T> {
    color: Color,
    value: T,
    left: Option<Box<RBTreeNode<T>>>,
    right: Option<Box<RBTreeNode<T>>>,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Color {
    Red,
    Black,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

fn rotate_right<T>(opt_node: &mut Option<Box<RBTreeNode<T>>>) {
    let mut a = opt_node.take().unwrap();
    let mut b = a.left.take().unwrap();
    a.left = b.right.take();
    b.right = Some(a);
    std::mem::replace(opt_node, Some(b));
}

fn rotate_left<T>(opt_node: &mut Option<Box<RBTreeNode<T>>>) {
    let mut a = opt_node.take().unwrap();
    let mut b = a.right.take().unwrap();
    a.right = b.left.take();
    b.left = Some(a);
    std::mem::replace(opt_node, Some(b));
}

fn insert<T: Ord>(
    opt_node: &mut Option<Box<RBTreeNode<T>>>,
    dir: Option<Direction>,
    new_node: RBTreeNode<T>,
) -> (Option<Direction>, Option<Direction>) {
    use Direction::*;

    let pattern = match opt_node {
        Some(node) => match new_node.value.cmp(&node.value) {
            Less => insert(&mut node.left, Some(Left), new_node),
            Greater => insert(&mut node.right, Some(Right), new_node),
            Equal => {
                let dir = if node.color == Black { None } else { dir };
                return (dir, None);
            }
        },
        None => {
            std::mem::replace(opt_node, Some(Box::new(new_node)));
            return (dir, None);
        }
    };

    match pattern {
        (Some(Left), Some(Left)) => rotate_right(opt_node),
        (Some(Right), Some(Right)) => rotate_left(opt_node),
        (Some(Left), Some(Right)) => {
            rotate_left(&mut opt_node.as_mut().unwrap().left);
            rotate_right(opt_node);
        }
        (Some(Right), Some(Left)) => {
            rotate_right(&mut opt_node.as_mut().unwrap().right);
            rotate_left(opt_node);
        }
        _ => {
            let dir = if opt_node.as_ref().unwrap().color == Black { None } else { dir };
            return (dir, pattern.0);
        }
    }

    opt_node.as_mut().unwrap().color = Red;
    opt_node.as_mut().unwrap().left.as_mut().unwrap().color = Black;
    opt_node.as_mut().unwrap().right.as_mut().unwrap().color = Black;

    (dir, None)
}

impl<T: Ord> RBTreeSet<T> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
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
        if self.contains(&value) {
            return false;
        }

        let color = if self.len == 0 { Black } else { Red };
        let new_node = RBTreeNode {
            color,
            value,
            left: None,
            right: None,
        };

        insert(&mut self.root, None, new_node);
        self.root.as_mut().unwrap().color = Black;
        self.len += 1;

        true
    }

    pub fn clear(&mut self) {
        self.root.take();
        self.len = 0;
    }
}

impl<T: ToString> RBTreeSet<T> {
    pub fn pretty_print(&self) {
        fn print_node<T: ToString>(
            opt_node: &Option<Box<RBTreeNode<T>>>,
            prefix: String,
            last: bool,
        ) {
            let prefix_current = if last { "`- " } else { "|- " };
            let node_str = match opt_node {
                Some(node) => {
                    let color_str = match node.color {
                        Red => "R_".to_string(),
                        Black => "B_".to_string(),
                    };
                    color_str + &node.value.to_string()
                }
                None => "".to_string(),
            };
            println!("{}{}{}", prefix, prefix_current, node_str);
            let prefix = prefix + if last { "   " } else { "|  " };
            if let Some(node) = opt_node {
                if node.left.is_some() || node.right.is_some() {
                    print_node(&node.left, prefix.to_string(), false);
                    print_node(&node.right, prefix.to_string(), true);
                }
            }
        }
        print_node(&self.root, "".to_string(), true);
    }
}
