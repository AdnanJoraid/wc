use std::{
    fs::File,
    io::{self, BufRead, BufReader},
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
        match self.pattern {
            CliOption::C => Self::execute_bytes_count(&self).unwrap_or_else(|e| {
                eprint!("Error while executing command -c: {}", e);
                0
            }),
            CliOption::L => Self::execute_lines_count(&self).unwrap_or_else(|e| {
                eprint!("Error while executing command -l: {}", e);
                0
            }),
            CliOption::W => Self::execute_words_count(&self).unwrap_or_else(|e| {
                eprint!("Error while executing command -w: {}", e);
                0
            }),
            CliOption::M => Self::execute_chars_count(&self).unwrap_or_else(|e| {
                eprint!("Error while executing command -m: {}", e);
                0
            }),
        }
    }

    fn execute_bytes_count(&self) -> Result<u64, io::Error> {
        let file = File::open(&self.path)?;
        let metadata = file.metadata()?;
        return Ok(metadata.len());
    }

    fn execute_lines_count(&self) -> Result<u64, io::Error> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let lines_count = reader.lines().count();
        return Ok(lines_count as u64);
    }
    fn execute_words_count(&self) -> Result<u64, io::Error> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut count = 0;
        for line in reader.lines() {
            count += line?.split_whitespace().count();
        }
        return Ok(count as u64);
    }
    fn execute_chars_count(&self) -> Result<u64, io::Error> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut char_count = 0;
        for line in reader.lines() {
            char_count += line?.split("").count();
        }
        return Ok(char_count as u64);
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
