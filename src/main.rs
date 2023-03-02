extern crate tmrustplayground;

use std::{env, process};
use std::collections::HashMap;
use std::path::Iter;
use tmrustplayground::{cache, minigrep};
use tmrustplayground::minigrep::Config;

fn run_clt() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Error description: {}", error);
        process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Error description: {}", error);
        process::exit(1);
    }
}

fn main() {
    let string = String::new();
}