use std::fs;
use std::time::{Instant};
use crate::interpreter::{interpret};
use crate::standard_lib::standard_lib_map;

mod interpreter;
mod standard_lib;

const ITER_COUNT: usize = 1;

fn main() {
    let _example_code = vec![
        "op add lorem 5 4",
        "jump 4 equal 1 1",
        "stop",
        "op sub ipsum 10 lorem",
        "print lorem",
        "print ipsum",
        // "printflush message1",
        "stop"
    ];
    let _bench_code = vec![
        "set accum 0 ",
        "set target 5000000",
        "op add accum accum 1",
        // "print accum",
        "jump 3 notEqual accum target",
        "stop"
    ];
    let fibonacci = fs::read_to_string("examples/fibonacci.mlog");
    let _fib_code;
    
    if fibonacci.is_err() {
        return
    }
    let fib_raw_code = fibonacci.unwrap();
    _fib_code = fib_raw_code.split("\n").collect();
    
    let target_code = &_fib_code;
    
    // println!("{:?}", target_code);
    
    let std = standard_lib_map();
    let total_instructions = interpret(&std, target_code).expect("me when i");
    let start = Instant::now();
    
    for _ in 0..ITER_COUNT {
        interpret(&std, target_code).expect("me when i");
    }
    
    let duration = start.elapsed();
    let time_spent = duration.as_millis();
    
    println!("{:?} ms", time_spent);
    println!("{} instructions/second", (total_instructions as usize * ITER_COUNT) as f64 / (time_spent as f64 / 1000f64));
    println!("{} instructions/iteration", total_instructions)
}
