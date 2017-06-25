#![allow(dead_code)]

#[derive(Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    key: i32,
    value: i32,
}

impl Node {
    pub fn new(value: i32, key: i32) -> Box<Node> {
        Box::from(Node {
            left: None,
            right: None,
            key: key,
            value: value,
        })
    }

    fn add_left(&mut self, node: Box<Node>) {
        let Node { ref mut left, .. } = *self;
        match *left {
            Some(ref mut child) => child.add(node),
            None => *left = Some(node),
        }
    }

    fn add_right(&mut self, node: Box<Node>) {
        let Node { ref mut right, .. } = *self;
        match *right {
            Some(ref mut child) => child.add(node),
            None => *right = Some(node),
        }
    }

    pub fn add(&mut self, node: Box<Node>) {
        match *node {
            Node { key, .. } if key < self.key => {
                self.add_left(node);
            },
            Node { key, .. } if key > self.key => {
                self.add_right(node);
            },
            _ => {}
        }
    }
}

struct BST {
    root: Option<Box<Node>>,
}


impl BST {
    pub fn new() -> BST {
        BST {
            root: None
        }
    }

    fn is_empty(&self, ) -> bool {
        match self.root {
            Some(_) => false,
            None => true,
        }
    }


    pub fn add(&mut self, (value, key): (i32, i32)) {
        let BST { ref mut root, .. } = *self;
        match *root {
            Some(ref mut node) => node.add(Node::new(value, key)),
            None => *root = Some(Node::new(value, key)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
    #[test]
    fn test_add_on_empty() {
        let mut bst = ::BST::new();
        bst.add((1, 2));
        assert!(bst.root.unwrap().value == 1);
    }
    #[test]
    fn test_add_left() {
        let mut bst = ::BST::new();
        bst.add((1, 2));
        bst.add((1, 1));
        assert!(bst.root.unwrap().left.unwrap().value == 1);
    }
    #[test]
    fn test_add_left_right() {
        let mut bst = ::BST::new();
        bst.add((1, 3));
        bst.add((2, 1));
        bst.add((3, 2));
        assert!(bst.root.unwrap().left.unwrap().right.unwrap().value == 3);
    }
    #[test]
    fn test_add_right() {
        let mut bst = ::BST::new();
        bst.add((1, 1));
        bst.add((1, 2));
        assert!(bst.root.unwrap().right.unwrap().value == 1);
    }
    #[test]
    fn test_add_right_left() {
        let mut bst = ::BST::new();
        bst.add((1, 1));
        bst.add((2, 3));
        bst.add((3, 2));
        assert!(bst.root.unwrap().right.unwrap().left.unwrap().value == 3);
    }
}
