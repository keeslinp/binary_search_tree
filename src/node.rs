#[derive(Debug)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    pub key: i32,
    pub value: i32,
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

    fn get_left(&self, key: i32) -> Result<i32, String> {
        let Node { ref left, .. } = *self;
        match *left {
            Some(ref child) => child.get(key),
            None => {
                Err(String::from("That key doesn't exist"))
            },
        }
    }
    
    fn get_right(&self, key: i32) -> Result<i32, String> {
        let Node { ref right, .. } = *self;
        match *right {
            Some(ref child) => child.get(key),
            None => {
                Err(String::from("That key doesn't exist"))
            },
        }
    }

    pub fn get(&self, key: i32) -> Result<i32, String> {
        match key {
            key if key < self.key => {
                self.get_left(key)
            },
            key if key > self.key => {
                self.get_right(key)
            },
            key if key == self.key => {
                Ok(self.value)
            },
            _ => Err(String::from("Error"))
        }
    }

    fn get_last_right(&self) -> &Node {
        self.right.as_ref().map(|left| left.get_last_right()).unwrap_or(&self)
    }

    fn get_last_left(&self) -> &Node {
        self.left.as_ref().map(|left| left.get_last_left()).unwrap_or(&self)
    }

    fn remove_node(&self, child: Option<Box<Node>>, target_key: i32) -> (Option<Box<Node>>, Result<i32, String>) {
        match child {
            Some(mut node) => {
                if node.key== target_key {
                    let val = node.value;
                    // Anyone wondering about the funky tuple magic here should check
                    // https://github.com/rust-lang/rust/issues/16223#issuecomment-307237373
                    ( match (node,) {
                        (box Node { left: None, right: None, .. },) => {
                            None
                        },
                        (box Node { left: Some(child), right: None, .. },) => {
                            Some(child)
                        },
                        (box Node { left: None, right: Some(child), .. },) => {
                            Some(child)
                        },
                        (box Node { left: Some(child_left), right: Some(child_right), .. },) => {
                            let &Node { key, value, .. } = child_right.get_last_left();
                            let mut new_node = Box::from(Node { 
                                value,
                                key,
                                left: Some(child_left),
                                right: Some(child_right),
                            });
                            new_node.remove_right(key).unwrap();
                            Some(new_node)
                        },
                    }, Ok(val))
                } else {
                    let resp = node.as_mut().remove(target_key);
                    (Some(node), resp)
                }
            }
            None => {
                (None, Err(String::from("That key doesn't exist")))
            },
        }
    }
    fn remove_left(&mut self, target_key: i32) -> Result<i32, String> {
        let left = self.left.take();
        let (new_left, outcome) = self.remove_node(left, target_key);
        self.left = new_left;
        outcome
    }

    fn remove_right(&mut self, target_key: i32) -> Result<i32, String> {
        let right = self.right.take();
        let (new_right, outcome) = self.remove_node(right, target_key);
        self.right = new_right;
        outcome
    }

    pub fn remove(&mut self, key: i32) -> Result<i32, String> {
        match key {
            key if key < self.key => {
                self.remove_left(key)
            },
            key if key > self.key => {
                self.remove_right(key)
            },
            _ => Err(String::from("Trying to remove self. Needs to be done at parent level"))
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

    #[test]
    fn test_get() {
        let mut node = Node::new(1, 1);
        node.add((2, 3)).unwrap();
        assert!(node.get(3).unwrap() == 2);
    }

    #[test]
    fn test_cannot_find() {
        let node = Node::new(1, 1);
        assert!(node.get(3).is_err());
    }

    #[test]
    fn test_remove_left_childless() {
        let mut node = Node::new(1, 3);
        node.add((2, 1)).unwrap();
        node.remove(1).unwrap();
        assert!(node.left.is_none());
    }

    #[test]
    fn test_remove_left_one_child_right() {
        let mut node = Node::new(1, 4);
        node.add((2, 2)).unwrap();
        node.add((3, 3)).unwrap();
        node.remove(2).unwrap();
        assert!(node.left.unwrap().value == 3);
    }
    #[test]
    fn test_remove_left_one_child_left() {
        let mut node = Node::new(1, 4);
        node.add((3, 3)).unwrap();
        node.add((2, 2)).unwrap();
        node.remove(3).unwrap();
        assert!(node.left.unwrap().value == 2);
    }
    #[test]
    fn test_remove_left_two_children() {
        let mut node = Node::new(1, 5);
        node.add((2, 2)).unwrap();
        node.add((4, 4)).unwrap();
        node.add((3, 3)).unwrap();
        node.add((1, 1)).unwrap();
        node.remove(2).unwrap();
        assert!(node.left.unwrap().value == 3);
    }
    #[test]
    fn test_remove_right_childless() {
        let mut node = Node::new(1, 3);
        node.add((1, 4)).unwrap();
        node.remove(4).unwrap();
        assert!(node.right.is_none());
    }

    #[test]
    fn test_remove_right_one_child_left() {
        let mut node = Node::new(1, 4);
        node.add((2, 6)).unwrap();
        node.add((3, 5)).unwrap();
        node.remove(6).unwrap();
        assert!(node.right.unwrap().value == 3);
    }
    #[test]
    fn test_remove_right_one_child_right() {
        let mut node = Node::new(1, 1);
        node.add((3, 2)).unwrap();
        node.add((2, 3)).unwrap();
        node.remove(2).unwrap();
        assert!(node.right.unwrap().value == 2);
    }
    #[test]
    fn test_remove_right_two_children() {
        let mut node = Node::new(1, 1);
        node.add((2, 3)).unwrap();
        node.add((4, 5)).unwrap();
        node.add((3, 4)).unwrap();
        node.add((1, 2)).unwrap();
        node.remove(3).unwrap();
        assert!(node.right.unwrap().value == 3);
    }
}
