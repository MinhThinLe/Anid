use std::fs::{self, File};
use std::path::PathBuf;
use std::process::exit;

use toml::{Table, Value};

const CONFIG_PATH: &str = ".config/anime-dowloader/";
const CONFIG_FILE: &str = "watchlist.toml";

#[derive(Debug, Default)]
pub struct Config {
    watch_list: Vec<AnimeEntry>,
}

#[derive(Debug)]
pub struct AnimeEntry {
    name: Box<str>,
    current_episode: u16,
}

#[derive(Debug)]
pub enum ParseConfigError {
    FileNotFound,
    InvalidTOML,
}

impl AnimeEntry {
    fn from_table(table: Value) -> Option<Self> {
        let name = table.get("name")?.as_str()?.into();
        let current_episode = table.get("current_episode")?.as_integer()? as u16;

        Some(Self {
            name,
            current_episode,
        })
    }
}

pub fn parse_config() -> Result<Config, ParseConfigError> {
    let mut config_file = get_config_path();
    config_file.push(CONFIG_FILE);
    let config_file = fs::read_to_string(config_file).or(Err(ParseConfigError::FileNotFound))?;
    let parsed = config_file
        .parse::<Table>()
        .or(Err(ParseConfigError::InvalidTOML))?;

    let mut config = Config::default();

    for item in parsed {
        if (item.0) == "config" {
            continue;
        }
        if let Some(entry) = AnimeEntry::from_table(item.1) {
            config.watch_list.push(entry);
        }
    }

    Ok(config)
}

pub fn make_config() {
    let mut config_path = get_config_path();
    let error = fs::create_dir_all(&config_path);
    match error {
        Ok(_) => (),
        Err(error) => {
            println!("Encountered error {error} while making config directory");
            exit(1);
        }
    }
    config_path.push(CONFIG_FILE);
    let error = File::create(config_path);
    match error {
        Ok(_) => (),
        Err(error) => {
            println!("Encountered error {error} while making config file");
            exit(1);
        }
    }
}

fn get_config_path() -> PathBuf {
    let mut home = match std::env::home_dir() {
        Some(dir) => dir,
        None => {
            println!("Couldn't get the user's home directory, exiting");
            exit(1);
        }
    };
    home.push(CONFIG_PATH);
    home
}
