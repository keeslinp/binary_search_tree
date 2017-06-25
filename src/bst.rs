use node::Node;
pub struct BST {
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


    pub fn add(&mut self, (value, key): (i32, i32)) -> Result<(), String> {
        let BST { ref mut root, .. } = *self;
        match *root {
            Some(ref mut node) => node.add((value, key)),
            None => {
                *root = Some(Node::new(value, key));
                Ok(())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use bst::BST;
    #[test]
    fn it_works() {
    }
    #[test]
    fn test_add_on_empty() {
        let mut bst = BST::new();
        bst.add((1, 2)).unwrap();
        assert!(bst.root.is_some());
    }
}
