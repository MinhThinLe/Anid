mod config_parser;

use std::process::exit;

use config_parser::*;

fn main() {
    let config = parse_config();
    let config = match config {
        Ok(config) => config,
        Err(ParseConfigError::FileNotFound) => {
            make_config();
            Config::default()
        }
        Err(ParseConfigError::InvalidTOML) => {
            println!("Invalid TOML");
            exit(1)
        }
    };
    println!("Parsing returned {:#?}", config);
}
