extern crate tmrustplayground;

use std::{env, process};
use std::fmt::Display;
use std::ops::Deref;
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

struct TMBox<T>(T) where T: Display;

impl<T> TMBox<T>
    where T: Display {
    fn new(x: T) -> TMBox<T> {
        TMBox(x)
    }
}

impl<T> Deref for TMBox<T>
    where T: Display {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for TMBox<T>
    where T: Display {
    fn drop(&mut self) {
        println!("Now drop the value. {}", self.0);
    }
}

fn main() {
    let message = TMBox::new(String::from("Rust"));

    println!("This is {}", message.0);
    drop(message);
    println!("Something occurred!");
}