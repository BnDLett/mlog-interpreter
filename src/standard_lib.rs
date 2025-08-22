use std::hint::black_box;
use crate::lexer::{Callback, GlobalState, Value, Variable};

macro_rules! op_type {
    ($name:expr) => {
        Variable {
            name: String::from($name),
            value: Default::default()
        }
    };
}

pub fn create_global_state() -> &'static mut GlobalState<'static> {
    // TODO: add error messages
    let global_state = Box::leak(Box::new(GlobalState {
        keywords: Vec::new(),
        variables: vec! {
            op_type!("add"), 
        }
    }));
    
    fn print(parameters: &Vec<Value>, _: &mut GlobalState) {
        let value = parameters.get(0).unwrap();
        let mut to_print = value.string.clone().unwrap();
        
        if value.references.is_some() {
            let reference_value = value.references.unwrap().value.string;
            if reference_value.is_none() { return; }
            to_print = reference_value.unwrap();
        }
        
        println!("{}", to_print);
    }
    let print_callback = Box::leak(Box::new(Callback {
        callback: print,
        parameter_count: 1,
        name: "print"
    }));
    // add_instruction(print_callback, &mut global_state);
    global_state.keywords.push(print_callback);

    let op = |parameters: &Vec<Value>, global_state: &mut GlobalState| {
        let operation = &parameters[0];
        let variable_name = &parameters[1];
        let x_value = &parameters[2];
        let y_value = &parameters[3];

        // TODO: support references
        if x_value.float.is_none() || y_value.float.is_none() {
            return;
        }

        let result = match operation.string.unwrap() {
            "add" => {
                x_value.float.unwrap() + y_value.float.unwrap()
            },
            &_ => todo!()
        };

        let variable = global_state.variables.iter_mut().find(|x| { x.name == variable_name.string.unwrap() });

        if variable.is_some() {
            variable.unwrap().value.float = Some(result);
            return;
        }
        
        let new_variable = Variable {
            name: "".to_string(),
            value: Value {
                string: None,
                references: None,
                float: Some(x_value.float.unwrap() + y_value.float.unwrap()),
            },
        };
        
        global_state.variables.push(new_variable);
    };
    let print_callback = Box::leak(Box::new(Callback {
        callback: op,
        parameter_count: 2,
        name: "op"
    }));
    // add_instruction(print_callback, &mut global_state);
    global_state.keywords.push(print_callback);
    
    fn noop(_: &Vec<Value>, _: &mut GlobalState) {
        black_box(0 + 0);
    }
    let callback = Box::leak(Box::new(Callback {
        callback: noop,
        parameter_count: 2,
        name: "noop"
    }));
    global_state.keywords.push(callback);
    
    global_state
}
