use std::time::Instant;
use crate::interpreter::interpret;
use crate::lexer::tokenize_vec;
use crate::standard_lib::create_global_state;

mod interpreter;
mod standard_lib;
mod lexer;

const ITER_COUNT: usize = 10_000_000;

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
    let _set_test = vec![
        "set xyz 5",
        "set lorem xyz",
        "set xyz 10"
    ];
    let _print_test = vec![
        String::from("print 'hello, world!'"),
        String::from("print \"Hello, world!\""),
    ];
    
    let start = Instant::now();
    
    let mut bench_code: Vec<String> = Vec::new();
    for _ in 0..ITER_COUNT {
        bench_code.push(String::from("op 2 3 4"));
    }
    
    let start_parsing = Instant::now();
    
    let global_state = create_global_state();
    let tokens = tokenize_vec(bench_code, global_state).unwrap();
    
    let start_execution = Instant::now();
    
    interpret(tokens, global_state);
    
    let total_duration = start.elapsed();
    let program_duration = start_parsing.elapsed();
    let execution_duration = start_execution.elapsed();
    
    let time_spent_total = total_duration.as_millis();
    let time_spent_running = program_duration.as_millis();
    let time_spent_executing = execution_duration.as_millis();
    
    println!("{:?} ms total", time_spent_total);
    println!("{:?} ms parse and execution", time_spent_running);
    println!("{:?} ms execution", time_spent_executing);
    
    println!("{} instructions/second", ITER_COUNT as f64 / (time_spent_executing as f64 / 1000f64));
    println!("{} instructions", ITER_COUNT)
    
    
    // let fibonacci = fs::read_to_string("examples/fibonacci.mlog");
    // 
    // if fibonacci.is_err() {
    //     return
    // }
    // let fib_raw_code = fibonacci.unwrap();
    // let _fib_code = fib_raw_code.split("\n").collect();
    // 
    // let target_code = &_fib_code;
    // 
    // // println!("{:?}", target_code);
    // 
    // let std = standard_lib_map();
    // let mut total_instructions = 0;
    // let start_execution = Instant::now();
    // 
    // for _ in 0..ITER_COUNT {
    //     total_instructions = interpret(&std, target_code).expect("me when i");
    // }
    // 
    // let duration = start_execution.elapsed();
    // let time_spent = duration.as_millis();
    // 
    // println!("{:?} ms", time_spent);
    // println!("{} instructions/second", (total_instructions as usize * ITER_COUNT) as f64 / (time_spent as f64 / 1000f64));
    // println!("{} instructions/iteration", total_instructions)
}
