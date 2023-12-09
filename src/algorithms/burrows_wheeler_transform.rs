pub mod quadratic_log {
    use std::collections::VecDeque;

    use crate::pkg::traits::Codec;

    /// Rotation represents a rotation of a string at an index
    #[derive(Debug)]
    struct Rotation(usize, String);

    impl Rotation {
        fn new(i: usize, s: String) -> Self {
            Rotation(i, s)
        }
    }

    /// BurrowsWheelerTransform represents a struct for the BWT compression algorithm
    pub struct BurrowsWheelerTransform {
        text: String,
        suffix_indices: Vec<usize>,
        encoded: String,
        original_pos: usize,
        decoded: String,
    }

    impl BurrowsWheelerTransform {
        pub fn new(mut text: String) -> Self {
            for ch in text.chars() {
                if ch == '$' {
                    panic!("The text must not contain the '$' character");
                }
            }
            text.push('$');

            let n = text.len();
            Self {
                text,
                suffix_indices: Vec::with_capacity(n),
                encoded: String::new(),
                original_pos: 0,
                decoded: String::new(),
            }
        }

        /// populate_suffix_vector gets a suffix array of rotations from the input string
        fn populate_suffix_vector(&mut self) {
            let text_length = self.text.len();
            let mut rotations: Vec<Rotation> = Vec::with_capacity(text_length);

            // for each character and index i in the input string, rotate it at i
            self.text.chars().enumerate().for_each(|(i, _)| {
                rotations.push(Rotation::new(i, (&self.text[i..]).to_string()));
            });

            // sort the rotations by the suffix strings
            rotations.sort_by(|a, b| a.1.cmp(&b.1));

            rotations.iter().enumerate().for_each(|(i, r)| {
                if r.1 == self.text {
                    self.original_pos = i;
                }
            });

            // get the indices of the sorted rotations in their order and set the suffixes field
            rotations.iter().for_each(|r| self.suffix_indices.push(r.0));
        }

        fn transform(&mut self) {
            let text_characters = self.text.chars().collect::<Vec<char>>();
            let text_length = self.text.len();

            // for each index in the stored suffixes
            self.suffix_indices.iter().for_each(|idx| {
                let ch_idx = match *idx {
                    0 => text_length - 1, // wrap the text at the last position to prevent underflow
                    _ => *idx - 1,        // get the previous position
                };
                // add the character at that index to the encoded
                self.encoded.push(text_characters[ch_idx]);
            });
        }

        fn invert_transform(&mut self) {
            let encoded_vector = self.encoded.chars().collect::<Vec<char>>();
            let mut sorted_encoded_vector = encoded_vector.clone();
            sorted_encoded_vector.sort();
            let encoded_len = encoded_vector.len();
            let mut left_shifts: Vec<usize> = vec![0; encoded_len];
            let mut curr_start_pos = self.original_pos;

            let mut int_arr: Vec<VecDeque<usize>> = vec![VecDeque::new(); 128];
            for i in 0..int_arr.len() {
                int_arr[i] = VecDeque::new();
            }

            // use the int_arr as a map of character to a queue of the character's
            // occurrences in the encoded vector in order of magnitude
            for i in 0..encoded_len {
                int_arr[(encoded_vector[i] as u8) as usize].push_back(i);
            }

            for i in 0..encoded_len {
                let queue_idx = (sorted_encoded_vector[i] as u8) as usize;
                left_shifts[i] = int_arr[queue_idx].pop_front().unwrap();
            }

            let mut decoded: Vec<char> = vec!['\0'; encoded_len];
            for i in 0..encoded_len {
                curr_start_pos = left_shifts[curr_start_pos];
                decoded[encoded_len - 1 - i] = encoded_vector[curr_start_pos];
            }

            self.decoded = decoded.iter().rev().collect::<String>();
            // pop the '$' we added
            self.decoded.pop();
        }
    }

    impl Codec for BurrowsWheelerTransform {
        fn encode(&mut self) {
            if self.text.is_empty() {
                return;
            }
            self.populate_suffix_vector();
            self.transform();
        }

        fn decode(&mut self) {
            self.invert_transform();
        }

        fn compressed(&self) -> String {
            self.encoded.clone()
        }

        fn decompressed(&self) -> String {
            self.decoded.clone()
        }
    }
}
