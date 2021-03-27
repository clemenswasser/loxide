#[derive(Debug, PartialEq)]
enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String(String),
    Number(f32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

struct Tokenizer {
    script: String,
}

impl Tokenizer {
    pub fn new(script: String) -> Self {
        Self { script }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut iter = self.script.chars().peekable();

        let mut line = 0usize;

        while let Some(c) = iter.peek() {
            if let Some(token) = match c {
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '{' => Some(Token::LeftBrace),
                '}' => Some(Token::RightBrace),
                ',' => Some(Token::Comma),
                ';' => Some(Token::Semicolon),
                '.' => Some(Token::Dot),
                '-' => Some(Token::Minus),
                '+' => Some(Token::Plus),
                '*' => Some(Token::Star),
                '!' => {
                    iter.next();
                    match iter.peek() {
                        Some('=') => Some(Token::BangEqual),
                        _ => Some(Token::Bang),
                    }
                }
                '=' => {
                    iter.next();
                    match iter.peek() {
                        Some('=') => {
                            iter.next();
                            Some(Token::EqualEqual)
                        }
                        _ => Some(Token::Equal),
                    }
                }
                '>' => {
                    iter.next();
                    match iter.peek() {
                        Some('=') => {
                            iter.next();
                            Some(Token::GreaterEqual)
                        }
                        _ => Some(Token::Greater),
                    }
                }
                '<' => {
                    iter.next();
                    match iter.peek() {
                        Some('=') => {
                            iter.next();
                            Some(Token::LessEqual)
                        }
                        _ => Some(Token::Less),
                    }
                }
                '/' => {
                    iter.next();
                    match iter.peek() {
                        Some('/') => {
                            while let Some(c) = iter.peek() {
                                if *c == '\n' {
                                    break;
                                } else {
                                    iter.next();
                                }
                            }
                            None
                        }
                        _ => Some(Token::Slash),
                    }
                }
                '"' => {
                    iter.next();
                    let string = iter
                        .clone()
                        .take_while(|c| {
                            if *c == '\n' {
                                line += 1
                            }

                            if *c != '"' {
                                iter.next();
                            }

                            *c != '"'
                        })
                        .collect::<String>();

                    if iter.peek() != Some(&'"') {
                        eprintln!("Missing closing \" in line {}", line);
                        std::process::exit(1);
                    }

                    Some(Token::String(string))
                }
                ' ' | '\r' | '\t' => None,
                '\n' => {
                    line += 1;
                    None
                }
                _ => {
                    if c.is_ascii_digit() {
                        if let Ok(number) = iter
                            .clone()
                            .take_while(|c| {
                                if *c == '\n' {
                                    line += 1
                                }

                                if c.is_ascii_digit() || *c == '.' {
                                    iter.next();
                                }

                                c.is_ascii_digit() || *c == '.'
                            })
                            .collect::<String>()
                            .parse::<f32>()
                        {
                            Some(Token::Number(number))
                        } else {
                            eprintln!("Unexpected number literal at line {}", line);
                            std::process::exit(1);
                        }
                    } else {
                        eprintln!("Unexpected token at line {}", line);
                        std::process::exit(1);
                    }
                }
            } {
                tokens.push(token);
            }

            iter.next();
        }
        tokens.push(Token::Eof);
        tokens
    }
}

fn run(script: String) {
    for token in Tokenizer::new(script).tokenize() {
        println!("{:?}", &token);
    }
}

fn main() {
    if let Some(script) = std::env::args()
        .nth(1)
        .map(std::path::PathBuf::from)
        .map(std::fs::read_to_string)
        .filter(Result::is_ok)
        .map(Result::unwrap)
    {
        run(script);
    } else {
        eprintln!("Usage: loxide <script>");
        std::process::exit(1);
    }
}

mod test {
    use super::*;

    #[test]
    fn tokenize() {
        let script = r#"// Comment test
(( )){} // Grouping
! * + - / = < > == <= >= != // Operators
"Hello World!" // String
1234
12.34"#;
        let mut tokens = Tokenizer::new(script.to_string()).tokenize().into_iter();

        assert_eq!(tokens.next(), Some(Token::LeftParen));
        assert_eq!(tokens.next(), Some(Token::LeftParen));
        assert_eq!(tokens.next(), Some(Token::RightParen));
        assert_eq!(tokens.next(), Some(Token::RightParen));
        assert_eq!(tokens.next(), Some(Token::LeftBrace));
        assert_eq!(tokens.next(), Some(Token::RightBrace));

        assert_eq!(tokens.next(), Some(Token::Bang));
        assert_eq!(tokens.next(), Some(Token::Star));
        assert_eq!(tokens.next(), Some(Token::Plus));
        assert_eq!(tokens.next(), Some(Token::Minus));
        assert_eq!(tokens.next(), Some(Token::Slash));
        assert_eq!(tokens.next(), Some(Token::Equal));
        assert_eq!(tokens.next(), Some(Token::Less));
        assert_eq!(tokens.next(), Some(Token::Greater));
        assert_eq!(tokens.next(), Some(Token::EqualEqual));
        assert_eq!(tokens.next(), Some(Token::LessEqual));
        assert_eq!(tokens.next(), Some(Token::GreaterEqual));
        assert_eq!(tokens.next(), Some(Token::BangEqual));

        assert_eq!(
            tokens.next(),
            Some(Token::String("Hello World!".to_string()))
        );

        assert_eq!(tokens.next(), Some(Token::Number(1234.)));

        assert_eq!(tokens.next(), Some(Token::Number(12.34)));

        assert_eq!(tokens.next(), Some(Token::Eof));
        assert_eq!(tokens.next(), None);
    }
}
