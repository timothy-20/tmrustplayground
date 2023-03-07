extern crate tmrustplayground;
extern crate core;

use core::slice;
use std::{env, process};
use std::cell::Cell;
use std::fmt::{Debug, Display, Error, Formatter, write};
use std::iter::Enumerate;
use std::ops::{Add, Range};
use std::os::unix::process::parent_id;
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
//     let tasks_ref = &mut tasks[..];
//     let (first, last) = tasks_ref.split_at_mut(tasks_ref.len() / 2);
//
//     distribute_tasks(Vec::from(first));
//     distribute_tasks(Vec::from(last));
// }

// fn split_at_mut<T>(slice: &mut [T], mid: usize) -> Vec<&mut [T]> {
//     let length = slice.len();
//     let p_slice = slice.as_mut_ptr();
//
//     assert!(mid <= length);
//
//     unsafe {
//         vec![
//             slice::from_raw_parts_mut(p_slice, mid),
//             slice::from_raw_parts_mut(p_slice.offset(mid as isize), length - mid)
//         ]
//     }
// }

struct Context<'c>(&'c str);
struct Parser<'p, 'c: 'p> {
    context: &'p Context<'c>,
}

impl<'p, 'c> Parser<'p, 'c> {
    fn parser(&self) -> Result<(), &'c str> {
        Err(&self.context.0[1..])
    }
}

/// # Example
/// ``` rust
/// if let Err(error) = parse_context(Context("Hello, timothy")) {
///     eprintln!("Error description: {}", error);
/// }
/// ```
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parser()
}

/// # Example
/// ``` rust
/// let num = 5;
/// let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
///
/// (*obj).highlight();
/// ```
trait Red {
    fn highlight(&self);
}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {
    fn highlight(&self) {
        println!("Highlighted!");
    }
}

trait OutlinePrint: Display {
    /// # Example
    /// ``` rust
    /// let point = Point { x: 10, y: 20 };
    ///
    /// point.outline_print();
    /// ```
    fn outline_print(&self) {
        let output = self.to_string();
        let length = output.len();

        println!("{}", "*".repeat(length + 4));
        println!("*{}*", " ".repeat(length + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(length + 2));
        println!("{}", "*".repeat(length + 4));
    }
}

#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl OutlinePrint for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn return_closure(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x * 10)
}

fn main() {

}