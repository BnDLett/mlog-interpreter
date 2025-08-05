use crate::interpreter::make_keyword_map;

mod interpreter;
mod standard_lib;

fn main() {
    let instructions = make_keyword_map();
}
