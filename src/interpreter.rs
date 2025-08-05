use std::collections::HashMap;

// This helps simplify the type parameter for the callback into a much simpler and easier to adjust
// type. This will also help simplify the verification of the parameter count. 
pub struct Callback {
    parameter_count: usize,
    callback: &'static dyn Fn(Vec<&str>)
}

/// Interprets a vector of primitive strings. These strings should represent valid mlog.
pub fn interpret(instruction_map: &HashMap<String, Callback>, code: Vec<&str>) -> Result<None, &'static str> {
    for line in code {
        let line_parameters: Vec<&str> = line.split(" ").collect();
        
        if !instruction_map.contains_key(line_parameters[0]) {
            return Err("Unrecognized keyword.")
        } else if line_parameters.len() == 0 {
            continue
        }
        
        let callback = &instruction_map[line_parameters[0]];
        let func = callback.func;
        func(line_parameters);
    }
    
    Ok(instruction_map)
}

/// Adds an instruction into the hashmap for the interpreter. 
pub fn add_instruction(instruction_map: &mut HashMap<String, Callback>, instruction: String,
                       callback: Callback) -> &HashMap<String, Callback> {
    instruction_map.insert(instruction, callback);
    instruction_map
}

/// Create and return a new keyword map. This simplifies the creation of the keyword map itself.
/// Otherwise, you'd have to manually state the type of the HashMap.
pub fn make_keyword_map() -> HashMap<String, Callback> {
    let map: HashMap<String, Callback> = HashMap::new();
    map
}
