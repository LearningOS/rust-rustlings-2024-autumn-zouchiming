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

pub struct myStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
    }
    
    impl<T> myStack<T> {
    pub fn new() -> Self {
    Self {
    q1: Queue::new(),
    q2: Queue::new(),
    }
    }
    
    pub fn push(&mut self, elem: T) {
    // 将元素加入队列 q2
    self.q2.enqueue(elem);
    
    // 将 q1 中的所有元素移到 q2 中
    while let Ok(value) = self.q1.dequeue() {
    self.q2.enqueue(value);
    }
    
    // 将 q2 赋值给 q1
    std::mem::swap(&mut self.q1, &mut self.q2);
    }
    
    pub fn pop(&mut self) -> Result<T, &str> {
        // 确保 q1 不为空
        if self.q1.is_empty() {
        return Err("Stack is empty");
        }
        
        // 从 q1 中弹出元素
        let mut value = self.q1.dequeue().unwrap(); // 获取并移除元素
        // 如果需要进行更多的操作，例如维护 q2，这里可以加入逻辑
        
        // 返回弹出的值
        Ok(value)
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
		let mut s = myStack::<i32>::new();
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