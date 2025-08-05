use std::any::Any;
use std::collections::HashMap;
use crate::interpreter::{make_keyword_map, Callback};

pub fn standard_lib_map() -> HashMap<String, Callback> {
    let instructions = make_keyword_map();

    fn op(parameters: Vec<&str>, variables: &HashMap<String, dyn Any>) {
        let op_type = parameters[1];
        let var = parameters[2];
        let x_str = parameters[3];
        let y_str = parameters[4];

        let x_result = x_str.parse::<f32>();
        let y_result = y_str.parse::<f32>();

        let x_val: f32;
        let y_val: f32;

        if x_result.is_ok() {
            x_val = x_result.unwrap();
        } else {

        }

        match var {
            "add" => {

            },
            _ => {
                println!("Unrecognized operation.")
            }
        }
    }

    instructions
}
