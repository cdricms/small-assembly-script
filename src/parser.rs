use std::fs;

use crate::{common::RES_KEYWORDS, datastructs::queue::Queue};

pub struct Parser {
    pub commands: Queue<Queue<String>>
}

impl Parser {

    pub fn new() -> Self {
        Self { commands: Queue::new_empty(20000)}
    }

    pub fn is_other_base(&self, other: &str) -> bool {
        if other.len() > 2 {
            let b = other.split_at(2).0;
            return b == "0b" || b == "0x";
        }

        false
    }

    pub fn parse(&mut self, path: String) {
        let file = fs::read_to_string(&path).unwrap();

        let mut skip_comments = false;
        let mut commands: Queue<Queue<String>> = Queue::new_empty(20000);
        let mut current_word = String::new();

        for line in file.lines() {
            let mut inside_q = Queue::new_empty(5);
            for (idx, char) in line.chars().enumerate() {
                match char {
                    ';' => skip_comments = true,
                    _ if !skip_comments => {
                        // TODO: Doesn't seem to work even though, the math is right.
                        // Use spaces at the end of every line
                        if idx + 1 == line.chars().count() || char == ' ' {
                            if RES_KEYWORDS.contains(&current_word.to_uppercase().as_str()) {
                                inside_q.push(current_word.clone());
                            } else {
                                if let Ok(value) = current_word.parse::<i32>() {
                                    inside_q.push(value.to_string());
                                } else {
                                    if self.is_other_base(current_word.as_str()) {
                                        inside_q.push(current_word.to_string());
                                    }
                                }
                            }
                            current_word.clear();

                        } else {
                            current_word.push_str(&char.to_string());
                        }
                    },
                    _ => break
                } 
            }
            commands.push(inside_q);
            skip_comments = false;
        }

        self.commands = commands;

    }

}