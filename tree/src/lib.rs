use std::cmp::Ordering::*;

// BTreeSet

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

impl<T: ToString> BTreeSet<T> {
    pub fn pretty_print(&self) {
        fn print_node<T: ToString>(opt_node: &Option<Box<BTreeNode<T>>>, prefix: String, last: bool) {
            let prefix_current = if last { "`- " } else { "|- " };
            let node_str = match opt_node {
                Some(node) => node.value.to_string(),
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

// RBTreeSet

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

#[derive(Eq, PartialEq, Copy, Clone)]
enum Pattern {
    LL,
    RR,
    LR,
    RL,
    Ok,
}

fn insert<T: Ord>(opt_node: &mut Option<Box<RBTreeNode<T>>>, new_node: RBTreeNode<T>) -> bool {
    match opt_node {
        Some(node) => {
            match new_node.value.cmp(&node.value) {
                Less => insert(&mut node.left, new_node),
                Greater => insert(&mut node.right, new_node),
                Equal => false,
            }
        }
        None => {
            std::mem::replace(opt_node, Some(Box::new(new_node)));
            true
        }
    }
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

fn rotate_left_right<T>(opt_node: &mut Option<Box<RBTreeNode<T>>>) {
    rotate_left(&mut opt_node.as_mut().unwrap().left);
    rotate_right(opt_node);
}

fn rotate_right_left<T>(opt_node: &mut Option<Box<RBTreeNode<T>>>) {
    rotate_right(&mut opt_node.as_mut().unwrap().right);
    rotate_left(opt_node);
}

fn pattern<T: Ord>(opt_node: &Option<Box<RBTreeNode<T>>>, value: &T) -> Option<Pattern> {
    let a = opt_node.as_ref()?;
    let (b, dir1) = match value.cmp(&a.value) {
        Less => (a.left.as_ref()?, Direction::Left),
        Greater => (a.right.as_ref()?, Direction::Right),
        Equal => return None,
    };
    let (c, dir2) = match value.cmp(&b.value) {
        Less => (b.left.as_ref()?, Direction::Left),
        Greater => (b.right.as_ref()?, Direction::Right),
        Equal => return None,
    };

    if (a.color, b.color, c.color) == (Color::Black, Color::Red, Color::Red) {
        match (dir1, dir2) {
            (Direction::Left, Direction::Left) => Some(Pattern::LL),
            (Direction::Right, Direction::Right) => Some(Pattern::RR),
            (Direction::Left, Direction::Right) => Some(Pattern::LR),
            (Direction::Right, Direction::Left) => Some(Pattern::RL),
        }
    } else {
        Some(Pattern::Ok)
    }
}

fn balance<T: Ord>(opt_node: &mut Option<Box<RBTreeNode<T>>>, value: &T) -> bool {
    match pattern(opt_node, value) {
        Some(Pattern::LL) => {
            rotate_right(opt_node);
            opt_node.as_mut().unwrap().color = Color::Red;
            opt_node.as_mut().unwrap().left.as_mut().unwrap().color = Color::Black;
            opt_node.as_mut().unwrap().right.as_mut().unwrap().color = Color::Black;
            true
        }
        Some(Pattern::RR) => {
            rotate_left(opt_node);
            opt_node.as_mut().unwrap().color = Color::Red;
            opt_node.as_mut().unwrap().left.as_mut().unwrap().color = Color::Black;
            opt_node.as_mut().unwrap().right.as_mut().unwrap().color = Color::Black;
            true
        }
        Some(Pattern::LR) => {
            rotate_left(&mut opt_node.as_mut().unwrap().left);
            rotate_right(opt_node);
            opt_node.as_mut().unwrap().color = Color::Red;
            opt_node.as_mut().unwrap().left.as_mut().unwrap().color = Color::Black;
            opt_node.as_mut().unwrap().right.as_mut().unwrap().color = Color::Black;
            true
        }
        Some(Pattern::RL) => {
            rotate_right(&mut opt_node.as_mut().unwrap().right);
            rotate_left(opt_node);
            opt_node.as_mut().unwrap().color = Color::Red;
            opt_node.as_mut().unwrap().left.as_mut().unwrap().color = Color::Black;
            opt_node.as_mut().unwrap().right.as_mut().unwrap().color = Color::Black;
            true
        }
        Some(Pattern::Ok) => {
            let next_opt_node = match value.cmp(&opt_node.as_ref().unwrap().value) {
                Less => &mut opt_node.as_mut().unwrap().left,
                Greater => &mut opt_node.as_mut().unwrap().right,
                Equal => panic!(""),
            };
            balance(next_opt_node, value)
        }
        None => false,
    }
}

fn check_for_debug<T>(set: &RBTreeSet<T>) {
    fn check<T>(opt_node: &Option<Box<RBTreeNode<T>>>, parent_color: Option<Color>) {
        if let Some(node) = opt_node {
            if node.color == Color::Red {
                if parent_color.is_none() {
                    panic!("root is red.");
                } else if parent_color == Some(Color::Red) {
                    panic!("parent and child are red.");
                }
                check(&node.left, Some(node.color));
                check(&node.right, Some(node.color));
            }
        }
    }
    check(&set.root, None);
}

impl<T: Ord + Copy + Clone> RBTreeSet<T> {
    pub fn new() -> RBTreeSet<T> {
        RBTreeSet { root: None, len: 0 }
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
        let color = if self.len == 0 { Color::Black } else { Color::Red };
        let new_node = RBTreeNode {
            color,
            value,
            left: None,
            right: None,
        };

        let inserted = insert(&mut self.root, new_node);

        if inserted {
            self.len += 1;
            while balance(&mut self.root, &value) {};
            self.root.as_mut().unwrap().color = Color::Black;
        }

        inserted
    }

    pub fn clear(&mut self) {
        self.root.take();
        self.len = 0;
    }
}

impl<T: ToString> RBTreeSet<T> {
    pub fn pretty_print(&self) {
        fn print_node<T: ToString>(opt_node: &Option<Box<RBTreeNode<T>>>, prefix: String, last: bool) {
            let prefix_current = if last { "`- " } else { "|- " };
            let node_str = match opt_node {
                Some(node) => {
                    let color_str = match node.color {
                        Color::Red => "R_".to_string(),
                        Color::Black => "B_".to_string(),
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
