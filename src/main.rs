use std::time::SystemTime;
use crate::interpreter::{interpret};
use crate::standard_lib::standard_lib_map;

mod interpreter;
mod standard_lib;

const ITER_COUNT: usize = 100_000;

fn main() {
    let example_code = vec![
        "op add lorem 5 4",
        "jump 4",
        "stop",
        "op sub ipsum 10 lorem",
        "print lorem",
        "print ipsum",
        "printflush message1",
        "stop"
    ];
    let std = standard_lib_map();
    
    let start = SystemTime::now();
    for i in 0..ITER_COUNT {
        interpret(&std, &example_code).expect("me when i");
    }
    
    let duration = start.elapsed().unwrap();
    let time_spent = duration.as_millis();
    
    println!("{:?} ms", time_spent);
    println!("{} instructions/second", (example_code.len() * ITER_COUNT) as f64 / (time_spent as f64 / 1000f64));
}
