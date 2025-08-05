use std::time::{Instant};
use crate::interpreter::{interpret};
use crate::standard_lib::standard_lib_map;

mod interpreter;
mod standard_lib;

const ITER_COUNT: usize = 100;

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
        "op add accum 0 0",
        "op add target 100000 0",
        "op add accum accum 1",
        "print accum",
        "jump 3 notEqual accum target",
        "stop"
    ];
    
    let target_code = &_bench_code;
    
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
