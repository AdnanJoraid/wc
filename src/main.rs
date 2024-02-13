/*
* The goal of this project
* ccwc -c test.txt -> # bytes
* ccwc -l text.txt -> # ines
* ccwc -w text.txt -> # words
* ccwc -m test.txt -> # characters
*/
use std::{
    fs::{metadata, File},
    io::Read,
    str::FromStr,
};
#[derive(Debug)]
enum CliOption {
    C,
    L,
    W,
    M,
}

impl FromStr for CliOption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-c" => Ok(CliOption::C),
            "-l" => Ok(CliOption::L),
            "-w" => Ok(CliOption::W),
            "-m" => Ok(CliOption::M),
            _ => Err(anyhow::anyhow!("Command does not exist")),
        }
    }
}
#[derive(Debug)]
struct Cli {
    pattern: CliOption,
    path: std::path::PathBuf,
}

impl Cli {
    pub fn execute_command(&self) -> u64 {
        println!("Here is the pattern: {:?}", self.pattern);
        match self.pattern {
            CliOption::C => Self::execute_bytes_count(&self).try_into().unwrap(),
            CliOption::L => Self::execute_lines_count(),
            CliOption::W => Self::execute_words_count(),
            CliOption::M => Self::execute_chars_count(),
            _ => 0,
        }
    }

    fn execute_bytes_count(&self) -> u64 {
        let file = File::open(&self.path).unwrap();
        let metadata = file.metadata().unwrap();
        return metadata.len();
    }
    //testing return below
    fn execute_lines_count() -> u64 {
        3
    }
    fn execute_words_count() -> u64 {
        4
    }
    fn execute_chars_count() -> u64 {
        5
    }
}

fn main() {
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let x = pattern.parse::<CliOption>().ok();
    if let Some(pattern) = x {
        let cli = Cli {
            pattern,
            path: path.into(),
        };
        println!("{:?}", cli.execute_command());
    }
}
