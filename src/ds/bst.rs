/* A boxed version of binary search tree
 */

#[derive(Debug)]
pub struct Node {
    val: i32,
    left: Option<NodeRef>,
    right: Option<NodeRef>
}

type NodeRef = Box<Node>;

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val: val,
            left: None,
            right: None
        }
    }

    pub fn add(&mut self, node: Node) {
        let child = if node.val <= self.val {
            &mut self.left
        } else {
            &mut self.right
        };

        match child {
            None => {
                child.replace(Box::new(node));
            }
            Some(child) => {
                child.as_mut().add(node)
            }
        }
    }

    pub fn inorder(&self, nodes: &mut Vec<i32>) {
        match &self.left {
            None => (),
            Some(left) => {
                left.inorder(nodes);
            }
        }
        nodes.push(self.val);
        match &self.right {
            None => (),
            Some(right) => {
                right.inorder(nodes);
            }
        }
    }
}

#[derive(Debug)]
pub struct BinTree {
    root: Option<NodeRef>,
}

impl BinTree {
    pub fn new() -> Self {
        BinTree {
            root: None
        }
    }

    pub fn insert(&mut self, n: Node) {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(n))
            }
            Some(node) => {
                node.as_mut().add(n)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.root {
            None => true,
            Some(_) => false
        }
    }

    pub fn inorder(&self) -> Vec<i32> {
        let mut nodes = vec![];
        match &self.root {
            None => {
                nodes
            }
            Some(node) => {
                node.as_ref().inorder(&mut nodes);
                nodes
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{BinTree, Node};

    #[test]
    fn build_tree() {
        let mut bintree = BinTree::new();
        let values = vec![7, 3, 15, 9, 20];
        for val in values {
            let new_node = Node::new(val);
            bintree.insert(new_node);
        }
        assert_eq!(bintree.is_empty(), false);
    }

    #[test]
    fn in_order_test() {
        let mut bintree = BinTree::new();
        let values = vec![7, 3, 15, 9, 20];
        for val in values {
            let new_node = Node::new(val);
            bintree.insert(new_node);
        }
        assert_eq!(bintree.inorder(), vec![3, 7, 9, 15, 20]);
    }
}
