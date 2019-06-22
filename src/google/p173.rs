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

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Node,
  pub right: Node,
}

type Node = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub struct BSTIterator {
    pub root: Node
}


impl BSTIterator {

    pub fn new(root: Node) -> Self {
        BSTIterator {
            root: root
        }
    }

    /** @return the next smallest number */
    pub fn next(&self) -> i32 {
        return -1
    }

    /** @return whether we have a next smallest number */
    pub fn has_next(&self) -> bool {
        return false
    }
}

#[cfg(test)]
mod test {
    use super::{BSTIterator, TreeNode};

    #[test]
    fn example1() {
        let mut root = TreeNode::new(7);
        println!("root: {:?}", root);
        assert_eq!(1, 1);
    }
}
