use crate::Cli;
use rand::prelude::SliceRandom;

pub fn generate_password(cli: &Cli) -> String {
    let alphabet: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let number: Vec<char> = ('0'..='9').collect();
    let symbol: Vec<char> = r#"~`!@#$%^&*()_-+={[}]|\:;"'<,>.?/"#.chars().collect();

    let mut source_letters: Vec<char> = Vec::new();
    source_letters.extend(alphabet);
    source_letters.extend(number);
    if !cli.no_symbols {
        source_letters.extend(symbol);
    }

    let mut rng = rand::thread_rng();
    let mut password: Vec<char> = Vec::new();
    for _ in 0..cli.length {
        let sample = *source_letters.choose(&mut rng).unwrap();
        password.push(sample);
    }

    password.iter().collect::<String>()
}
