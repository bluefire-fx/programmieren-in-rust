pub fn main() {
    loop {
        // Read input from the user and just do nothing when the input is empty
        let input = read_string();
        if input.is_empty() {
            continue;
        }

        // Debug output
        println!("\n{}\n", input);

        // tokenize output
        let mut foo = tokenize(&input);
        for bar in foo {
            println!("{:?}",bar)
        }
    }
}

fn tokenize(s: &str) -> Result<Vec<Token>, LexErr> {
    let mut tokens = Vec::new();
    let mut number_buffer= String::new();
    let mut s_iter = s.chars();

    loop {
        match s_iter.next() {
            Some(x) => match x {
                ' ' => (),
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    tokens.push(Token::with_char(x).expect("matched char already"));
                },
                '0'..='9' => {
                    number_buffer.push(x);
                    loop {
                        match s_iter.next() {
                            Some(y) => match y {
                                ' ' => match Token::with_number(&number_buffer) {
                                    Ok(res) => {
                                        tokens.push(res);
                                        number_buffer.clear();
                                        break;
                                    }
                                    _Err => return Err(LexErr::Unknown),
                                },
                                '+' | '-' | '*' | '/' | '(' | ')' => {
                                    match Token::with_number(&number_buffer) {
                                        Ok(res) => {
                                            tokens.push(res);
                                            number_buffer.clear();
                                            tokens.push(Token::with_char(y).expect("matched char already"));
                                            break;
                                        }
                                        _Err => return Err(LexErr::Unknown),
                                    }
                                }
                                '0'..='9' | '.' | ',' => number_buffer.push(y),
                                _ => return Err(LexErr::Unknown),
                            }
                            None => {
                                match Token::with_number(&number_buffer) {
                                    Ok(res) => {
                                        tokens.push(res);
                                        number_buffer.clear();
                                        break;
                                    }
                                    _Err => return Err(LexErr::Unknown),
                                }
                            }
                        }
                    }
                }
                _ => return Err(LexErr::Unknown),
            }
            None => {
                if number_buffer.len() > 0 {
                    match Token::with_number(&number_buffer) {
                        Ok(res) => tokens.push(res),
                        _Err => return Err(LexErr::Unknown),
                    }
                }
                break;
            }
        }
    }
    Ok(tokens)
}

#[derive(Debug)]
enum Token {
    Number(String),
    Operator(MathOperation),
    OpenParentheses,
    CloseParentheses,
}

impl Token {
    fn with_char(input: char) -> Result<Self, String> {
        match input {
            '+' => Ok(Token::Operator(MathOperation::Addition)),
            '-' => Ok(Token::Operator(MathOperation::Subtraction)),
            '*' => Ok(Token::Operator(MathOperation::Multiplication)),
            '/' => Ok(Token::Operator(MathOperation::Division)),
            '(' => Ok(Token::OpenParentheses),
            ')' => Ok(Token::CloseParentheses),
            _ => Err(format!("Cannot create token from char: {}", input)),
        }
    }
    fn with_number(input: &str) -> Result<Token, String> {
        return if valid(input) {
            Ok(Token::Number(String::from(input)))
        } else {
            Err(format!("Cannot create token from string: {}", input))
        };

        fn valid(input: &str) -> bool {
            let mut decimal_separator_counter = 0;
            for item in input.chars() {
                if decimal_separator_counter > 1 {
                    return false;
                }
                match item {
                    '0'..='9' => {},
                    '.' | ',' => decimal_separator_counter += 1,
                    _ => return false,
                }
            }
            true
        }
    }
}

#[derive(Debug)]
enum MathOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

enum LexErr {
    Unknown,
}

/// Reads a string from the user (with a nice prompt).
fn read_string() -> String {
    use std::io::Write;

    // Print prompt
    print!("calc > ");
    std::io::stdout().flush().unwrap();

    // Read line
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_end().len();
    buffer.truncate(new_len);

    buffer
}