mod counter;

use clap::Parser;
use counter::count_line;

#[derive(Parser)]
#[clap(
    author = "Damin",
    version = "0.1.0",
    name = "cl",
    about = "Recursive Line Counter",
    long_about = None
)]
pub struct Cli {
    /// Directory where you count lines
    directory: String,

    /// File's extension which you count
    extension: Option<String>,

    /// Output logs verbosely
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();
    match count_line(&cli) {
        Ok(line_count) => println!("{}", line_count),
        Err(err) => eprintln!("{}", err),
    };
}
