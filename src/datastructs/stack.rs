use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Stack<T: Debug + Clone> {
    items: Vec<T>,
    length: usize
}


impl<T: Debug + Clone> Stack<T> {
    pub fn new_empty(max_size: usize) -> Self {
        if max_size < 1 {
            panic!("Queue max size must be over 0");
        }
        let items: Vec<T> = Vec::with_capacity(max_size);
        Self { items, length: 0 }

    }
    pub fn new(head: T, max_size: usize) -> Self {
        let mut s = Self::new_empty(max_size);
        s.push(head);
        s
    }

    pub fn head(&mut self) -> Option<T> {
        let head = self.pop(); 
        if let Some(head) = head.clone() {
            self.push(head);
        }
        head
    }

    pub fn push(&mut self, value: T) {
        if self.length > self.items.len() {
            return;
        }
        self.items.push(value);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.items.is_empty() {
            self.length -= 1;
        }
        self.items.pop()
    }

}
