pub fn main() {
    let e = Expr::new_example();

    println!("{:#?}", e);
    println!("{:#?}", e.evaluate());
    println!("{:#?}", e.parse(&tokenize("1 + 2 / 3").unwrap()));

    return;
    loop {
        // Read input from the user and just do nothing when the input is empty
        let input = read_string();
        if input.is_empty() {
            continue;
        }

        // Debug output
        println!("\n{}\n", input);

        // tokenize output
        let foo = tokenize(&input);
        for bar in foo {
            println!("{:?}", bar)
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
                                '0'..='9' | '.'  => number_buffer.push(y),
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
    Number(f64),
    OpenParentheses,
    CloseParentheses,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Token {
    fn with_char(input: char) -> Result<Self, TokenParseError> {
        match input {
            '+' => Ok(Self::Addition),
            '-' => Ok(Self::Subtraction),
            '*' => Ok(Self::Multiplication),
            '/' => Ok(Self::Division),
            '(' => Ok(Self::OpenParentheses),
            ')' => Ok(Self::CloseParentheses),
            _ => Err(TokenParseError::UnknownChar(input)),
        }
    }
    fn with_number(input: &str) -> Result<Self, TokenParseError> {
        match input.parse() {
            Ok(success) => Ok(Self::Number(success)),
            Err(_) => Err(TokenParseError::InvalidNumberString(String::from(input))),
        }
    }
}

#[derive(Debug)]
enum TokenParseError {
    UnknownChar(char),
    InvalidNumberString(String),
}

#[derive(Debug)]
enum Op {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
enum LexErr {
    Unknown,
}

impl Op {
    fn apply(&self, left: f64, right: f64) -> Option<f64> {
        let result = match *self {
            Self::Addition => left + right,
            Self::Subtraction => left - right,
            Self::Multiplication => left * right,
            Self::Division => {
                if right == 0.0 {
                    f64::NAN
                } else {
                    left / right
                }
            }
        };
        if result.is_infinite() || result.is_nan() || result.is_subnormal() {
            return None
        }
        Some(result)
    }
}

#[derive(Debug)]
enum Expr {
    Leaf(f64),
    Branch {
        op: Op,
        children: Vec<Expr>,
    },
}

impl Expr {
    fn evaluate(&self) -> Option<f64> {
        match *self {
            Self::Leaf(data) => Some(data),
            Self::Branch { ref op, ref children } => {
                let mut children_iter = children.iter();
                let mut data = match children_iter.next() {
                    Some(expr) => match expr.evaluate() {
                        Some(res) => res,
                        None => return None,
                    },
                    None => return None,
                };
                while let Some(next) = children_iter.next() {
                    let next_evaluated = match next.evaluate() {
                        Some(x) => x,
                        None => return None,
                    };
                    data = match op.apply(data, next_evaluated) {
                        Some(res) => res,
                        None => return None,
                    };
                }
                Some(data)
            }
        }
    }
    fn new_example() -> Self {
        Self::Branch {
            op: Op::Addition,
            children: vec![
                Self::Leaf(3.0),
                Self::Branch {
                    op: Op::Subtraction,
                    children: vec![
                        Self::Leaf(6.0),
                        Self::Leaf(1.0),
                    ],
                },
            ],
        }
    }
    fn new_example_2() -> Self {
        Self::Branch {
            op: Op::Addition,
            children: vec![
                Self::Leaf(10.0),
                Self::Branch {
                    op: Op::Division,
                    children: vec![
                        Self::Leaf(6.0),
                        Self::Branch {
                            op: Op::Multiplication,
                            children: vec![
                                Self::Leaf(0.1),
                                Self::Leaf(20.0),
                                Self::Leaf(0.5),
                            ],
                        },
                        Self::Leaf(3.0),
                    ],
                },
            ],
        }
    }
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