use clap::Parser;
use std::fmt::Debug;

#[derive(Parser, Debug, Default, Clone)]
#[clap(author = "Author Name", version, about)]
/// A text compressor
pub struct Argument {
    /// name of the text file to compress
    file_name: String,
    /// multi-thread support. It is false by default
    #[clap(short, long, action)]
    multithread: bool,
    /// whether to use the rest API or CLI
    #[clap(short, long, action)]
    cli: bool,
}

impl Argument {
    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn should_multithread(&self) -> bool {
        self.multithread
    }

    pub fn use_cli(&self) -> bool {
        self.cli
    }

    /// validate_file_name checks that the file name is a valid one and eats whitespaces
    pub fn validate_file_name(&mut self) -> Result<(), String> {
        let mut name = String::new();
        for word in self.file_name.split_whitespace() {
            name.push_str(word);
        }
        self.file_name = name;

        if self.file_name.is_empty() {
            return Err(String::from("file name cannot be empty"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_argument_works() {
        let file_name = "dummy.txt".to_string();
        let multithread = true;
        let cli = true;

        let args = Argument {
            file_name: file_name.clone(),
            multithread,
            cli,
        };

        assert_eq!(args.file_name, file_name);
        assert_eq!(args.multithread, multithread);
        assert_eq!(args.cli, cli);
    }
}
