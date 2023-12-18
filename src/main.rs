use std::env;
use std::path;
use std::process;

use dbc::Config;

const DEAFULT_CONF_PATH: &str = ".config/dbc/config.toml";

fn main() {
    let home_path = env::var("HOME").unwrap_or_else(|err| {
        eprintln!("Failed to get value of $HOME variable: {err}");
        process::exit(1);
    });

    let conf_path = path::Path::new(&home_path).join(&DEAFULT_CONF_PATH);
    let conf_path = conf_path.to_str().unwrap_or_else(|| {
        eprintln!("Failed to build path to config");
        process::exit(1);
    });

    let conf = Config::build(conf_path).unwrap_or_else(|err| {
        eprintln!("Failed to load config: {err}");
        process::exit(1);
    });

    println!("Config: {:#?}", conf);
}
