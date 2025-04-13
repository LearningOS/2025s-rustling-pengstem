/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T>
{
    // q1 is the main queue holding the stack elements
    q1: Queue<T>,
    // q2 is used as temporary storage during pop
    q2: Queue<T>
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new()
        }
    }

    // Push element onto the top of the stack
    pub fn push(&mut self, elem: T) {
        // Always enqueue to q1
        self.q1.enqueue(elem);
    }

    // Remove the element from the top of the stack and return it
    pub fn pop(&mut self) -> Result<T, &'static str> {
        // Check if q1 is empty initially
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // Move all elements except the last one from q1 to q2
        while self.q1.size() > 1 {
            // Dequeue from q1 and enqueue to q2.
            // Using if let to handle the Result without borrowing issues
            if let Ok(val) = self.q1.dequeue() {
                self.q2.enqueue(val);
            } else {
                // This should not happen based on the size check
                unreachable!();
            }
        }

        // Pop the last element from q1 (which is the top of our stack)
        let result = match self.q1.dequeue() {
            Ok(val) => Ok(val),
            Err(_) => Err("Stack is empty") // Using static lifetime for error string
        };

        // Swap q1 and q2 after we're done with q1
        std::mem::swap(&mut self.q1, &mut self.q2);
        
        result
    }

    // Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        // The stack is empty if the main queue (q1) is empty.
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = MyStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}