mod config_parser;
mod download;

use std::process::exit;

use config_parser::*;
use download::download;

fn main() {
    let config = parse_config();
    let mut config = match config {
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

    download(&mut config);
}
