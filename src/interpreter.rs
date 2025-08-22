use crate::lexer::{Callback, GlobalState, Value};

#[inline(always)]
pub fn interpret(tokens: Vec<(&Callback, Vec<Value>)>, global_state: &mut GlobalState) {
    let mut _accumulator = 0usize;
    let mut position = 0usize;
    
    let mut line;
    let mut line_result;
    let mut instruction: &Callback;
    let mut parameters;
    
    loop {
        line_result = tokens.get(position);
        
        // .get() returns None if out of bounds.
        if line_result.is_none() { break; }
        line = line_result.unwrap();
        
        instruction = line.0;
        parameters = &line.1;
        
        if parameters.len() > (instruction.parameter_count) {
            println!("Invalid parameter count on instruction {}.", position);
            println!("Expected {}, found {}", instruction.parameter_count, parameters.len());
            return;
        } else {
            (instruction.callback)(parameters, global_state);
        }
        
        _accumulator += 1;
        position += 1;
    }
}
