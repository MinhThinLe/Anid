use std::process::{Command, exit};
use crate::{Config, AnimeEntry};

const SUCCESS: i32 = 0;
const COMMAND_NOT_FOUND: i32 = 127;
const FAILURE: i32 = 1;

pub fn download(config: &mut Config) {
    for entry in config.watch_list.iter_mut() {
        download_entry(entry);
    }
}

fn download_entry(entry: &mut AnimeEntry) {
    let args = entry.get_download_arguments();
    let downloader = Command::new("ani-cli")
        .current_dir(entry.get_target_directory())
        .args(args)
        .status();
    let code = match downloader {
        Ok(status_code) => status_code.code(),
        Err(error) => {
            println!("Error {error} unhandled, exiting now");
            exit(1);
        }
    };
    let Some(code) = code else {
        return;
    };
    
    match code {
        SUCCESS => entry.next_episode(),
        COMMAND_NOT_FOUND => println!("ani-cli executable not found, maybe try installing it?"),
        FAILURE => println!("Failed to download {}, check if it's a unique name?", entry.get_name()),
        code => println!("Unknown return code {code}"),
    };
}
