mod generator;

use generator::generate_password;
use seahorse::{App, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("Password Generator")
        .description("Secure customizable password generator")
        .author("Damin")
        .version("0.1.0")
        .usage("pwdgen <length>")
        .action(generate_password)
        .flag(
            Flag::new("exclude-symbols", FlagType::Bool)
                .description("exclude symbols from password")
                .alias("s"),
        );

    app.run(args);
}
