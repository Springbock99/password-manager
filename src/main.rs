use clap::{Arg, Command};
use rand::Rng;
use rand::distributions::Alphanumeric;

fn main() {
let matches = Command::new("password-manager")
    .version("0.1.0")
    .about("A secure password manager using Rust")
    .subcommand(
        Command::new("generate")
        .about("Generates a new password")
        .arg(
            Arg::new("length")
            .short('1')
            .long("length")
            .value_name("LENGTH")
            .help("Sets the length of the password")
            .default_value("16")
            .value_parser(clap::value_parser!(usize))
        ),
    )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        let lenght: usize = *matches.get_one::<usize>("lenghth").unwrap();
        let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(lenght)
        .map(char::from)
        .collect();
    println!("Generate Password: {}", password);
    }
    
}
