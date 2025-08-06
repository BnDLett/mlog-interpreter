use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
#[derive(Clone)]
pub struct VariableTypes {
    pub string: Option<String>,
    pub float: Option<f32>
}

pub struct GlobalVariables<'a> {
    pub position: usize,
    pub variables: VariableMap,
    pub print_buffer: Vec<String>,
    pub instruction_map: &'a BTreeMap<String, Callback>
}

#[derive(Clone)]
pub struct VariableMap {
    variables: HashMap<String, VariableTypes>
}

impl VariableMap {
    pub fn new() -> Self {
        Self {variables: HashMap::with_capacity(64)}
    }
    
    pub fn modify(&mut self, name: &str, value: VariableTypes) {
        self.variables.insert(String::from(name), value);
    }
    
    pub fn get(&mut self, name: &str) -> Result<&mut VariableTypes, ()> {
        self.variables.get_mut(name).ok_or(())
    }
    
    pub fn get_or(&mut self, name: &str, fallback: VariableTypes) -> VariableTypes {
        if !self.variables.contains_key(name) {
            return fallback;
        }
        
        self.variables.get_mut(name).unwrap().clone()
    }
}

// This helps simplify the type parameter for the callback into a much simpler and easier to adjust
// type. This will also help simplify the verification of the parameter count.
#[allow(dead_code)]
pub struct Callback {
    pub parameter_count: usize,
    pub callback: fn(Vec<&str>, &mut GlobalVariables, &Vec<Vec<&str>>)
}

/// Interprets a vector of primitive strings. These strings should represent valid mlog.
pub fn interpret(instruction_map: &BTreeMap<String, Callback>, code: &Vec<&str>) -> Result<u128, &'static str> {
    let mut global_state = GlobalVariables {
        position: 1usize, 
        variables: VariableMap::new(),
        print_buffer: vec![],
        instruction_map
    };

    let mut processed_lines = vec![];
    
    for line in code {
        let line_parameters: Vec<&str> = line.split(" ").collect();
        
        if line_parameters.is_empty() || line_parameters[0] == "" {
            continue;
        }
        
        if !instruction_map.contains_key(line_parameters[0]) {
            return Err("Unrecognized keyword.")
        }
        
        processed_lines.push(line_parameters);
    }

    let mut accumulator = 0u128;

    loop {
        if global_state.position - 1 > processed_lines.len() {
            break;
        } else if global_state.position - 1 == processed_lines.len() {
            global_state.position = 1;
        }
        
        let line_parameters = &processed_lines[global_state.position - 1];
        
        if line_parameters.len() == 0 {
            continue
        }

        let func = instruction_map[line_parameters[0]].callback;
        func(line_parameters.clone(), &mut global_state, &processed_lines);
        // println!("{}", global_state.position);

        global_state.position += 1;
        accumulator += 1;
        // println!("{:?}", global_state.variables.variables);
    }
    
    Ok(accumulator)
}

/// Adds an instruction into the BTreeMap for the interpreter. 
pub fn add_instruction(instruction_map: &mut BTreeMap<String, Callback>, instruction: String,
                       callback: Callback) -> &BTreeMap<String, Callback> {
    instruction_map.insert(instruction, callback);
    instruction_map
}

/// Create and return a new keyword map. This simplifies the creation of the keyword map itself.
/// Otherwise, you'd have to manually state the type of the BTreeMap.
pub fn make_keyword_map() -> BTreeMap<String, Callback> {
    let map: BTreeMap<String, Callback> = BTreeMap::new();
    map
}
