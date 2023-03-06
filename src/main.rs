extern crate tmrustplayground;

use std::{env, process};
use std::cell::Cell;
use std::fmt::{Debug, Display};
use std::iter::Enumerate;
use std::ops::Range;
use std::ptr::addr_of_mut;
use tmrustplayground::{cache, minigrep};
use tmrustplayground::minigrep::Config;

use std::thread;
use std::time:: Duration;
use std::sync::mpsc;
use std::vec::Drain;

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

// #[derive(Copy, Clone)]
// struct ForkAndJoin<T, F>
//     where F: Fn() -> T {
//     completion: F,
//     value: T
// }
//
// impl<T, F> ForkAndJoin<T, F>
//     where F: Fn() -> T {
//
// }
//
// fn distribute_tasks<T, F>(mut tasks: Vec<F>)
//     where T: Debug + Display + Clone,
//           F: Fn() -> T + Clone {
//     if tasks.len() == 1 {
//         if let Some(task) = tasks.first() {
//             eprintln!("Result: {}", (*task)());
//         }
//         return;
//     }
//
//     let half_index = tasks.len() / 2;
//     let first_half = vec![];
//     let last_half = vec![];
//
//     tasks[..half_index].clone_from_slice(&first_half);
//     tasks[half_index..].clone_from_slice(&last_half);
//     distribute_tasks(first_half);
//     distribute_tasks(last_half);
// }

fn main() {
    let mut list = vec![1, 2, 3, 4, 5];

    for (index, value) in list.iter().enumerate() {
        eprintln!("{} is at index {}", value, index);
    }
}