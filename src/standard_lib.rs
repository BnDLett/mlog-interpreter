use std::hint::black_box;
use crate::lexer::{Callback, GlobalState, Value};

pub fn create_global_state() -> &'static mut GlobalState<'static> {
    let global_state = Box::leak(Box::new(GlobalState {
        keywords: Vec::new(),
        variables: Vec::new()
    }));
    
    fn print(parameters: &Vec<&Value>) {
        let to_print = parameters.get(0).unwrap();
        // println!("{}", to_print.string.clone().unwrap());
    }
    let print_callback = Box::leak(Box::new(Callback {
        callback: print,
        parameter_count: 1,
        name: "print"
    }));
    // add_instruction(print_callback, &mut global_state);
    global_state.keywords.push(print_callback);
    
    fn op(parameters: &Vec<&Value>) {
        let x = parameters[0];
        let y = parameters[1];
        
        if x.float.is_none() || y.float.is_none() {
            return;
        }
        
        black_box(x.float.unwrap() + y.float.unwrap());
    }
    let print_callback = Box::leak(Box::new(Callback {
        callback: op,
        parameter_count: 2,
        name: "op"
    }));
    // add_instruction(print_callback, &mut global_state);
    global_state.keywords.push(print_callback);
    
    global_state
}
