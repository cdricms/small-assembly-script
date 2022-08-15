use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Queue<T: Debug + Clone> {
    items: Vec<T>,
    pub length: usize,
}

impl<T: Debug + Clone> Queue<T> {
    pub fn new_empty(max_size: usize) -> Self {
        if max_size < 1 {
            panic!("Queue max size must be over 0");
        }
        let items: Vec<T> = Vec::with_capacity(max_size);
        Self { items, length: 0 }

    }
    pub fn new(head: T, max_size: usize) -> Self {
        let mut q = Self::new_empty(max_size);
        q.push(head);
        q
    }


    pub fn push(&mut self, value: T) {
        if self.length > self.items.len() {
            return;
        }
        self.items.push(value);
        self.length += 1;
    }

    pub fn head(&mut self) -> Option<T> {
        let head = self.pop(); 
        if let Some(head) = head.clone() {
            self.push(head);
        }
        head
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.items.is_empty() {
            self.length -= 1;
            return Some(self.items.remove(0));
        }

        None
    }

}