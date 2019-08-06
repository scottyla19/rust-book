use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

   

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.filename)?;

//     println!("With text:\n{}", contents);

//     Ok(())
// }

// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }