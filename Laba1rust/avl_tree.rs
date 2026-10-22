use std::cmp::max;

pub struct Node {
    pub data: i32,
    pub height: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct AVLTree {
    pub root: Option<Box<Node>>,
}

impl AVLTree {
    pub fn new() -> Self { AVLTree { root: None } }

    fn height(n: &Option<Box<Node>>) -> i32 {
        n.as_ref().map_or(0, |node| node.height)
    }

    fn update_height(n: &mut Node) {
        n.height = 1 + max(Self::height(&n.left), Self::height(&n.right));
    }

    fn rotate_right(mut y: Box<Node>) -> Box<Node> {
        let mut x = y.left.take().unwrap();
        y.left = x.right.take();
        Self::update_height(&mut y);
        x.right = Some(y);
        Self::update_height(&mut x);
        x
    }

    fn rotate_left(mut x: Box<Node>) -> Box<Node> {
        let mut y = x.right.take().unwrap();
        x.right = y.left.take();
        Self::update_height(&mut x);
        y.left = Some(x);
        Self::update_height(&mut y);
        y
    }

    fn balance(mut n: Box<Node>) -> Box<Node> {
        Self::update_height(&mut n);
        let bf = Self::height(&n.left) - Self::height(&n.right);

        if bf > 1 {
            if Self::height(&n.left.as_ref().unwrap().right) > Self::height(&n.left.as_ref().unwrap().left) {
                let left = n.left.take().unwrap();
                n.left = Some(Self::rotate_left(left));
            }
            return Self::rotate_right(n);
        } else if bf < -1 {
            if Self::height(&n.right.as_ref().unwrap().left) > Self::height(&n.right.as_ref().unwrap().right) {
                let right = n.right.take().unwrap();
                n.right = Some(Self::rotate_right(right));
            }
            return Self::rotate_left(n);
        }
        n
    }

    fn insert_rec(node: Option<Box<Node>>, val: i32) -> Option<Box<Node>> {
        match node {
            None => Some(Box::new(Node { data: val, height: 1, left: None, right: None })),
            Some(mut n) => {
                if val < n.data {
                    n.left = Self::insert_rec(n.left.take(), val);
                } else if val > n.data {
                    n.right = Self::insert_rec(n.right.take(), val);
                } else {
                    return Some(n);
                }
                Some(Self::balance(n))
            }
        }
    }

    pub fn insert(&mut self, val: i32) {
        self.root = Self::insert_rec(self.root.take(), val);
    }

    fn find_min(mut n: Box<Node>) -> Box<Node> {
        while n.left.is_some() { n = n.left.take().unwrap(); }
        n
    }

    fn remove_rec(node: Option<Box<Node>>, val: i32) -> Option<Box<Node>> {
        match node {
            None => None,
            Some(mut n) => {
                if val < n.data {
                    n.left = Self::remove_rec(n.left.take(), val);
                } else if val > n.data {
                    n.right = Self::remove_rec(n.right.take(), val);
                } else {
                    if n.left.is_none() { return n.right; }
                    if n.right.is_none() { return n.left; }
                    let min_node = Self::find_min(n.right.take().unwrap());
                    n.data = min_node.data;
                    n.right = Self::remove_rec(n.right.take(), n.data);
                }
                Some(Self::balance(n))
            }
        }
    }

    pub fn remove(&mut self, val: i32) {
        self.root = Self::remove_rec(self.root.take(), val);
    }

    fn contains_rec(node: &Option<Box<Node>>, val: i32) -> bool {
        match node {
            None => false,
            Some(n) => {
                if val == n.data { true }
                else if val < n.data { Self::contains_rec(&n.left, val) }
                else { Self::contains_rec(&n.right, val) }
            }
        }
    }

    pub fn contains(&self, val: i32) -> bool {
        Self::contains_rec(&self.root, val)
    }

    pub fn print_in_order(node: &Option<Box<Node>>, out: &mut Vec<i32>) {
        if let Some(n) = node {
            Self::print_in_order(&n.left, out);
            out.push(n.data);
            Self::print_in_order(&n.right, out);
        }
    }

    pub fn print(&self) {
        let mut out = Vec::new();
        Self::print_in_order(&self.root, &mut out);
        println!("{:?}", out);
    }
}
