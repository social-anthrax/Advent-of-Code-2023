#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod task_handler;
mod tasks;

use std::collections::HashMap;

#[allow(clippy::wildcard_imports)]
use tasks::*;

type Callback = Box<dyn Fn()>;

fn main() {
    let x: Vec<(u8, Callback)> = vec![(1, Box::new(task1::tasks)), (2, Box::new(task2::tasks))];

    let functions = x.into_iter().collect::<HashMap<u8, Callback>>();
    for (day, func) in &functions {
        println!("\x1b[93mDay {day} \x1b[0m");
        func();
    }
}
