use std::collections::HashMap;

#[derive(Debug)]
pub enum VariableTypes {
    Str(&'static str),
    Float(f32),
}

pub struct GlobalVariables {
    pub position: usize,
    pub variables: VariableMap,
    pub print_buffer: Vec<String>
}

pub struct VariableMap {
    variables: HashMap<String, VariableTypes>
}

impl VariableMap {
    pub fn new() -> Self {
        Self {variables: HashMap::new()}
    }
    
    pub fn modify(&mut self, name: &str, value: VariableTypes) {
        self.variables.insert(name.parse().unwrap(), value);
    }
    
    pub fn get(&self, name: &str) -> Result<&VariableTypes, ()> {
        self.variables.get(name).ok_or(())
    }
    
    pub fn get_or(&self, name: &str, fallback: &'static VariableTypes) -> &VariableTypes {
        if !self.variables.contains_key(name) {
            return fallback;
        }
        
        self.variables.get(name).unwrap()
    }
}

// This helps simplify the type parameter for the callback into a much simpler and easier to adjust
// type. This will also help simplify the verification of the parameter count. 
pub struct Callback {
    pub parameter_count: usize,
    pub callback: fn(Vec<&str>, &mut GlobalVariables)
}

/// Interprets a vector of primitive strings. These strings should represent valid mlog.
pub fn interpret(instruction_map: &HashMap<String, Callback>, code: &Vec<&str>) -> Result<(), &'static str> {
    let mut global_state = GlobalVariables {
        position: 1usize, 
        variables: VariableMap::new(),
        print_buffer: vec![]
    };

    loop {
        if global_state.position - 1 > code.len() {
            break;
        } else if global_state.position - 1 == code.len() {
            global_state.position = 1;
        }
        
        let line = code[global_state.position - 1];
        let line_parameters: Vec<&str> = line.split(" ").collect();
        
        if !instruction_map.contains_key(line_parameters[0]) {
            return Err("Unrecognized keyword.")
        } else if line_parameters.len() == 0 {
            continue
        }

        let func = instruction_map[line_parameters[0]].callback;
        func(line_parameters, &mut global_state);
        // println!("{}", global_state.position);

        global_state.position += 1;
    }
    
    // println!("{:?}", global_state.variables.variables);
    Ok(())
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
