#[macro_use] extern crate text_io;

use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::fs::File;
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::env;

fn main() -> std::io::Result<()> {
    const ENV_FILE: &str = ".env";
    const ENV_DIST_FILE: &str = ".env.dist";

    if env::args().any(|arg| arg.eq(&String::from("--help")) || arg.eq(&String::from("-h"))) {
        println!("usage: envpopulate [--quiet|-q]");

        return Ok(());
    }

    let quiet = env::args().any(|arg| arg.eq(&String::from("--quiet")) || arg.eq(&String::from("-q")));

    let env_file = File::open(ENV_FILE);
    let env_dist_file = match File::open(ENV_DIST_FILE) {
        Ok(f) => f,
        Err(err) => return Err(Error::new(ErrorKind::Other, format!("Couldn't open .env.dist: {}", err.to_string())))
    };

    let mut env_dist_entries = HashMap::new();
    for line in BufReader::new(env_dist_file).lines().filter_map(|line|
        if let Ok(line) = line { Option::Some(line) } else { Option::None }) {

        if line.starts_with('#') {
            continue;
        }

        if let Some(idx) = line.find('=') {
            let key = line[..idx].to_string();
            let value = line[(idx + 1)..].to_string();

            env_dist_entries.insert(key, value);
        }
    }

    let mut env_entries: HashMap<String, String> = HashMap::new();
    if let Ok(env_file) = env_file {
        for line in BufReader::new(env_file).lines().filter_map(|line|
            if let Ok(line) = line { Option::Some(line) } else { Option::None }) {

            if let Some(idx) = line.find('=') {
                let key = line[..idx].to_string();
                let value = line[(idx + 1)..].to_string();

                env_entries.insert(key, value);
            }
        }
    }

    let mut env_file_for_write = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(ENV_FILE)?;

    for (key, value) in env_dist_entries.iter() {
        if ! env_entries.contains_key(key) {
            if quiet {
                env_file_for_write.write_fmt(format_args!("{}={}\n", key, value))?;
            } else {
                println!("{}=? ({})", key, value);
                let user_said: String = read!("{}\n");

                if user_said.trim().is_empty() {
                    env_file_for_write.write_fmt(format_args!("{}={}\n", key, value))?;
                } else {
                    env_file_for_write.write_fmt(format_args!("{}={}\n", key, user_said))?;
                }
            }
        }
    }

    Ok(())
}
