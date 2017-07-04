use node::Node;
use errors::Errors;

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

    pub fn add(&mut self, new_node: (i32, i32)) -> Result<(), Errors> {
        let root = &mut self.root;
        if let &mut Some(ref mut node) = root {
            node.add(new_node)
        } else {
            *root = Some(Node::new(new_node));
            Ok(())
        }
    }

    pub fn get(&self, key: i32) -> Result<i32, Errors> {
        let BST { ref root, .. } = *self;
        match *root {
            Some(ref node) => node.get(key),
            None => Err(Errors::EmptyTree),
        }
    }

    pub fn remove(&mut self, target_key: i32) -> Result<i32, Errors> {
        let BST { ref mut root, .. } = *self;
        match *root {
            Some (_) => {
                let key = root.as_ref().unwrap().key;
                if key == target_key {
                    let value = root.as_ref().unwrap().value;
                    *root = None;
                    Ok(value)
                } else {
                    root.as_mut().unwrap().remove(target_key)
                }
            },
            None => Err(Errors::EmptyTree),
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

    #[test]
    fn test_get() {
        let mut bst = BST::new();
        bst.add((1, 2)).unwrap();
        bst.add((2, 4)).unwrap();
        assert!(bst.get(4).unwrap() == 2);
    }

    #[test]
    fn test_get_empty() {
        let bst = BST::new();
        assert!(bst.get(3).is_err());
    }

    #[test]
    fn remove_root() {
        let mut bst = BST::new();
        bst.add((1, 1)).unwrap();
        bst.remove(1).unwrap();
        assert!(bst.root.is_none());
    }

    #[test]
    fn remove_empty() {
        let mut bst = BST::new();
        assert!(bst.remove(1).is_err());
    }
}
