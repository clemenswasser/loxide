#[derive(Debug, PartialEq)]
enum Token<'a> {
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
    String(&'a str),
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

struct Tokenizer<'a> {
    script: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(script: &'a str) -> Self {
        Self { script }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut iter = self.script.chars().enumerate().peekable();

        let mut line = 0usize;

        while let Some(&(i, c)) = iter.peek() {
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
                        Some((_, '=')) => Some(Token::BangEqual),
                        Some(_) => Some(Token::Bang),
                        _ => unimplemented!(),
                    }
                }
                '=' => {
                    iter.next();
                    match iter.peek() {
                        Some((_, '=')) => {
                            iter.next();
                            Some(Token::EqualEqual)
                        }
                        Some(_) => Some(Token::Equal),
                        _ => unimplemented!(),
                    }
                }
                '>' => {
                    iter.next();
                    match iter.peek() {
                        Some((_, '=')) => {
                            iter.next();
                            Some(Token::GreaterEqual)
                        }
                        Some(_) => Some(Token::Greater),
                        _ => unimplemented!(),
                    }
                }
                '<' => {
                    iter.next();
                    match iter.peek() {
                        Some((_, '=')) => {
                            iter.next();
                            Some(Token::LessEqual)
                        }
                        Some(_) => Some(Token::Less),
                        _ => unimplemented!(),
                    }
                }
                '/' => {
                    iter.next();
                    match iter.peek() {
                        Some((_, '/')) => {
                            while let Some((_, c)) = iter.peek() {
                                if *c == '\n' {
                                    break;
                                } else {
                                    iter.next();
                                }
                            }
                            None
                        }
                        Some(_) => Some(Token::Slash),
                        _ => unimplemented!(),
                    }
                }
                '"' => {
                    iter.next();

                    let chars = iter
                        .clone()
                        .take_while(|(_, c)| {
                            if *c == '\n' {
                                line += 1
                            }

                            if *c != '"' {
                                iter.next();
                            }

                            *c != '"'
                        })
                        .count();

                    match iter.peek() {
                        Some((_, '"')) => {}
                        _ => panic!("Missing closing \" in line {}", line),
                    }

                    Some(Token::String(&self.script[i + 1..i + 1 + chars]))
                }
                ' ' | '\r' | '\t' => None,
                '\n' => {
                    line += 1;
                    None
                }
                _ => {
                    if c.is_ascii_digit() {
                        if let Ok(number) = self.script[i..i + iter
                            .clone()
                            .take_while(|(_, c)| {
                                if *c == '\n' {
                                    line += 1
                                }

                                if c.is_ascii_digit() || *c == '.' {
                                    iter.next();
                                }

                                c.is_ascii_digit() || *c == '.'
                            })
                            .count()]
                            .parse::<f32>()
                        {
                            Some(Token::Number(number))
                        } else {
                            panic!("Unexpected number literal at line {}", line);
                        }
                    } else {
                        panic!("Unexpected token at line {}", line);
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
    for token in Tokenizer::new(script.as_str()).tokenize() {
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
        panic!("Usage: loxide <script>");
    }
}

mod test {
    use super::*;

    #[test]
    fn tokenize() {
        let script = "// Comment test\n\
                           (( )){}                     // Grouping\n\
                           ! * + - / = < > == <= >= != // Operators\n\
                           \"Hello World!\"              // String\n\
                           1234                        // Number\n\
                           12.34                       // Number";

        eprintln!("{}", script);
        let tokenizer = Tokenizer::new(script);
        let mut tokens = tokenizer.tokenize().into_iter();

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

        assert_eq!(tokens.next(), Some(Token::String("Hello World!")));

        assert_eq!(tokens.next(), Some(Token::Number(1234.)));

        assert_eq!(tokens.next(), Some(Token::Number(12.34)));

        assert_eq!(tokens.next(), Some(Token::Eof));
        assert_eq!(tokens.next(), None);
    }
}
