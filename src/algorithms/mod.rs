use std::fmt::{Debug, Formatter};

pub mod huffman;
pub mod lempel_ziv_welch;
pub mod run_length_encoding;

#[derive(Clone)]
pub enum Algorithm {
    Rle,
    Huffman,
    Bwt,
    Lzw,
    Invalid,
}

impl Debug for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::Huffman
    }
}

impl From<String> for Algorithm {
    fn from(value: String) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "rle" => Algorithm::Rle,
            "huffman" => Algorithm::Huffman,
            "bwt" => Algorithm::Bwt,
            "lzw" => Algorithm::Lzw,
            _ => Algorithm::Invalid,
        }
    }
}

impl ToString for Algorithm {
    fn to_string(&self) -> String {
        match self {
            Algorithm::Rle => "RLE".to_string(),
            Algorithm::Huffman => "Huffman".to_string(),
            Algorithm::Bwt => "BWT".to_string(),
            Algorithm::Lzw => "LZW".to_string(),
            Algorithm::Invalid => "invalid".to_string(),
        }
    }
}

impl PartialEq for Algorithm {
    fn eq(&self, other: &Self) -> bool {
        return self.to_string() == other.to_string();
    }
}

#[cfg(test)]
mod test {
    use crate::algorithms::Algorithm;

    #[test]
    fn algorithm_default_works() {
        let default_algo = Algorithm::default();
        assert_eq!(default_algo, Algorithm::Huffman);
    }

    #[test]
    fn algorithm_to_string_works() {
        let test_cases = vec![
            (Algorithm::Rle, "RLE"),
            (Algorithm::Huffman, "Huffman"),
            (Algorithm::Bwt, "BWT"),
            (Algorithm::Lzw, "LZW"),
            (Algorithm::Invalid, "invalid"),
        ];

        for test_case in test_cases {
            assert_eq!(test_case.0.to_string(), test_case.1.to_string());
        }
    }

    #[test]
    fn algorithm_from_string_works() {
        let test_cases = vec![
            ("RLE", Algorithm::Rle),
            ("RlE", Algorithm::Rle),
            ("rle", Algorithm::Rle),
            ("HuffMan", Algorithm::Huffman),
            ("huFfMan", Algorithm::Huffman),
            ("HuffMAN", Algorithm::Huffman),
            ("BWt", Algorithm::Bwt),
            ("bWt", Algorithm::Bwt),
            ("bwt", Algorithm::Bwt),
            ("lzW", Algorithm::Lzw),
            ("lzw", Algorithm::Lzw),
            ("LZw", Algorithm::Lzw),
        ];

        for test_case in test_cases {
            assert_eq!(Algorithm::from(test_case.0.to_string()), test_case.1);
        }
    }
}
