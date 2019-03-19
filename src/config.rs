use crate::cli;
use serde_derive::Deserialize;
use std::fs;
use std::io::{BufReader, Read};
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub command: Option<String>,
    pub input_dir: Option<String>,
    pub output_dir: Option<String>,
    pub logfile: Option<String>,
    pub threads: Option<usize>,
}

pub fn get_config() -> Config {
    let conf_args = get_config_from_args();
    let conf_toml = get_config_from_toml();
    fn merge<T>(a: Option<T>, b: Option<T>) -> Option<T> {
        if !a.is_none() {
            a
        } else {
            b
        }
    }
    Config {
        command: merge(conf_args.command, conf_toml.command),
        input_dir: merge(conf_args.input_dir, conf_toml.input_dir),
        output_dir: merge(conf_args.output_dir, conf_toml.output_dir),
        logfile: merge(conf_args.logfile, conf_toml.logfile),
        threads: merge(conf_args.threads, conf_toml.threads),
    }
}

fn get_config_from_toml() -> Config {
    if let Ok(file) = fs::File::open("patra.toml") {
        let mut content = String::new();
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut content).unwrap();
        toml::from_str(content.as_str()).unwrap()
    } else {
        Config {
            command: None,
            input_dir: None,
            output_dir: None,
            logfile: None,
            threads: None,
        }
    }
}

fn get_config_from_args() -> Config {
    let matches = cli::build_cli().get_matches();
    Config {
        command: matches.value_of("command").map(|s| s.to_owned()),
        input_dir: matches.value_of("input-dir").map(|s| s.to_owned()),
        output_dir: matches.value_of("output-dir").map(|s| s.to_owned()),
        logfile: matches.value_of("logfile").map(|s| s.to_owned()),
        threads: matches.value_of("threads").map(|s| s.parse().unwrap()),
    }
}
