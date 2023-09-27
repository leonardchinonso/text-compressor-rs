use crate::pkg::traits::Codec;

/// RLE is the zero product struct for the RUN LENGTH ENCODING algorithm
pub struct Rle {
    input: String,
    encoded: String,
    output: String,
}

impl Rle {
    pub fn new(text: String) -> Self {
        Self {
            input: text,
            encoded: String::new(),
            output: String::new(),
        }
    }
}

/// Codec trait implementation for the RLE algorithm
impl Codec for Rle {
    /// encode compresses a given list of text characters to get a smaller size
    fn encode(&mut self) {
        let text = self.input.chars().collect::<Vec<char>>();
        let n = text.len();
        self.encoded = String::with_capacity(n);
        let mut idx = 0_usize;
        while idx < n {
            let mut ch_count = 1;
            while idx < n - 1 && text[idx] == text[idx + 1] {
                ch_count += 1;
                idx += 1;
            }
            self.encoded.push(text[idx]);
            self.encoded.extend(format!("{ch_count}").chars());
            idx += 1;
        }
    }

    /// decode decompresses a given compressed text to get the original text
    fn decode(&mut self) {
        let code = self.encoded.chars().collect::<Vec<char>>();
        let n = code.len();
        let mut idx = 0_usize;
        let mut digit_str = String::new();
        let mut last_letter_idx = 0_usize;
        while idx < n {
            while idx < n && code[idx].is_ascii_digit() {
                digit_str.push(code[idx]);
                idx += 1;
            }
            if !digit_str.is_empty() {
                let count = digit_str.parse::<usize>().unwrap();
                let s = String::from(code[last_letter_idx]);
                self.output.push_str(&s.repeat(count));
                digit_str.clear();
            } else {
                last_letter_idx = idx;
                idx += 1;
            }
        }
    }

    fn compressed(&self) -> String {
        self.encoded.clone()
    }

    fn decompressed(&self) -> String {
        self.output.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::pkg::traits::Codec;

    #[test]
    fn encoder_works() {
        let test_cases = vec![
            ("abracadabra!", "a1b1r1a1c1a1d1a1b1r1a1!1"),
            ("aabbc", "a2b2c1"),
            ("aaaaaaaaaa", "a10"),
        ];

        for test_case in test_cases {
            let mut rle = super::Rle::new(test_case.0.to_string());
            rle.encode();
            assert_eq!(rle.compressed(), test_case.1);
        }
    }

    #[test]
    fn decoder_works() {
        let test_cases = vec![
            ("abracadabra!", "a1b1r1a1c1a1d1a1b1r1a1!1", "abracadabra!"),
            ("aabbc", "a2b2c1", "aabbc"),
            ("aaaaaaaaaa", "a10", "aaaaaaaaaa"),
        ];

        for test_case in test_cases {
            let mut rle = super::Rle::new(test_case.0.to_string());
            rle.encode();
            assert_eq!(rle.compressed(), test_case.1);
            rle.decode();
            assert_eq!(rle.decompressed(), test_case.2);
        }
    }
}
