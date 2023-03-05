extern crate tmrustplayground;

use std::{env, process};
use std::cell::Cell;
use std::ops::Range;
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

fn execute<T>(tasks: Vec<fn() -> T>, is_multiple: bool) {
    let (sender, receiver) = mpsc::channel();

    if is_multiple {
        for value in distribute_tasks(&tasks) {
            if let Err(error) = sender.send(value) {
                eprintln!("Unable to send task's result. Error description: {}", error);
                break
            }
        }
    } else {

    }
}

fn distribute_tasks<T>(tasks: &Vec<fn() -> T>) -> Vec<T> {
    if tasks.len() == 1 {
        if let Some(task) = tasks.first() {
            return vec![task()];
        }
        return vec![]
    }

    let half_index = tasks.len() / 2;
    let mut result = vec![];

    for i in (0..=tasks.len()).step_by(half_index) {
        result.append(&mut distribute_tasks(&Vec::from(&tasks[i..=(i+half_index)])));
    }

    result
}

fn main() {
    let mut tasks = vec![];

    for i in 1..=10 {
        tasks.push(|| {
            thread::sleep(Duration::from_secs_f32(i as f32 * 0.1));
            i
        });
    }

    distribute_tasks(&tasks);
}