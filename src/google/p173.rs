/*
 * Implement an iterator over a binary search tree (BST). Your iterator will be initialized with the root node of a BST.
 * Calling next() will return the next smallest number in the BST.
 *
 * Example:
 * BSTIterator iterator = new BSTIterator(root);
 * iterator.next();    // return 3
 * iterator.next();    // return 7
 * iterator.hasNext(); // return true
 * iterator.next();    // return 9
 * iterator.hasNext(); // return true
 * iterator.next();    // return 15
 * iterator.hasNext(); // return true
 * iterator.next();    // return 20
 * iterator.hasNext(); // return false
 */

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
}

#[derive(Debug)]
pub struct BinTree {
    pub root: Node,
}

type Node = Option<NodeRef>;
type NodeRef = Rc<RefCell<TreeNode>>;

impl TreeNode {

    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn add(&mut self, node: TreeNode) {
        let child = if node.val <= self.val {
            &mut self.left
        } else {
            &mut self.right
        };

        match child {
            None => {
                child.replace(Rc::new(RefCell::new(node)));
            }
            Some(child) => {
                child.borrow_mut().add(node)
            }
        }
    }

}

impl BinTree {
    pub fn new() -> Self {
        BinTree {
            root: None
        }
    }

    pub fn insert(&mut self, val: i32) {
        let new_node = TreeNode::new(val);
        match &self.root {
            None => {
                self.root = Some(Rc::new(RefCell::new(new_node)));
            },
            Some(node) => {
                node.borrow_mut().add(new_node)
            }
        }
    }
}

#[derive(Debug)]
pub struct BSTIterator {
    nodes: Vec<i32>
}

impl BSTIterator {
    pub fn new(root: Node) -> Self {
        let mut stack: Vec<i32> = vec![];
        let _nodes = BSTIterator::_in_order_traverse(root, &mut stack);
        BSTIterator {
            nodes: stack
        }
    }

    pub fn _in_order_traverse(root: Node, stack: &mut Vec<i32>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(node) => {
                BSTIterator::_in_order_traverse(node.as_ref().borrow().clone().left, stack);
                stack.push(node.borrow().val);
                BSTIterator::_in_order_traverse(node.as_ref().borrow().clone().right, stack)
            }
        }
    }

    pub fn has_next(&self) -> bool {
        return self.nodes.len() != 0;
    }

    pub fn next(&mut self) -> i32 {
        self.nodes.remove(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_tree() {
        let mut tree = BinTree::new();
        let values = vec![7, 3, 15, 9, 20];
        for val in values {
            tree.insert(val);
        }
        println!("tree: {:#?}", tree);
    }

    #[test]
    fn create_iter() {
        let mut tree = BinTree::new();
        let values = vec![7, 3, 15, 9, 20];
        for val in values {
            tree.insert(val);
        }
        let mut iter = BSTIterator::new(tree.root);
        assert_eq!(iter.next(), 3);
        assert_eq!(iter.next(), 7);
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), 9);
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), 15);
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), 20);
        assert_eq!(iter.has_next(), false);
    }
}
