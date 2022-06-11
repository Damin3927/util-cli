mod generator;

use clap::Parser;
use generator::generate_password;

#[derive(Parser)]
#[clap(
    author = "Damin",
    version = "0.1.0",
    name = "pwdgen",
    about = "Secure customizable password generator",
    long_about = None
)]
pub struct Cli {
    /// Password length
    length: usize,

    /// Exclude symbols from password
    #[clap(short = 's', long)]
    no_symbols: bool,
}

fn main() {
    let cli = Cli::parse();
    let password = generate_password(&cli);
    println!("{}", password);
}
