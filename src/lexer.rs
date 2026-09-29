use std::fmt::{self};
use crate::error::LexerError;


#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Identifier(String), // [a-zA-Z][a-zA-Z0-9]*
    IntLiteral(i32),   // [1-9][0-9]*
    StringLiteral(String),  // ".*"
    Int,
    Plus,   // +
    Minus, // -
    Mul, // *
    Div, // /
    Modulo, // %
    Assign, // =
    Equal, // == 
    NotEqual, // !=
    LessThan, // <
    LessThanEqual, // <=
    GreaterThan, // >
    GreaterThanEqual, // >=
    If, // if
    Else, // else
    While, // while
    Semi, // ;
    LPar, // (
    RPar, // )
    LCurl, // {
    RCurl, // }
    Print, // print
    Comma, // ,
    Return, // return
    EOF
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Token {
        Token {kind, line, col }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Identifier(ident) => write!(f,"{}", ident),
            TokenKind::IntLiteral(value) => write!(f,"{}", &value.to_string()),
            TokenKind:: StringLiteral(s) => write!(f,"{}", s),
            TokenKind::Int => write!(f,"int"),
            TokenKind::Plus =>  write!(f,"+"),
            TokenKind::Minus =>  write!(f,"-"),
            TokenKind::Mul =>  write!(f,"*"),
            TokenKind::Div =>  write!(f,"/"),
            TokenKind::Modulo => write!(f, "%"),
            TokenKind::Assign =>  write!(f,"="),
            TokenKind::Equal =>  write!(f,"=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::LessThan =>  write!(f,"<"),
            TokenKind::LessThanEqual =>  write!(f,"<="),
            TokenKind::GreaterThan =>  write!(f,">"),
            TokenKind::GreaterThanEqual =>  write!(f,">="),
            TokenKind::If =>  write!(f,"if"),
            TokenKind::Else =>  write!(f,"else"),
            TokenKind::While =>  write!(f,"while"),
            TokenKind::Semi =>  write!(f,";"),
            TokenKind::LPar =>  write!(f,"("),
            TokenKind::RPar =>  write!(f,")"),
            TokenKind::LCurl =>  write!(f,"{{"),
            TokenKind::RCurl =>  write!(f,"}}"),
            TokenKind::Print =>  write!(f,"print"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Return => write!(f, "return"),
            TokenKind::EOF => write!(f, "EOF")
        }
    }
}

#[derive(PartialEq)]
enum State {
    Start,
    InInt,
    InString,
    InIdent,
}


pub struct Lexer {
    state: State,
    input: String,
    current: usize,
    line: usize,
    col: usize,
}


impl Lexer {

    pub fn new(input: String) -> Lexer {
        Lexer { state: State::Start, input, current: 0, line: 0, col: 0 }
    }

    fn new_token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.line, self.col)
    }


    fn next(&mut self) -> Result<Option<Token>, LexerError> {

        self.state = State::Start;
        let mut start_pos = self.current;

        while self.current < self.input.len() {

            let c = self.input.as_bytes()[self.current];

            match self.state {
                State::Start => {              
                    match c {
                        b' ' => {
                            self.current += 1;
                            self.col += 1;
                            start_pos = self.current;
                        },
                        b'\n' => {
                            self.line += 1;
                            self.current += 1;
                            self.col = 0;
                            start_pos = self.current;
                        }
                        b'+' => {
                            let token = self.new_token(TokenKind::Plus);
                            self.current += 1;
                            self.col += 1;
                            return Ok(Some(token));
                        },
                        b'-' => {
                            let token = self.new_token(TokenKind::Minus);
                            self.current += 1;
                            self.col += 1;
                            return Ok(Some(token));
                        }
                        b'*' => {
                            let token = self.new_token(TokenKind::Mul);
                            self.current += 1;
                            self.col += 1;
                            return Ok(Some(token));
                        },
                        b'/' => {
                            self.current += 1;
                            let next = self.input.as_bytes()[self.current];
                            
                            match next {
                                b'*' => {
                                    self.current += 1;
                                    
                                    'l1: loop {
                                        let c = self.input.as_bytes()[self.current];

                                        match c {
                                            b'*' => {
                                                self.current += 1;

                                                let c = self.input.as_bytes()[self.current];

                                                if c == b'/' {
                                                    self.current += 1;
                                                    break 'l1;
                                                }
                                            },
                                            _ => {
                                                self.current += 1;
                                            }
                                        }
                                    }
                                }
                                b'/' => {
                                    self.current += 1;

                                    'l1: loop {
                                        let c = self.input.as_bytes()[self.current];

                                        match c {
                                            b'\n' => {
                                                break 'l1;
                                            }
                                            _ => {
                                                self.current += 1;
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    let token = self.new_token(TokenKind::Div);
                                    self.col += 1;
                                    return Ok(Some(token));
                                }
                            }          
                        },
                        b'%' => {
                            let token = self.new_token(TokenKind::Modulo);
                            self.current += 1;
                            self.col += 1;
                            return Ok(Some(token));
                        }
                        b';' => {
                            let token = self.new_token(TokenKind::Semi);
                            self.current += 1;
                            self.col += 1;
                            return Ok(Some(token));
                        },
                        b',' => {
                            let token = self.new_token(TokenKind::Comma);
                            self.current += 1;
                            self.col += 1;
                            return Ok(Some(token));
                        },
                        b'a'..=b'z' | b'A'..=b'Z' => {
                            self.current += 1;
                            self.state = State::InIdent;
                        }
                        b'0'..=b'9' => {
                            self.current += 1;
                            self.state = State::InInt;
                        }
                        b'=' => {
                            let next = self.input.as_bytes()[self.current+1];

                            match next {
                                b'=' => {
                                    let token = self.new_token(TokenKind::Equal);
                                    self.current += 2;
                                    self.col += 2;
                                    return Ok(Some(token));
                                },
                                _ => {
                                    let token = self.new_token(TokenKind::Assign);
                                    self.current += 1;
                                    self.col += 1;
                                    return Ok(Some(token));
                                }
                            }
                        },
                        b'!' => {
                            let next = self.input.as_bytes()[self.current+1];

                            match next {
                                b'=' => {
                                    let token = self.new_token(TokenKind::NotEqual);
                                    self.current += 2;
                                    self.col += 2;
                                    return Ok(Some(token));
                                }
                                _ => {
                                    return Err(LexerError(format!("Invalid symbol '{}'", c)))
                                }
                            }
                        },
                        b'(' => {
                            let token = self.new_token(TokenKind::LPar);
                            self.current += 1;
                            self.col += 1;
                            return Ok(Some(token));
                        },
                        b')' => {
                            let token = self.new_token(TokenKind::RPar);
                            self.current += 1;
                            self.col += 1;
                            return Ok(Some(token));
                        },
                        b'<' => {
                            let next = self.input.as_bytes()[self.current+1];

                            match next {
                                b'=' => {
                                    let token = self.new_token(TokenKind::LessThanEqual);
                                    self.current += 2;
                                    self.col += 2;      
                                    return Ok(Some(token));
                                },
                                _ => {
                                    let token = self.new_token(TokenKind::LessThan);
                                    self.current += 1;
                                    self.col += 1;
                                    return Ok(Some(token));
                                }
                            }
                        },
                        b'>' => {
                            let next = self.input.as_bytes()[self.current+1];

                            match next {
                                b'=' => {
                                    let token = self.new_token(TokenKind::GreaterThanEqual);
                                    self.current += 2;
                                    self.col += 2;      
                                    return Ok(Some(token));
                                },
                                _ => {
                                    let token = self.new_token(TokenKind::GreaterThan);
                                    self.current += 1;
                                    self.col += 1;
                                    return Ok(Some(token));
                                }
                            }
                        },
                        b'"' => {
                            self.current += 1;
                            self.col += 1;
                            start_pos += 1;
                            self.state = State::InString;
                        },
                        b'{' => {
                            let token = self.new_token(TokenKind::LCurl);
                            self.current += 1;
                            self.col += 1;
                            return Ok(Some(token));
                        },
                        b'}' => {
                            let token = self.new_token(TokenKind::RCurl);
                            self.current += 1;
                            self.col += 1;
                            return Ok(Some(token));
                        }
                        _ => {
                            return Err(LexerError(format!("Invalid symbol '{}'", c)))
                        },
                    }   
                },
                State::InInt => {
                    match c {
                        b'0'..=b'9' => {
                            self.current += 1;
                        }
                        _ => {
                            let len = self.current - start_pos;
                            let value = str::parse::<i32>(&self.input[start_pos..self.current]).unwrap();

                            let token = self.new_token(TokenKind::IntLiteral(value));
                            self.col += len;

                            return Ok(Some(token));
                        }
                    }
                },
                State::InIdent => {
                    match c {
                        b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9'  => {
                            self.current += 1;
                        }
                        _ => {
                            let len = self.current - start_pos;
                            let value = self.input[start_pos..self.current].to_string();
                            let token = match value.as_str() {
                                "int" => { self.new_token(TokenKind::Int) },
                                "if" => { self.new_token(TokenKind::If )},
                                "else" => { self.new_token(TokenKind::Else) },
                                "while" => { self.new_token(TokenKind::While) },
                                "print" => { self.new_token(TokenKind::Print) },
                                "return" => { self.new_token(TokenKind::Return )},
                                _ => { self.new_token(TokenKind::Identifier(value)) }
                            };
                            self.col += len;
                            return Ok(Some(token));
                        }
                    }
                },
                State::InString => {
                    match c {
                        b'"' => {
                            //let len = self.current - start_pos;
                            let value = self.input[start_pos..self.current].to_string();
                            let token = self.new_token(TokenKind::StringLiteral(value));
                            self.current += 1;
                            self.col += 1;

                            return Ok(Some(token));
                        },
                        _ => {
                            self.current += 1;
                            self.col += 1;
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    pub fn parse_all(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next()? {
            tokens.push(token);
        }
        tokens.push(self.new_token(TokenKind::EOF));

        Ok(tokens)
    }
}