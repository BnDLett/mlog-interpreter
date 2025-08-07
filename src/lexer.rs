use std::collections::{BTreeMap, HashMap};

const MAX_INSTRUCTIONS: usize = 1024;
const MAX_VARIABLES: usize = 1024;

// there's nothing we can do for memory alignment :(
// --- napoleon the programmer
#[derive(Clone)]
pub struct Instruction {
    parameters: u8,
    callback: fn(Vec<Variable>, &mut GlobalVariables)
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Variable {
    pub string: Option<String>, // string representation of variable
    pub float: Option<f64>,     // float representation of variable, if there is one.
    pub name: Option<String>
}

#[derive(Clone)]
pub struct VariableMap {
    variables: HashMap<String, Variable>
}

impl VariableMap {
    pub fn new() -> Self {
        Self {variables: HashMap::with_capacity(64)}
    }
    
    pub fn modify(&mut self, name: &str, value: Variable) {
        self.variables.insert(String::from(name), value);
    }
    
    pub fn get(&mut self, name: &str) -> Result<&mut Variable, ()> {
        self.variables.get_mut(name).ok_or(())
    }
    
    pub fn get_or(&mut self, name: &str, fallback: Variable) -> Variable {
        if !self.variables.contains_key(name) {
            return fallback;
        }
        
        self.variables.get_mut(name).unwrap().clone()
    }
}

pub struct Token {
    pub instruction: Instruction,
    pub parameters: Vec<Variable>
}

pub struct GlobalVariables<'a> {
    pub position: usize,
    // pub variable_map: VariableMap,             // maps a string to a variable
    pub variables: [Variable; MAX_VARIABLES],  // the variables themselves
    pub print_buffer: Vec<String>,
    #[allow(dead_code)]
    pub instruction_map: &'a HashMap<String, Instruction>
}

pub struct Program {
    instructions: Vec<Token>,
    variables: [Variable; MAX_VARIABLES]
}

/// Adds an instruction into the BTreeMap for the interpreter. 
pub fn add_instruction(instruction_map: &mut BTreeMap<String, Instruction>, instruction: String,
                       callback: Instruction) -> &BTreeMap<String, Instruction> {
    instruction_map.insert(instruction, callback);
    instruction_map
}

pub fn tokenize_vec(instructions: &BTreeMap<String, Instruction>, code: Vec<String>, 
                    global_variables: &mut GlobalVariables) -> Result<Program, String> {
    let mut tokens = vec![];
    
    let mut token;
    let mut accumulator = 1;
    
    for line in code {
        token = tokenize(instructions, line, accumulator, global_variables);
        tokens.push(token?);
        
        accumulator += 1;
    }
    
    Ok(Program {
        instructions: tokens,
        variables: global_variables.variables.clone(),
    })
}

pub fn tokenize(instructions: &BTreeMap<String, Instruction>, line: String, position: usize, 
                global_variables: &mut GlobalVariables) -> Result<Token, String> {
    if line.ends_with(":") {
        return Err(String::from("Labels are not yet supported."));
    }
    
    let in_string = false;
    let instruction_str = line.split(" ").collect::<Vec<_>>()[0];
    let instruction_result = instructions.get(instruction_str);
    
    if instruction_result.is_none() {
        let error_message = String::from("Could not find instruction on line ");
        return Err(error_message + &*position.to_string());
    }
    
    const QUOTE: char = '"';
    const BACKSLASH: char = '\\';
    const SPACE: char = ' ';
    
    let mut previous_char = char::from(u8::MAX); // Use MAX to prevent accidental collisions
    let mut parameters = vec![];
    let mut current_word = String::from("");
    
    for ch in line.chars() {
        if ch == QUOTE && previous_char != BACKSLASH {
            in_string == !in_string;
            continue
        }
        
        if ch == SPACE && !in_string {
            let var = process_word(current_word.clone(), global_variables);
            
            parameters.push(var?);
            current_word.clear();
        }
        
        previous_char = ch;
        current_word.push(ch);
    }
    
    Ok(Token {
        instruction: instruction_result.unwrap().clone(),
        parameters
    })
}

pub fn process_word(word: String, global_variables: &mut GlobalVariables) -> Result<Variable, String> {
    let float_parse_result = word.parse::<f64>();
    
    if float_parse_result.is_ok() {
        return Ok(Variable {
            string: Some(word.clone()),
            float: Some(float_parse_result.unwrap()),
            name: None
        });
    }
    
    if word.contains(" ") {
        return Ok(Variable {
            string: Some(word),
            float: None,
            name: None
        });
    }
    
    global_variables.variables.sort_by_key(|_| {});
    let variable_index = global_variables.variables.binary_search_by_key(&word, |a| {a.clone().name.unwrap()});
    
    if variable_index.is_err() {
        return Err(format!("Unknown variable: {}", word));
    }
    
    let variable = &global_variables.variables[variable_index.unwrap()];
    Ok(variable.clone())
}
