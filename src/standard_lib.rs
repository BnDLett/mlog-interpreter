use std::collections::HashMap;
use crate::interpreter::{add_instruction, make_keyword_map, Callback, GlobalVariables, VariableMap, VariableTypes};

/// Parses a value into a f32.
fn parse_value(value: &str, global_variables: &GlobalVariables) -> f32 {
    let result = value.parse::<f32>();

    if result.is_ok() {
        result.unwrap()
    } else {
        let get_result = global_variables.variables.get(value);

        if get_result.is_err() {
            println!("Invalid parameter: {}. Defaulting to zero.", value);
            return 0f32;
        }
        
        let get_value = get_result.unwrap();

        match get_value {
            VariableTypes::Float(var_value) => {
                *var_value
            }
            _ => {
                println!("Could find variable {}. Defaulting to zero.", value);
                0f32
            }
        }
    }
}

pub fn standard_lib_map() -> HashMap<String, Callback> {
    let mut instructions = make_keyword_map();

    fn op(parameters: Vec<&str>, global_variables: &mut GlobalVariables) {
        let op_type = parameters[1];
        let var = parameters[2];
        let x_str = parameters[3];
        let y_str = parameters[4];

        let x = parse_value(x_str, &global_variables);
        let y = parse_value(y_str, &global_variables);
        
        let result = match op_type {
            "add" => {
                x + y
            }
            "sub" => {
                x - y
            }
            "mul" => {
                x * y
            }
            "div" => {
                x / y
            }
            
            _ => {
                println!("Unrecognized operation. Defaulting to zero.");
                0f32
            }
        };
        
        global_variables.variables.modify(var, VariableTypes::Float(result));
    }
    add_instruction(&mut instructions, String::from("op"), Callback {
        parameter_count: 4,
        callback: op
    });

    instructions
}
