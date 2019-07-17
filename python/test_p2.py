'''
You are given two non-empty linked lists representing two non-negative integers.
The digits are stored in reverse order and each of their nodes contain a single digit.
Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example:
Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
Output: 7 -> 0 -> 8
Explanation: 342 + 465 = 807.
'''
import pytest

class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

    def __repr__(self):
        return f'ListNode(val={self.val}, next={self.next})'

def from_list(l):
    ''' Create linked list from list of integers
    '''
    nodes = [ListNode(i) for i in l]
    current_index = 0
    while current_index < len(nodes) - 1:
        nodes[current_index].next = nodes[current_index + 1]
        current_index += 1

    return nodes[0]

def to_list(n):
    ''' Create a list from node
    '''
    stack = []
    while n:
        stack.append(n.val)
        n = n.next
    return stack

class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        n1 = int(''.join(map(str, reversed(to_list(l1)))))
        n2 = int(''.join(map(str, reversed(to_list(l2)))))
        return from_list([int(i) for i in reversed(str(n1 + n2))])

def test_conversion():
    l1 = [2, 4, 3]
    assert to_list(from_list(l1)) == l1

def test_example1():
    l1 = from_list([2, 4, 3])
    l2 = from_list([5, 6, 4])
    ans = from_list([7, 0, 8])

    sol = Solution()
    assert to_list(sol.addTwoNumbers(l1, l2)) == to_list(ans)

def test_failed1():
    l1 = from_list([1, 8])
    l2 = from_list([0])
    ans = from_list([1, 8])
    sol = Solution()
    assert to_list(sol.addTwoNumbers(l1 ,l2)) == to_list(ans)
