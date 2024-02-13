mod wcli;
use crate::wcli::{cli::Cli, cli_option::CliOption};
fn main() {
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let cli_command = pattern.parse::<CliOption>().ok();
    if let Some(pattern) = cli_command {
        let cli = Cli {
            pattern,
            path: path.into(),
        };
        cli.execute_command();
    } else {
        let cli = Cli {
            pattern: CliOption::Empty,
            path: path.into(),
        };
        cli.execute_command();
    }
}
