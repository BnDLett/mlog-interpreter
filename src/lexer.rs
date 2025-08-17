const DOUBLE_QUOTE: char = '"';
const SINGLE_QUOTE: char = '\'';

pub enum Token<'a> {
    Keyword(&'a Callback<'a>),
    Parameter(Value<'a>)
}

#[derive(Clone)]
pub struct Callback<'a> {
    pub(crate) callback: fn(Vec<Value<'a>>),
    pub(crate) parameter_count: usize,
    pub(crate) name: String
}

pub struct Value<'a> {
    pub string: Option<String>,
    pub float: Option<f64>,
    pub references: Option<&'a Variable<'a>>,
}

pub struct Variable<'a> {
    value: Value<'a>,
    name: String
}

pub struct GlobalState<'a> {
    pub keywords: Vec<&'a Callback<'a>>,
    pub variables: Vec<Variable<'a>>,
}

pub fn tokenize_vec(source_code: Vec<String>, global_state: &'static GlobalState<'static>) -> Vec<Result<Vec<Token<'static>>, String>> {
    let mut result = Vec::new();
    
    for line in source_code {
        let tokens = tokenize_line(line, global_state);
        result.push(tokens)
    }
    
    result
}

pub fn tokenize_line(source_code: String, global_state: &'static GlobalState<'static>) -> Result<Vec<Token<'static>>, String> {
    let mut string_operator = None;
    let mut current_word = String::new();
    let mut _accumulator = 0usize;
    let mut result = Vec::new();
    
    if source_code.is_empty() {
        return Ok(vec![]);
    }
    
    for letter in source_code.chars() {
        _accumulator += 1;
        
        if (string_operator.is_some()) && (letter == string_operator.unwrap()) {
            string_operator = None;
            current_word.push(letter);
            
            continue;
        } else if ((letter == SINGLE_QUOTE) || (letter == DOUBLE_QUOTE)) && string_operator.is_none() {
            string_operator = Some(letter);
            current_word.push(letter);
            
            continue;
        };
        
        if letter == ' ' {
            let word = current_word.clone();
            current_word.clear();
            
            if word.is_empty() {
                continue;
            }
            
            let token = tokenize(word, global_state);
            result.push(token?);
            continue;
        }
        
        current_word.push(letter);
    }
    
    Ok(result)
}

pub fn tokenize(word: String, global_state: &'static GlobalState<'static>) -> Result<Token<'static>, String> {
    let keyword = get_keyword(word.clone(), global_state);
    
    if word.contains('"') || word.contains("'") {
        let word_length = word.len();
        let string = String::from(&word[1..(word_length - 1)]);
        let float = parse_float(string.clone()); // TODO: allow non-strict auto float parse?
        
        return Ok(Token::Parameter(Value {
            string: Some(string),
            float,
            references: None,
        }))
    }
    
    if keyword.is_none() {
        return Err(format!("Couldn't identify word: {}.", word));
    }
    
    Ok(Token::Keyword(keyword.unwrap()))
}

pub fn parse_float(word: String) -> Option<f64> {
    let parse_result = trim_whitespace(word).parse::<f64>();
    
    if parse_result.is_err() {
        return None;
    }
    
    Some(parse_result.unwrap())
}

pub fn trim_whitespace(s: String) -> String {
    // first attempt: allocates a vector and a string
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

pub fn get_keyword<'a>(word: String, global_state: &'a GlobalState<'a>) -> Option<&'a Callback<'a>> {
    global_state.keywords.iter().find(|x| {x.name == word}).cloned()
}

// pub fn add_instruction(callback: &'static Callback, global_state: &'static mut GlobalState<'static>) {
//     global_state.keywords.push(callback);
// }
