use rand::prelude::SliceRandom;
use seahorse::Context;

pub fn generate_password(ctx: &Context) {
    let alphabet: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let number: Vec<char> = ('0'..='9').collect();
    let symbol: Vec<char> = r#"~`!@#$%^&*()_-+={[}]|\:;"'<,>.?/"#.chars().collect();

    if ctx.args.len() != 1 {
        return println!(
            "Error: Arg size is invalid. Expected {}, got {}",
            1,
            ctx.args.len()
        );
    }

    let length_like = ctx.args[0].parse::<u8>();
    if let Err(_) = length_like {
        return println!(
            "Error: Password legnth must be number. Given: {}",
            ctx.args[0]
        );
    }
    let length = length_like.unwrap();

    let mut source_letters: Vec<char> = Vec::new();
    source_letters.extend(alphabet);
    source_letters.extend(number);
    if !ctx.bool_flag("exclude-symbols") {
        source_letters.extend(symbol);
    }

    let mut rng = rand::thread_rng();
    let mut password: Vec<char> = Vec::new();
    for _ in 0..length {
        let sample = *source_letters.choose(&mut rng).unwrap();
        password.push(sample);
    }

    println!("{}", password.iter().collect::<String>());
}
