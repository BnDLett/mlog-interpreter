use std::collections::{BTreeMap};
use crate::interpreter::{add_instruction, make_keyword_map, Callback, GlobalVariables, VariableTypes};

/// Parses a value into a f64.
fn parse_value(value: &str, global_variables: &mut GlobalVariables) -> Result<f64, f64> {
    let result = value.parse::<f64>();
    // let result = lexical_core::parse(value.as_bytes());

    if result.is_ok() {
        Ok(result.unwrap())
    } else {
        let get_result = global_variables.variables.get(value);

        if get_result.is_err() {
            println!("Invalid parameter: {}. Defaulting to zero.", value);
            return Err(0f64);
        }

        let get_value = get_result.unwrap();

        if get_value.float.is_none() {
            println!("Invalid parameter: {}. Defaulting to zero.", value);
            Err(0f64)
        } else {
            Ok(get_value.float.unwrap())
        }
    }
}

fn  parse_string(value: String, global_variables: &mut GlobalVariables) -> String {
    let result;

    if value.starts_with('"') && value.ends_with('"') {
        result = String::from(&value[1..(value.len() - 1)]).replace("\\n", "\n")
    } else {
        let fallback_value = VariableTypes {
            string: Some(String::from("null")),
            float: Some(0f64),
        };
        let mut variable = global_variables.variables.get_or(&value, fallback_value);

        if variable.string.is_none() {
            let mut bytes = [b'0'; lexical_core::BUFFER_SIZE];
            let parsed = lexical_core::write(variable.float.unwrap(), &mut bytes);
            result = variable.string.insert(String::from_utf8_lossy(parsed).to_string()).clone();
        } else {
            result = variable.clone().string.unwrap();
        }
    }

    result
}

pub fn standard_lib_map() -> BTreeMap<String, Callback> {
    let mut instructions = make_keyword_map();

    fn set(parameters: Vec<&str>, global_variables: &mut GlobalVariables, _: &Vec<Vec<&str>>) {
        let variable_name = parameters[1];
        let value_str = &parameters[2..].join(" ");
        let value: VariableTypes;

        let parsed_float = parse_value(value_str, global_variables);
        // println!("{:?}", parsed_float);

        value = if parsed_float.is_ok() {
            // println!("parsed_str: {}", parsed_float.unwrap());
            VariableTypes {
                string: Some(parsed_float.unwrap().to_string()),
                float: Some(parsed_float.unwrap())
            }
        } else {
            let parsed_str = parse_string(value_str.clone(), global_variables);
            // println!("parsed_str: {}", parsed_str);
            
            VariableTypes {
                string: Some(parsed_str),
                float: None
            }
        };

        global_variables.variables.modify(variable_name, value);
    }
    add_instruction(&mut instructions, String::from("set"), Callback {
        parameter_count: 2,
        callback: set
    });

    fn op(parameters: Vec<&str>, global_variables: &mut GlobalVariables, _: &Vec<Vec<&str>>) {
        let op_type = parameters[1];
        let var = parameters[2];
        let x_str = parameters[3];
        let y_str = parameters[4];

        let x = parse_value(x_str, global_variables).unwrap_or(0f64);
        let y = parse_value(y_str, global_variables).unwrap_or(0f64);

        // println!("x: {} --- y: {}", x, y);

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
            
            "eq" => {
                (x == y) as i32 as f64
            }

            _ => {
                println!("Unrecognized operation. Defaulting to zero.");
                0f64
            }
        };
        
        global_variables.variables.modify(var, VariableTypes {
            float: Some(result),
            // string: Some(result.to_string())
            string: None
        });
    }
    add_instruction(&mut instructions, String::from("op"), Callback {
        parameter_count: 4,
        callback: op
    });

    fn jump(parameters: Vec<&str>, global_variables: &mut GlobalVariables, _: &Vec<Vec<&str>>) {
        let position_str = parameters[1];
        let operation = parameters[2];
        let x_str = parameters[3];
        let y_str = parameters[4];
        
        let position = parse_value(position_str, global_variables).unwrap_or(0f64);
        let x = parse_value(x_str, global_variables);
        let y = parse_value(y_str, global_variables);
        
        let should_jump;
        
        should_jump = match operation {
            "equal" => x == y,
            "notEqual" => x != y,
            
            _ => {
                println!("Unrecognized operation. Defaulting to false.");
                false
            }
        };
        
        if !should_jump {
            return;
        }

        global_variables.position = (position as usize) - 1;
        // global_variables.position = position as usize;
        //
        // let line_parameters = &code[global_variables.position - 1];
        //
        // if line_parameters.len() == 0 {
        //     return;
        // }
        //
        // let func = global_variables.instruction_map[line_parameters[0]].callback;
        // func(line_parameters.clone(), global_variables, code);
    }
    add_instruction(&mut instructions, String::from("jump"), Callback {
        parameter_count: 4,
        callback: jump
    });

    fn stop(_: Vec<&str>, global_variables: &mut GlobalVariables, _: &Vec<Vec<&str>>) {
        // In theory, this should trigger the condition to stop the execution. This is due to it
        // ending execution once the position is larger than the code vector.
        global_variables.position = usize::MAX - 1;
    }
    add_instruction(&mut instructions, String::from("stop"), Callback {
        parameter_count: 0,
        callback: stop
    });
    
    fn end(_: Vec<&str>, global_variables: &mut GlobalVariables, _: &Vec<Vec<&str>>) {
        global_variables.position = 0;
    }
    add_instruction(&mut instructions, String::from("end"), Callback {
        parameter_count: 0,
        callback: end
    });

    fn printbuf(parameters: Vec<&str>, global_variables: &mut GlobalVariables, _: &Vec<Vec<&str>>) {
        let parameter = parameters[1..].join(" ");
        let to_print: String = parse_string(parameter, global_variables);

        global_variables.print_buffer.push(to_print);
    }
    add_instruction(&mut instructions, String::from("print"), Callback {
        parameter_count: 0,
        callback: printbuf
    });

    fn printflush(parameters: Vec<&str>, global_variables: &mut GlobalVariables, _: &Vec<Vec<&str>>) {
        let out = parameters[1];
        println!("{}: {}", out, global_variables.print_buffer.join(""));
        
        global_variables.print_buffer.clear();
    }
    add_instruction(&mut instructions, String::from("printflush"), Callback {
        parameter_count: 1,
        callback: printflush
    });

    instructions
}
