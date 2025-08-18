// Whether certain values should be parsed ahead of time or not. Recommended value is `true`.
const PARSE_DATA_AHEAD: bool = true;

const DOUBLE_QUOTE: char = '"';
const SINGLE_QUOTE: char = '\'';

pub enum Token<'a> {
    Keyword(&'a Callback<'a>),
    Parameter(Value<'a>)
}

#[derive(Clone)]
pub struct Callback<'a> {
    pub callback: fn(&Vec<&Value>),
    pub name: &'a str,
    pub parameter_count: usize,
}

pub struct Value<'a> {
    pub string: Option<&'a str>,
    pub references: Option<&'a Variable<'a>>,
    pub float: Option<f64>,
}

pub struct Variable<'a> {
    pub name: String,
    pub value: Value<'a>,
}

pub struct GlobalState<'a> {
    pub keywords: Vec<&'a Callback<'a>>,
    pub variables: Vec<Variable<'a>>,
}

pub fn tokenize_vec(source_code: Vec<String>, global_state: &'static GlobalState<'static>) -> Result<Vec<Vec<Token<'static>>>, String> {
    let mut result = Vec::with_capacity(source_code.len());

    for line in source_code {
        let tokens = tokenize_line(line, global_state);
        result.push(tokens?)
    }

    Ok(result)
}

pub fn tokenize_line(source_code: String, global_state: &'static GlobalState<'static>) -> Result<Vec<Token<'static>>, String> {
    let mut string_operator = None;
    let mut current_word = String::new();
    let mut _accumulator = 0usize;
    let mut result = Vec::new();
    let mut at_end;

    if source_code.is_empty() {
        return Ok(vec![]);
    }

    for letter in source_code.chars() {
        _accumulator += 1;
        at_end = _accumulator == source_code.len();

        if (string_operator.is_some()) && (letter == string_operator.unwrap()) {
            string_operator = None;
            current_word.push(letter);
        } else if ((letter == SINGLE_QUOTE) || (letter == DOUBLE_QUOTE)) && string_operator.is_none() {
            string_operator = Some(letter);
            current_word.push(letter);

            continue;
        };

        if (letter == ' ' || at_end) && string_operator.is_none() {
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
    let keyword = get_keyword(&word, global_state);

    if word.contains('"') || word.contains("'") {
        let word_length = word.len();
        let string = &word[1..(word_length - 1)];
        let mut float = None;
        
        if PARSE_DATA_AHEAD {
            float = parse_float(word.clone());
        }
        
        return Ok(Token::Parameter(Value {
            string: Some(String::from(string).leak()),
            float,
            references: None
        }))
    }
    
    let float = parse_float(word.clone());
    
    if float.is_some() {
        if !PARSE_DATA_AHEAD {
            return Ok(Token::Parameter(Value {
                string: None,
                float,
                references: None
            }));
        }
        
        return Ok(Token::Parameter(Value {
            string: Some(float.unwrap().to_string().leak()),
            float,
            references: None
        }))
    }
    
    let variable = global_state.variables.iter().find(|x| {x.name == word});
    
    if variable.is_some() {
        return Ok(Token::Parameter(Value {
            string: None,
            float: None,
            references: variable
        }));
    }

    if keyword.is_none() {
        return Err(format!("Couldn't identify word: {}.", word));
    }

    Ok(Token::Keyword(keyword.unwrap()))
}

pub fn parse_float(word: String) -> Option<f64> {
    // println!("{}", word);
    let parse_result = word.trim().parse::<f64>();

    if parse_result.is_err() {
        return None;
    }

    Some(parse_result.unwrap())
}

// pub fn trim_whitespace(s: String) -> String {
//     // first attempt: allocates a vector and a string
//     let words: Vec<_> = s.split_whitespace().collect();
//     words.join(" ")
// }

pub fn get_keyword<'a>(word: &String, global_state: &'a GlobalState<'a>) -> Option<&'a Callback<'a>> {
    global_state.keywords.iter().find(|x| {x.name == word}).cloned()
}

// pub fn get_spaces_count(string: &String) -> usize {
//     let mut count = 0usize;
//     
//     for char in string.chars() {
//         if char == ' ' { count += 1 }
//     }
//     
//     count
// }

// pub fn add_instruction(callback: &'static Callback, global_state: &'static mut GlobalState<'static>) {
//     global_state.keywords.push(callback);
// }
