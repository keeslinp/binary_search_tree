use errors::Errors;

#[derive(Debug)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    pub key: i32,
    pub value: i32,
}


impl Node {
    pub fn new((value, key): (i32, i32)) -> Box<Node> {
        Box::from(Node {
            left: None,
            right: None,
            key: key,
            value: value,
        })
    }

    fn add_left(&mut self, new_node : (i32, i32)) -> Result<(), Errors> {
        let Node { ref mut left, .. } = *self;
        if let &mut Some(ref mut child) = left {
            child.add(new_node)
        } else {
            *left = Some(Node::new(new_node));
            Ok(())
        }
    }

    fn add_right(&mut self, new_node: (i32, i32)) -> Result<(), Errors> {
        let Node { ref mut right, .. } = *self;
        if let &mut Some(ref mut child) = right {
            child.add(new_node)
        } else {
            *right = Some(Node::new(new_node));
            Ok(())
        }
    }

    pub fn add(&mut self, (val , key): (i32, i32)) -> Result<(), Errors>{

        if key < self.key {
            self.add_left((val, key))
        } else if key > self.key {
            self.add_right((val, key))
        } else {
            Err(Errors::DuplicateKey)
        }
    }

    fn get_left(&self, key: i32) -> Result<i32, Errors> {
        let Node { ref left, .. } = *self;
        if let &Some(ref child) = left {
            child.get(key)
        } else {
            Err(Errors::DuplicateKey)
        }
    }
    
    fn get_right(&self, key: i32) -> Result<i32, Errors> {
        let Node { ref right, .. } = *self;
        if let &Some(ref child) = right {
            child.get(key)
        } else {
            Err(Errors::DoesNotExist)
        }
    }

    pub fn get(&self, key: i32) -> Result<i32, Errors> {
        if key < self.key {
            self.get_left(key)
        } else if key > self.key {
            self.get_right(key)
        } else {
            Ok(self.value)
        }
    }

    fn get_last_right(&self) -> &Node {
        self.right.as_ref().map(|left| left.get_last_right()).unwrap_or(&self)
    }

    fn get_last_left(&self) -> &Node {
        self.left.as_ref().map(|left| left.get_last_left()).unwrap_or(&self)
    }

    fn remove_node(mut self: Box<Self>, target_key: i32) -> (Option<Box<Node>>, Result<i32, Errors>) {
        if self.key == target_key {
            let val = self.value;
            // Anyone wondering about the funky tuple magic here should check
            // https://github.com/rust-lang/rust/issues/16223#issuecomment-307237373
            ( match (self,) {
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
                    let mut new_self = Box::from(Node { 
                        value,
                        key,
                        left: Some(child_left),
                        right: Some(child_right),
                    });
                    new_self.remove_right(key).unwrap();
                    Some(new_self)
                },
            }, Ok(val))
        } else {
            let resp = self.remove(target_key);
            (Some(self), resp)
        }
    }
    fn remove_left(&mut self, target_key: i32) -> Result<i32, Errors> {
        let left = self.left.take();
        let (new_left, outcome) = if let Some(child) = left {
            child.remove_node(target_key)
        } else {
            (None, Err(Errors::DoesNotExist))
        };

        self.left = new_left;
        outcome
    }

    fn remove_right(&mut self, target_key: i32) -> Result<i32, Errors> {
        let right = self.right.take();
        let (new_right, outcome) = if let Some(child) = right {
            child.remove_node(target_key)
        } else {
            (None, Err(Errors::DoesNotExist))
        };
        self.right = new_right;
        outcome
    }

    pub fn remove(&mut self, key: i32) -> Result<i32, Errors> {
        match key {
            key if key < self.key => {
                self.remove_left(key)
            },
            key if key > self.key => {
                self.remove_right(key)
            },
            _ => Err(Errors::WrongLevel)
        }
    }
}

#[cfg(test)]
mod test {
    use node::Node;
    #[test]
    fn test_add_left() {
        let mut node = Node::new((1, 2));
        node.add((1, 1)).unwrap();
        assert!(node.left.unwrap().value == 1);
    }
    #[test]
    fn test_add_left_right() {
        let mut node = Node::new((1, 3));
        node.add((2, 1)).unwrap();
        node.add((3, 2)).unwrap();
        assert!(node.left.unwrap().right.unwrap().value == 3);
    }
    #[test]
    fn test_add_right() {
        let mut node = Node::new((1, 1));
        node.add((1, 2)).unwrap();
        assert!(node.right.unwrap().value == 1);
    }
    #[test]
    fn test_add_right_left() {
        let mut node = Node::new((1, 1));
        node.add((2, 3)).unwrap();
        node.add((3, 2)).unwrap();
        assert!(node.right.unwrap().left.unwrap().value == 3);
    }

    #[test]
    fn test_add_duplicate() {
        let mut node = Node::new((1, 1));
        assert!(node.add((12, 1)).is_err());
    }

    #[test]
    fn test_get() {
        let mut node = Node::new((1, 1));
        node.add((2, 3)).unwrap();
        assert!(node.get(3).unwrap() == 2);
    }

    #[test]
    fn test_cannot_find() {
        let node = Node::new((1, 1));
        assert!(node.get(3).is_err());
    }

    #[test]
    fn test_remove_left_childless() {
        let mut node = Node::new((1, 3));
        node.add((2, 1)).unwrap();
        node.remove(1).unwrap();
        assert!(node.left.is_none());
    }

    #[test]
    fn test_remove_left_one_child_right() {
        let mut node = Node::new((1, 4));
        node.add((2, 2)).unwrap();
        node.add((3, 3)).unwrap();
        node.remove(2).unwrap();
        assert!(node.left.unwrap().value == 3);
    }
    #[test]
    fn test_remove_left_one_child_left() {
        let mut node = Node::new((1, 4));
        node.add((3, 3)).unwrap();
        node.add((2, 2)).unwrap();
        node.remove(3).unwrap();
        assert!(node.left.unwrap().value == 2);
    }
    #[test]
    fn test_remove_left_two_children() {
        let mut node = Node::new((1, 5));
        node.add((2, 2)).unwrap();
        node.add((4, 4)).unwrap();
        node.add((3, 3)).unwrap();
        node.add((1, 1)).unwrap();
        node.remove(2).unwrap();
        assert!(node.left.unwrap().value == 3);
    }
    #[test]
    fn test_remove_right_childless() {
        let mut node = Node::new((1, 3));
        node.add((1, 4)).unwrap();
        node.remove(4).unwrap();
        assert!(node.right.is_none());
    }

    #[test]
    fn test_remove_right_one_child_left() {
        let mut node = Node::new((1, 4));
        node.add((2, 6)).unwrap();
        node.add((3, 5)).unwrap();
        node.remove(6).unwrap();
        assert!(node.right.unwrap().value == 3);
    }
    #[test]
    fn test_remove_right_one_child_right() {
        let mut node = Node::new((1, 1));
        node.add((3, 2)).unwrap();
        node.add((2, 3)).unwrap();
        node.remove(2).unwrap();
        assert!(node.right.unwrap().value == 2);
    }
    #[test]
    fn test_remove_right_two_children() {
        let mut node = Node::new((1, 1));
        node.add((2, 3)).unwrap();
        node.add((4, 5)).unwrap();
        node.add((3, 4)).unwrap();
        node.add((1, 2)).unwrap();
        node.remove(3).unwrap();
        assert!(node.right.unwrap().value == 3);
    }
}
