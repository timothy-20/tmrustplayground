extern crate tmrustplayground;

use std::{env, process};
use std::cell::Cell;
use tmrustplayground::{cache, minigrep};
use tmrustplayground::minigrep::Config;

use std::thread;
use std::time:: Duration;
use std::sync::mpsc;

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
    let (sender, receiver) = mpsc::channel();

    for _ in 0..= 1 {
        thread::spawn(move || {
            let messages = vec![
                String::from("Send message [1]"),
                String::from("Send message [2]"),
                String::from("Send message [3]"),
                String::from("Send message [4]"),
                String::from("Send message [5]"),
            ];

            for message in messages {
                if let Err(error) = sender.send(message) {
                    println!("Unable send to channel. Error description: {}", error);
                    break
                }

                thread::sleep(Duration::from_secs_f32(0.5));
            }
        });
    }

    for receive_value in receiver {
        println!("Result: {}", receive_value);
    }
}