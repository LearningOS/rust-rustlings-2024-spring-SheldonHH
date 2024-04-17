/*
	queue
	This question requires you to use queues to implement the functionality of the stac
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
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        self.elements.first().ok_or("Queue is empty")
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
        Queue::new()
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.q2.enqueue(elem);
        while let Ok(elem) = self.q1.dequeue() {
            self.q2.enqueue(elem);
        }
        std::mem::swap(&mut self.q1, &mut self.q2);
    }
    // 🔥
    // 需要修改 MyStack 类的 pop 方法，确保当队列为空时，返回的错误信息与测试用例中预期的一致。这可以通过捕获 Queue 的错误信息并替换为 "Stack is empty" 来实现。
    pub fn pop(&mut self) -> Result<T, &str> {
        self.q1.dequeue().or(Err("Stack is empty"))
    }

    pub fn is_empty(&self) -> bool {
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