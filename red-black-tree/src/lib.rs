use std::cmp::Ordering::*;
use std::fmt::{Display, Error, Formatter};

use ansi_term::Colour;

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

    // insert value recursively and get inserted direction of RED child and RED grandchild.
    // inserted direction is None, if the child or the grand child is BLACK.
    let pattern = match opt_node {
        Some(node) => match new_node.value.cmp(&node.value) {
            Less => insert(&mut node.left, Some(Left), new_node),
            Greater => insert(&mut node.right, Some(Right), new_node),
            Equal => match node.color {
                Red => return (dir, None),
                Black => return (None, None),
            },
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
        _ => match opt_node.as_ref().unwrap().color {
            Red => return (dir, pattern.0),
            Black => return (None, pattern.0),
        },
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

impl<T: Display> Display for RBTreeSet<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        fn fmt_node<T: Display>(
            f: &mut Formatter,
            opt_node: &Option<Box<RBTreeNode<T>>>,
            prefix: String,
            last: bool,
        ) -> Result<(), Error> {
            let prefix_current = if last { "`- " } else { "|- " };
            write!(f, "{}{}", prefix, prefix_current)?;
            if let Some(node) = opt_node {
                let node_str = match node.color {
                    Red => Colour::Red.paint(format!("{}", node.value)),
                    Black => Colour::Black.paint(format!("{}", node.value)),
                };
                write!(f, "{}", node_str)?;
            }
            writeln!(f)?;
            let prefix = prefix + if last { "   " } else { "|  " };
            if let Some(node) = opt_node {
                if node.left.is_some() || node.right.is_some() {
                    fmt_node(f, &node.left, prefix.to_string(), false)?;
                    fmt_node(f, &node.right, prefix.to_string(), true)?;
                }
            }
            Ok(())
        }
        fmt_node(f, &self.root, "".to_string(), true)
    }
}
