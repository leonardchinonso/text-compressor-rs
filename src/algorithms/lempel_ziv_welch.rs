use std::collections::HashMap;
use crate::pkg::traits::Codec;

/// LempelZivWelch represents a struct for the LZW compression algorithm
pub struct LempelZivWelch {
    text: String,
    encoded: Vec<u64>,
    output: String,
}

impl LempelZivWelch {
    pub fn new(text: String) -> Self {
        Self {
            text,
            encoded: Vec::new(),
            output: String::new()
        }
    }
}

impl Codec for LempelZivWelch {
    fn encode(&mut self) {
        let mut size = 255_u8;
        let mut hashmap = (0..=size).fold(
            HashMap::new(),
            |mut hashmap, idx| {
                hashmap.insert((idx as char).to_string(), idx as u64);
                hashmap
            }
        );

        let mut size = (size as u64) + 1;
        let mut found_chars = String::new();
        for ch in self.text.chars() {
            let chars_to_add = format!("{}{}", found_chars, ch);
            match hashmap.contains_key(&chars_to_add) {
                true => { found_chars = chars_to_add; },
                false => {
                    self.encoded.push(hashmap.get(&found_chars).unwrap().clone());
                    size += 1;
                    hashmap.insert(chars_to_add, size);
                    found_chars = ch.to_string();
                },
            }
        }

        if found_chars.is_empty() {
            self.encoded.push(hashmap.get(&found_chars).unwrap().clone());
        }
    }

    fn decode(&mut self) {
        let mut size = 255_u8;
        let mut hashmap = (0..=size).fold(
            HashMap::new(),
            |mut hashmap, idx| {
                hashmap.insert(idx as u64, (idx as char).to_string());
                hashmap
            }
        );

        let mut size = (size as u64) + 1;
        // truncation will happen as intended, self.encoded[0] will never be size 16 integer
        let mut chs = vec![((self.encoded[0] as u8) as char)];
        for i in 1..self.encoded.len() {
            let code = self.encoded[i];
            let entry = match hashmap.contains_key(&code) {
                true => hashmap.get(&code).unwrap().clone().chars().collect::<Vec<char>>(),
                false => {
                    let mut v = chs.clone();
                    v.push(chs[0]);
                    v
                },
            };
            self.output.push_str(entry.iter().collect::<String>().as_str());
            size += 1;
            let mut new_entry = chs.iter().collect::<String>();
            new_entry.push(entry[0]);
            hashmap.insert(size, new_entry);
            chs = entry;
        }
    }

    fn compressed(&self) -> String {
        println!("{:?}", self.encoded.clone());
        "".to_string()
    }

    fn decompressed(&self) -> String {
        self.output.clone()
    }
}