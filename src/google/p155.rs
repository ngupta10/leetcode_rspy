/*
 * Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
 *
 *      push(x) -- Push element x onto stack.
 *      pop() -- Removes the element on top of the stack.
 *      top() -- Get the top element.
 *      getMin() -- Retrieve the minimum element in the stack.
 *
 * Examples:
 * ----------
 * MinStack minStack = new MinStack();
 * minStack.push(-2);
 * minStack.push(0);
 * minStack.push(-3);
 * minStack.getMin();   --> Returns -3.
 * minStack.pop();
 * minStack.top();      --> Returns 0.
 * minStack.getMin();   --> Returns -2.
 */

#[derive(Debug)]
pub struct MinStack {
    items: Vec<i32>,
    min_items: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            min_items: vec![],
            items: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.items.insert(0, x);
        if self.min_items.len() == 0 {
            self.min_items.push(x)
        } else {
            if x <= self.min_items[0] {
                self.min_items.insert(0, x)
            }
        }
    }

    fn pop(&mut self) {
        let x = self.items.remove(0);
        if x == self.min_items[0] {
            self.min_items.remove(0);
        }
    }

    fn top(&self) -> i32 {
        self.items[0]
    }

    fn get_min(&self) -> i32 {
        self.min_items[0]
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod test {
    use super::MinStack;

    #[test]
    fn example1() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        println!("stack: {:?}", stack);
        stack.pop();
        println!("stack: {:?}", stack);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn example2() {
        let mut stack = MinStack::new();
        stack.push(0);
        stack.push(1);
        stack.push(0);
        println!("stack: {:?}", stack);
        assert_eq!(stack.get_min(), 0);
        stack.pop();
        println!("stack: {:?}", stack);
        assert_eq!(stack.get_min(), 0);
    }

}
