use crate::lexer::{Callback, GlobalState, Value};

pub fn create_global_state() -> &'static mut GlobalState<'static> {
    let global_state = Box::leak(Box::new(GlobalState {
        keywords: Vec::new(),
        variables: Vec::new()
    }));
    
    fn print(parameters: Vec<Value<'static>>) {
        let to_print = parameters.get(0).unwrap().string.clone().unwrap().clone();
        println!("{}", to_print)
    }
    let print_callback = Box::leak(Box::new(Callback {
        callback: print,
        parameter_count: 1,
        name: String::from("print")
    }));
    // add_instruction(print_callback, &mut global_state);
    global_state.keywords.push(print_callback);
    
    global_state
}
