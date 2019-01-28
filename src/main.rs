#[macro_use] extern crate text_io;
extern crate clap;

use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::fs::File;
use std::fs::OpenOptions;
use std::collections::HashMap;
use clap::{Arg, App};

fn main() -> std::io::Result<()> {
    let args = App::new("envpopulate")
        .version("0.1.0")
        .author("Nikita Nefedov <inefedor@gmail.com>")
        .about("Populate your .env file from distribution .env.dist interactively and incrementally")
        .arg(Arg::with_name("quiet")
            .short("q").long("quiet")
            .help("Don't ask for user input on each env-variable, just take the values from distribution file"))
        .arg(Arg::with_name("env")
            .long("env")
            .help("Custom filename of your .env file")
            .takes_value(true)
            .default_value(".env"))
        .arg(Arg::with_name("env-dist")
            .long("env-dist")
            .help("Custom filename of your .env.dist (distribution) file, this file will be taken as a source of defaults")
            .takes_value(true)
            .default_value(".env.dist"))
        .get_matches();

    let env_file_name = args.value_of("env").unwrap_or(".env");
    let env_dist_file_name = args.value_of("env-dist").unwrap_or(".env.dist");
    let quiet = args.is_present("quiet");

    let env_file = File::open(env_file_name);
    let env_dist_file = match File::open(env_dist_file_name) {
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
        .open(env_file_name)?;

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
