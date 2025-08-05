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

    fn jump(parameters: Vec<&str>, global_variables: &mut GlobalVariables) {
        let position_str = parameters[1];
        let position = parse_value(position_str, &global_variables);

        // one is subtracted since it'll be added again by the end of execution.
        global_variables.position = (position as usize) - 1;
    }
    add_instruction(&mut instructions, String::from("jump"), Callback {
        parameter_count: 1,
        callback: jump
    });

    fn stop(parameters: Vec<&str>, global_variables: &mut GlobalVariables) {
        // In theory, this should trigger the condition to stop the execution. This is due to it
        // ending execution once the position is larger than the code vector.
        global_variables.position = usize::MAX - 1;
    }
    add_instruction(&mut instructions, String::from("stop"), Callback {
        parameter_count: 0,
        callback: stop
    });
    
    fn end(parameters: Vec<&str>, global_variables: &mut GlobalVariables) {
        global_variables.position = 0;
    }
    add_instruction(&mut instructions, String::from("end"), Callback {
        parameter_count: 0,
        callback: end
    });

    fn printbuf(parameters: Vec<&str>, global_variables: &mut GlobalVariables) {
        let parameter = parameters[1..].join(" ");
        let to_print: String;
        
        if parameter.starts_with('"') && parameter.ends_with('"') {
            to_print = String::from(&parameter[1..(parameter.len() - 1)])
        } else {
            let variable = global_variables.variables.get_or(&parameter, &VariableTypes::Str("null"));
            
            to_print = match variable {
                VariableTypes::Float(value) => value.to_string(),
                VariableTypes::Str(value) => String::from(value.clone()),
                // _ => to_print = String::from("null") // false null lmaooooo
            };
        }

        global_variables.print_buffer.push(to_print);
    }
    add_instruction(&mut instructions, String::from("print"), Callback {
        parameter_count: 0,
        callback: printbuf
    });

    fn printflush(parameters: Vec<&str>, global_variables: &mut GlobalVariables) {
        let out = parameters[1];
        println!("{}: {}", out, global_variables.print_buffer.join("\n"));
        
        global_variables.print_buffer.clear();
    }
    add_instruction(&mut instructions, String::from("printflush"), Callback {
        parameter_count: 1,
        callback: printflush
    });

    instructions
}
