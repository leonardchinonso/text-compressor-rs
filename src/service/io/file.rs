use crate::service::pkg::traits::{Reader, Writer};
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::io::Write;

/// File is a struct representing file operations on text data
pub struct File {
    in_path: String,
    out_path: String,
    content: String,
}

impl File {
    pub fn new(in_path: &str, out_path: &str) -> Self {
        Self {
            in_path: in_path.to_string(),
            out_path: out_path.to_string(),
            content: String::new(),
        }
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl PartialEq<Self> for File {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Eq for File {}

impl Reader for File {
    fn read(&mut self) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(self.in_path.clone())?;
        self.content = content.clone();
        Ok(content)
    }
}

impl Writer for File {
    fn write(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut file = fs::File::create(self.out_path.clone())?;
        file.write_all(data)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::service::pkg::traits::{Reader, Writer};
    use log::info;

    #[test]
    fn read_works() {
        let mut file = super::File::new("test_data.txt", "out_data.txt");
        match file.read() {
            Ok(data) => {
                info!("File read successfully");
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn write_works() {
        let mut file = super::File::new("test_data.txt", "out_data.txt");
        let data = file.read().expect("could not read file!");
        file.write(data.as_bytes())
            .expect("cannot write data to file");
    }
}
