'''
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
'''

class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

    def __repr__(self):
        return f'TreeNode(val={self.val}, left={self.left}, right={self.right})'

    def add(self, node):
        if node.val <= self.val:
            if self.left is None:
                self.left = node
            else:
                self.left.add(node)
        else:
            if self.right is None:
                self.right = node
            else:
                self.right.add(node)


class BinTree:
    def __init__(self):
        self.root = None

    def __repr__(self):
        return f'BinTree(root={self.root})'

    def insert(self, node):
        if self.root is None:
            self.root = node
        else:
            self.root.add(node)


class BSTIterator:
    def __init__(self, root: TreeNode):
        self.nodes = self._traverse_inorder(root, [])

    def _traverse_inorder(self, root, nodes):
        '''

        '''
        if root is None:
            return nodes
        else:
            if root.left is not None:
                self._traverse_inorder(root.left, nodes)
            nodes.append(root.val)
            if root.right is not None:
                self._traverse_inorder(root.right, nodes)
        return nodes

    def next(self) -> int:
        '''
        @return the next smallest number
        '''
        return self.nodes.pop(0)

    def hasNext(self) -> bool:
        '''
        @return whether we has a next smaller number
        '''
        return len(self.nodes) != 0

def test_iterator():
    values = [7, 3, 15, 9, 20]
    nodes = [TreeNode(val) for val in values]
    print(f'nodes: {nodes}')
    tree = BinTree()
    for node in nodes:
        tree.insert(node)
    print(f'tree: {tree}')

    obj = BSTIterator(tree.root)

    assert obj.next() == 3
    assert obj.next() == 7
    assert obj.hasNext() == True
    assert obj.next() == 9
    assert obj.hasNext() == True
    assert obj.next() == 15
    assert obj.hasNext() == True
    assert obj.next() == 20
    assert obj.hasNext() == False
