use crate::pkg::traits::Codec;
use std::collections::HashMap;

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
            output: String::new(),
        }
    }
}

impl Codec for LempelZivWelch {
    fn encode(&mut self) {
        // if there is no text to process, abandon the call
        if self.text.is_empty() {
            return;
        }

        // use a hashmap to hold all ASCII characters for reference
        let mut size = 255_u8;
        let mut hashmap = (0..=size).fold(HashMap::new(), |mut hashmap, idx| {
            hashmap.insert((idx as char).to_string(), idx as u64);
            hashmap
        });

        // transform the data to a vector of characters
        let data = self.text.chars().collect::<Vec<char>>();

        // create a pattern to hold the common substrings in the text
        let mut pattern = data.first().unwrap().to_string();

        // continuing from the last recorded ASCII character, build more repeated patterns
        let mut size = (size as u64) + 1;
        for i in 0..data.len()-1 {
            // start a potential pattern
            let curr_char = data[i + 1];

            let mut new_pattern = pattern.clone();
            new_pattern.push(curr_char);

            // if the new pattern is in the hashmap,
            if hashmap.contains_key(&new_pattern) {
                // update the old pattern with the new one
                pattern = new_pattern;
            } else { // if the new pattern is not in the hashmap
                // add the old pattern to the encoded result
                self.encoded.push(hashmap.get(&pattern).unwrap().clone());
                // put the new pattern in the hashmap with the next index available as value
                hashmap.insert(new_pattern, size);
                size += 1;
                // start the pattern from the current character
                pattern = curr_char.to_string();
            }
        }
        self.encoded.push(hashmap.get(&pattern).unwrap().clone());
    }

    fn decode(&mut self) {
        // create a hashmap of byte number to the corresponding ASCII value
        let mut size = 255_u8;
        let mut hashmap = (0..=size).fold(HashMap::new(), |mut hashmap, idx| {
            hashmap.insert(idx as u64, (idx as char).to_string());
            hashmap
        });

        // initialize the current encoded index with the first value from the encoded vector
        let mut current_encoded_index = self.encoded[0];
        let mut next_encoded_value = 0u64;
        // get the corresponding string for the current encoded index
        let mut current_decoded_string = hashmap.get(&current_encoded_index).unwrap().clone();
        // get the first character of the current decoded string
        let mut first_char_decoded_string = current_decoded_string.chars().next().unwrap().to_string();

        // add the current decoded string to the output
        self.output.push_str(current_decoded_string.as_str());

        // initialize the count with the next ASCII value
        let mut count = (size as u64) + 1;

        // iterate over the encoded vector
        for i in 0..self.encoded.len()-1 {
            // get the next encoded value
            next_encoded_value = self.encoded[i+1];
            // if the next encoded value is in the hashmap, update the current decoded string
            // else append the first character of the decoded string to the current decoded string
            match hashmap.get(&next_encoded_value) {
                Some(value) => {
                    current_decoded_string = value.clone();
                },
                None => {
                    current_decoded_string.push_str(&first_char_decoded_string);
                }
            }
            // update the first character of the decoded string
            first_char_decoded_string = current_decoded_string.chars().next().unwrap().to_string();
            // create a new string to be inserted into the hashmap
            let v = format!("{}{}", hashmap.get(&current_encoded_index).unwrap(), first_char_decoded_string);
            // add the current decoded string to the output
            self.output.push_str(&current_decoded_string);
            // insert the new string into the hashmap
            hashmap.insert(count, v);
            count += 1;
            // update the current encoded index
            current_encoded_index = next_encoded_value;
        }

        // add the last decoded string to the output
        format!("{}{}", hashmap.get(&current_encoded_index).unwrap(), first_char_decoded_string);
    }

    fn compressed(&self) -> String {
        self.encoded.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(" ")
    }

    fn decompressed(&self) -> String {
        self.output.clone()
    }
}
