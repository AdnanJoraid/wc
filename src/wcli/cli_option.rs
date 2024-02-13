use std::str::FromStr;

#[derive(Debug)]
pub enum CliOption {
    C,
    L,
    W,
    M,
    Empty,
}

impl FromStr for CliOption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-c" => Ok(CliOption::C),
            "-l" => Ok(CliOption::L),
            "-w" => Ok(CliOption::W),
            "-m" => Ok(CliOption::M),
            "- " => Ok(CliOption::Empty),
            _ => Err(anyhow::anyhow!("Command does not exist")),
        }
    }
}
