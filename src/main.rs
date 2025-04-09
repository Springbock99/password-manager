use clap::{Arg, Command};
use rand::Rng;
use rand::distributions::Uniform;

fn main() {
    let matches = Command::new("password-manager")
        .version("0.1.0")
        .about("A secure password manager using Rust")
        .subcommand(
            Command::new("generate")
                .about("Generates a new password")
                .arg(
                    Arg::new("length")
                        .short('l')
                        .long("length")
                        .value_name("LENGTH")
                        .help("Sets the length of the password")
                        .default_value("16")
                        .value_parser(clap::value_parser!(usize))
                )
                .arg(
                    Arg::new("site")
                        .short('s')
                        .long("site")
                        .value_name("SITE")
                        .help("The site this password is for (e.g., google)")
                        .required(false)
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        let length: usize = *matches.get_one::<usize>("length").unwrap();
        let site: Option<String> = matches.get_one::<String>("site").cloned(); 
        let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
        let char_distribution = Uniform::new(0, chars.len());
        let password: String = rand::thread_rng()
            .sample_iter(&char_distribution)
            .map(|i| chars.chars().nth(i).unwrap())
            .take(length)
            .collect();
        match site {
            Some(s) => println!("Generated Password for {}: {}", s, password),
            None => println!("Generated Password: {}", password),
        }
    }
}