#[derive(Debug)]
pub struct Node {
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

    fn add_left(&mut self, (value, key): (i32, i32)) -> Result<(), String> {
        let Node { ref mut left, .. } = *self;
        match *left {
            Some(ref mut child) => child.add((value, key)),
            None => {
                *left = Some(Node::new(value, key));
                Ok(())
            },
        }
    }

    fn add_right(&mut self, (value, key): (i32, i32)) -> Result<(), String> {
        let Node { ref mut right, .. } = *self;
        match *right {
            Some(ref mut child) => child.add((value, key)),
            None => {
                *right = Some(Node::new(value, key));
                Ok(())
            },
        }
    }

    pub fn add(&mut self, pair: (i32, i32)) -> Result<(), String>{
        match pair {
            (_, key) if key < self.key => {
                self.add_left(pair)
            },
            (_, key) if key > self.key => {
                self.add_right(pair)
            },
            _ => {
                Err(String::from("That key already exists"))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use node::Node;
    #[test]
    fn test_add_left() {
        let mut node = Node::new(1, 2);
        node.add((1, 1)).unwrap();
        assert!(node.left.unwrap().value == 1);
    }
    #[test]
    fn test_add_left_right() {
        let mut node = Node::new(1, 3);
        node.add((2, 1)).unwrap();
        node.add((3, 2)).unwrap();
        assert!(node.left.unwrap().right.unwrap().value == 3);
    }
    #[test]
    fn test_add_right() {
        let mut node = Node::new(1, 1);
        node.add((1, 2)).unwrap();
        assert!(node.right.unwrap().value == 1);
    }
    #[test]
    fn test_add_right_left() {
        let mut node = Node::new(1, 1);
        node.add((2, 3)).unwrap();
        node.add((3, 2)).unwrap();
        assert!(node.right.unwrap().left.unwrap().value == 3);
    }

    #[test]
    fn test_add_duplicate() {
        let mut node = Node::new(1, 1);
        assert!(node.add((12, 1)).is_err());
    }
}
