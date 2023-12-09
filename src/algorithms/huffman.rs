use crate::data_structures::huffman_node::{HuffmanLeaf, HuffmanNode, Link, Node};
use crate::data_structures::priority_queue::Heap;
use crate::pkg::traits::Codec;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Huffman represents the implementation of the Huffman Encoding algorithm
pub struct Huffman {
    root: Link,
    text: String,
    char_frequencies: HashMap<char, i32>,
    char_encodings: HashMap<char, String>,
    encoded: String,
    decoded: String,
}

impl Huffman {
    pub fn new(text: String) -> Self {
        let mut huffman = Self {
            root: None,
            text,
            char_frequencies: Default::default(),
            char_encodings: Default::default(),
            encoded: String::new(),
            decoded: String::new(),
        };

        huffman.populate_char_frequencies();

        huffman
    }

    fn populate_char_frequencies(&mut self) {
        self.char_frequencies = self.text.chars().fold(HashMap::new(), |mut hashmap, ch| {
            *hashmap.entry(ch).or_insert(0) += 1;
            hashmap
        });
    }

    fn generate(&mut self, curr_node: Link, curr_code: &str) {
        if let Some(curr_node) = curr_node {
            let curr_node = curr_node.borrow();
            if curr_node.is_leaf() {
                self.char_encodings
                    .insert(curr_node.get_character().unwrap(), curr_code.to_string());
                return;
            }

            self.generate(curr_node.get_left(), format!("{}0", curr_code).as_str());
            self.generate(curr_node.get_right(), format!("{}1", curr_code).as_str());
        }
    }

    fn get_encoded_text(&mut self) {
        let mut priority_queue: Heap<Box<dyn Node>> = Heap::with_capacity(self.text.len());
        for (k, v) in &self.char_frequencies {
            priority_queue.push(Box::new(HuffmanLeaf::new(*k, *v)));
        }

        while priority_queue.size() > 1 {
            let first = priority_queue.pop().unwrap();
            let second = priority_queue.pop().unwrap();
            priority_queue.push(Box::new(HuffmanNode::new(
                Some(Rc::new(RefCell::new(first))),
                Some(Rc::new(RefCell::new(second))),
            )));
        }

        let root = priority_queue.pop().unwrap();
        self.root = Some(Rc::new(RefCell::new(root)));
        self.generate(self.root.clone(), "");
    }
}

impl Codec for Huffman {
    fn encode(&mut self) {
        self.get_encoded_text();
        for ch in self.text.chars() {
            let s = self.char_encodings.get(&ch).unwrap();
            self.encoded.push_str(s);
        }
    }

    fn decode(&mut self) {
        let curr_node = self.root.clone();
        if let Some(mut curr_node) = curr_node {
            for ch in self.encoded.chars() {
                curr_node = if ch == '0' {
                    curr_node.borrow().get_left()
                } else {
                    curr_node.borrow().get_right()
                }
                .unwrap();

                if curr_node.borrow().is_leaf() {
                    self.decoded
                        .push(curr_node.borrow().get_character().unwrap());
                    curr_node = self.root.clone().unwrap();
                }
            }
        }
    }

    fn compressed(&self) -> String {
        self.encoded.clone()
    }

    fn decompressed(&self) -> String {
        self.decoded.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn huffman_works() {
        let test_cases = vec!["abracadabra!", "aabbc", "aaaaaaaabbbbbbbccccdd", "12ab"];

        for test_case in test_cases {
            let mut huffman = Huffman::new(test_case.to_string());
            huffman.encode();
        }
    }

    #[test]
    fn decoder_works() {
        let test_cases = vec!["abracadabra!", "aabbc", "aaaaaaaabbbbbbbccccdd", "12ab"];

        for test_case in test_cases {
            let mut huffman = Huffman::new(test_case.to_string());
            huffman.encode();
            huffman.decode();
            assert_eq!(huffman.decoded, test_case.to_string());
        }
    }
}
