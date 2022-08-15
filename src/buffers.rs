use crate::datastructs::stack::Stack;

pub struct Stdout {
    pub buffer: Stack<u8>
}

impl Stdout {

    pub fn new() -> Self {
        let mut stack = Stack::new(0u8, 20000);
        stack.pop();
        Self { buffer: stack }
    }

    pub fn output(&mut self) {
        loop {
            if let Some(chr) = self.buffer.pop() {
                print!("{chr}");
            } else {
                break;
            }
        }
    }
}

pub struct Stderr {
    pub buffer: Stack<char>
}