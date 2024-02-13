use crate::wcli::cli_option::CliOption;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Cli {
    pub pattern: CliOption,
    pub path: std::path::PathBuf,
}

impl Cli {
    pub fn execute_command(&self) {
        match self.pattern {
            CliOption::C => Self::execute_bytes_count(&self),
            CliOption::L => Self::execute_lines_count(&self),
            CliOption::W => Self::execute_words_count(&self),
            CliOption::M => Self::execute_chars_count(&self),
            CliOption::Empty => {
                Self::execute_bytes_count(&self);
                Self::execute_lines_count(&self);
                Self::execute_words_count(&self);
            }
        }
    }

    fn execute_bytes_count(&self) {
        let file = File::open(&self.path).unwrap();
        let metadata = file.metadata().unwrap();
        println!("{} {}", metadata.len(), self.path.to_string_lossy());
    }

    fn execute_lines_count(&self) {
        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);
        let lines_count = reader.lines().count();
        println!("{} {}", lines_count, self.path.to_string_lossy());
    }
    fn execute_words_count(&self) {
        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);
        let mut count = 0;
        for line in reader.lines() {
            count += line.unwrap().split_whitespace().count();
        }
        println!("{} {}", count, self.path.to_string_lossy());
    }
    fn execute_chars_count(&self) {
        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);
        let mut char_count = 0;
        for line in reader.lines() {
            char_count += line.unwrap().split("").count();
        }
        println!("{} {}", char_count, self.path.to_string_lossy());
    }
}
