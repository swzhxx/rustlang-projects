use std::fmt::{Binary, Debug, Display};

type Link<T> = Option<Box<BinaryTree<T>>>;

#[derive(Debug, Clone)]
struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Clone> BinaryTree<T> {
    fn new(key: T) -> Self {
        BinaryTree {
            key,
            left: None,
            right: None,
        }
    }

    fn insert_left_tree(&mut self, key: T) {
        if self.left.is_none() {
            let node = BinaryTree::new(key);
            self.left = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node))
        }
    }

    fn insert_right_tree(&mut self, key: T) {
        if self.right.is_none() {
            let node = BinaryTree::new(key);
            self.right = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }
}

impl<T: Clone> BinaryTree<T> {
    fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    fn get_key(&self) -> T {
        self.key.clone()
    }

    fn set_key(&mut self, key: T) {
        self.key = key
    }
}

impl<T: Clone + Debug> BinaryTree<T> {
    // 前序遍历
    fn preorder(&self) {
        println!("kes {:?}", &self.key);
        if !self.left.is_none() {
            self.left.as_ref().unwrap().preorder()
        }
        if !self.right.is_none() {
            self.right.as_ref().unwrap().preorder()
        }
    }

    // 后序遍历
    fn postorder(&self) {
        if !self.left.is_none() {
            self.left.as_ref().unwrap().postorder();
        }
        if !self.right.is_none() {
            self.right.as_ref().unwrap().postorder();
        }
        println!("key is {:?}", &self.key);
    }

    fn inorder(&self) {
        if !self.left.is_none() {
            self.left.as_ref().unwrap().inorder()
        }
        println!("key is {:?}", &self.key);
        if !self.right.is_none() {
            self.right.as_ref().unwrap().inorder();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_binary_tree() {
        let mut bt = BinaryTree::new('a');
        let root = bt.get_key();
        assert_eq!(root, 'a');

        let left = bt.get_left();
        assert!(left.is_none());
        let right = bt.get_right();
        assert!(right.is_none());

        bt.insert_left_tree('b');
        bt.insert_right_tree('e');

        let left = bt.get_left();
        let right = bt.get_right();

        assert_eq!('b', left.unwrap().get_key());
        assert_eq!('e', right.unwrap().get_key());
    }
}
