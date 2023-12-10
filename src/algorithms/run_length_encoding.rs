use crate::pkg::traits::Codec;
use std::fmt::Display;

#[derive(Debug, Clone)]
struct RunLengthEncodingPart(u8, u64);

impl Display for RunLengthEncodingPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (self.0 as char).to_string().repeat(self.1 as usize)
        )
    }
}

/// RLE is the zero product struct for the RUN LENGTH ENCODING algorithm
#[derive(Clone)]
pub struct RunLengthEncoding {
    text: String,
    encoded: Vec<RunLengthEncodingPart>,
    decoded: String,
}

impl RunLengthEncoding {
    pub fn new(text: String) -> Self {
        Self {
            text,
            encoded: Vec::new(),
            decoded: String::new(),
        }
    }
}

/// Codec trait implementation for the RLE algorithm
impl Codec for RunLengthEncoding {
    /// encode compresses a given list of text characters to get a smaller size
    fn encode(&mut self) {
        let n = self.text.len();
        let text_chars = self.text.chars().collect::<Vec<char>>();

        let mut i = 0usize;
        while i < n {
            let mut char_count = 1u64;
            while i < n - 1 && text_chars[i] == text_chars[i + 1] {
                char_count += 1;
                i += 1;
            }
            self.encoded
                .push(RunLengthEncodingPart(text_chars[i] as u8, char_count));
            i += 1;
        }
    }

    /// decode decompresses a given compressed text to get the original text
    fn decode(&mut self) {
        for part in self.encoded.iter() {
            self.decoded.push_str(part.to_string().as_str());
        }
    }

    fn compressed(&self) -> String {
        let mut s = String::new();
        for part in self.encoded.iter() {
            s.push_str(format!("{part:?}").as_str());
        }
        s
    }

    fn decompressed(&self) -> String {
        self.decoded.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::pkg::traits::Codec;

    #[test]
    fn encoder_works() {
        let test_cases = vec![
            ("abracadabra!", "RlePart(97, 1)RlePart(98, 1)RlePart(114, 1)RlePart(97, 1)RlePart(99, 1)RlePart(97, 1)RlePart(100, 1)RlePart(97, 1)RlePart(98, 1)RlePart(114, 1)RlePart(97, 1)RlePart(33, 1)"),
            ("aabbc", "RlePart(97, 2)RlePart(98, 2)RlePart(99, 1)"),
            ("aaaaaaaaaa", "RlePart(97, 10)"),
        ];

        for test_case in test_cases {
            let mut rle = super::RunLengthEncoding::new(test_case.0.to_string());
            rle.encode();
            assert_eq!(rle.compressed(), test_case.1);
        }
    }

    #[test]
    fn decoder_works() {
        let test_cases = vec!["abracadabra!", "aabbc", "aaaaaaaaaa"];

        for test_case in test_cases {
            let mut rle = super::RunLengthEncoding::new(test_case.to_string());
            rle.encode();
            rle.decode();
            assert_eq!(rle.decompressed(), test_case);
        }
    }
}
