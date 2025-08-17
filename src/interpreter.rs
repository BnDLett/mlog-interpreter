use crate::lexer::{GlobalState, Token};

#[inline(always)]
pub fn interpret(tokens: Vec<Vec<Token<'static>>>, global_state: &GlobalState) {
    let mut _accumulator = 0usize;
    let mut position = 0usize;
    
    let mut line;
    let mut parameters = Vec::with_capacity(20);
    let mut instruction;
    let mut parameters_array;
    
    loop {
        line = tokens.get(position);
        
        // .get() returns None if out of bounds.
        if line.is_none() { break; }
        
        instruction = line.unwrap().get(0).unwrap();
        parameters_array = line.unwrap().get(1..).unwrap();
        parameters.clear();
        
        for parameter in parameters_array {
            match parameter {
                Token::Parameter(value) => { parameters.push(value); },
                _ => {
                    println!("A critical error occurred when parsing parameters in the executor.");
                    return;
                }
            }
        }
        
        match instruction {
            Token::Keyword(keyword) => {
                if parameters.len() > (keyword.parameter_count) {
                    println!("Invalid parameter count on instruction {}.", position);
                    println!("Expected {}, found {}", keyword.parameter_count, parameters.len());
                    return;
                } else {
                    (keyword.callback)(&parameters);
                }
            }
            _ => { return; }
        }
        
        _accumulator += 1;
        position += 1;
    }
}
