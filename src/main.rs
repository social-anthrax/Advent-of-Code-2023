#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod task_handler;
mod tasks;

#[allow(clippy::wildcard_imports)]
use tasks::*;

type Callback = Box<dyn Fn()>;

fn main() {
    let functions: Vec<Callback> = vec![
        Box::new(task1::tasks),
        Box::new(task2::tasks),
        Box::new(task3::tasks),
    ];

    for (day, func) in functions.iter().enumerate() {
        println!("\x1b[93mDay {} \x1b[0m", day + 1);
        func();
    }
}
