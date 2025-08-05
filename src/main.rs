use crate::interpreter::{interpret, make_keyword_map};
use crate::standard_lib::standard_lib_map;

mod interpreter;
mod standard_lib;

fn main() {
    let example_code = vec![
        "op add lorem 5 4",
        "op sub ipsum 10 lorem"
    ];
    let std = standard_lib_map();
    
    interpret(&std, &example_code).expect("me when i");
}
