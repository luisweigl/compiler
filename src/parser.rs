use crate::{error::ParserError, lexer::{Token, TokenKind}};

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Modulo,
    Equal, NotEqaul, LessThan, LessThanEqual, GreaterThan, GreaterThanEqual
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Decl {
        var_type: Type,
        identifier: String,
        init: Option<Expr>,
    },
    Assign {
        identifier: String,
        value: Expr
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>
    }, 
    While {
        condition: Expr,
        body: Vec<Stmt>
    },
    Print(Expr),
    FunctionDef {
        name: String,
        return_type: Type,
        args: Vec<(Type, String)>,
        block: Vec<Stmt>
    },
    Return(Expr)
}

#[derive(Debug, Clone)]
pub enum Type {
    Int
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i32),
    Variable(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    }
}

pub type Program = Vec<Stmt>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), ParserError> {

        let token = &self.tokens[self.current];

        if token.kind == expected {
            self.current += 1;
            return Ok(())
        }
        Err(ParserError(format!("Expected '{}', found '{}' at line {}, col {}", expected, token.kind, token.line+1, token.col+1)))
    }

    fn expect_identifer(&mut self) -> Result<String, ParserError> {
        let token = &self.tokens[self.current];

        match &token.kind {
            TokenKind::Identifier(identifier) => {
                self.current += 1;
                Ok(identifier.clone())
            }
            _ => {
                return Err(ParserError(format!("Expected Identifier, found '{}' at line {}, col {}", token.kind, token.line+1, token.col+1)));
            }
        }
    }

    fn expect_int(&mut self) -> Result<i32, ParserError> {
        let token = &self.tokens[self.current];
        
        match &token.kind {
            TokenKind::IntLiteral(val) => {
                self.current += 1;
                Ok(*val)
            }
            _ => {
                return Err(ParserError(format!("Expected int Literal, found '{}' at line {}, col {}", token.kind, token.line+1, token.col+1)));
            }
        }
    }

    fn _expect_string(&mut self) -> Result<String, ParserError> {
        let token = &self.tokens[self.current];

        match &token.kind {
            TokenKind::StringLiteral(val) => {
                self.current += 1;
                Ok(val.clone())
            }
            _ => {
                return Err(ParserError(format!("Expected string Literal, found '{}' at line {}, col {}", token.kind, token.line+1, token.col+1)));
            }
        }
    }

    fn parse_function_call_args(&mut self) -> Result<Vec<Expr>, ParserError> {

        let mut args = Vec::new();

        while self.peek().kind != TokenKind::RPar {
            args.push(self.parse_expr()?);

            if self.peek().kind != TokenKind::RPar {
                self.expect(TokenKind::Comma)?;
            }
        }

        Ok(args)
    }

    fn parse_function_definition_args(&mut self) -> Result<Vec<(Type, String)>, ParserError> {

        let mut args = Vec::new();

        while self.peek().kind != TokenKind::RPar {
            self.expect(TokenKind::Int)?;
            let ident = self.expect_identifer()?;

            args.push((Type::Int, ident));

            if self.peek().kind != TokenKind::RPar {
                self.expect(TokenKind::Comma)?;
            }
        }

        Ok(args)
    }

    fn parse_term(&mut self) -> Result<Expr, ParserError> {
        let token = self.peek();

        match token.kind {
                TokenKind::IntLiteral(_) => {
                    let val = self.expect_int()?;
                    Ok(Expr::Int(val))
                },
                TokenKind::Identifier(_) => {
                    let ident = self.expect_identifer()?;

                    match self.peek().kind {
                        TokenKind::LPar => {
                            self.expect(TokenKind::LPar)?;
                            let args = self.parse_function_call_args()?;
                            self.expect(TokenKind::RPar)?;

                            Ok(Expr::FunctionCall { name: ident, args })
                        },
                        _ => {
                            Ok(Expr::Variable(ident))
                        }
                    }
                },
                TokenKind::LPar => {
                    self.expect(TokenKind::LPar)?;

                    let inner = self.parse_expr()?;

                    self.expect(TokenKind::RPar)?;

                    Ok(inner)
                },
                _ => {
                    return Err(ParserError(format!("Expected Literal or Variable, found {} at line {}, col {}", token.kind, token.line+1, token.col+1)));
                }
            }
    }

    fn parse_decl(&mut self) -> Result<Stmt, ParserError> {
        match self.peek().kind {
            TokenKind::Int => {
                self.expect(TokenKind::Int)?;
                let identifier = self.expect_identifer()?;

                match self.peek().kind {
                    TokenKind::Assign => {
                        self.expect(TokenKind::Assign)?;
                        let init = self.parse_expr()?;

                        self.expect(TokenKind::Semi)?;

                        return Ok(Stmt::Decl { var_type: Type::Int, identifier: identifier.to_string(), init: Some(init) });
                    }
                    TokenKind::LPar => {
                        self.expect(TokenKind::LPar)?;
                        let args = self.parse_function_definition_args()?;
                        self.expect(TokenKind::RPar)?;

                        let block = self.parse_block()?;

                        return Ok(Stmt::FunctionDef { name: identifier, return_type: Type::Int, args, block })
                    },
                    TokenKind::Semi => {
                        self.expect(TokenKind::Semi)?;
                        return Ok(Stmt::Decl { var_type: Type::Int, identifier: identifier.to_string(), init: None});
                    }
                    _ => {
                        let token = self.peek();
                        return Err(ParserError(format!("Invalid Declaration at line {}, col {}", token.line+1, token.col+1)));
                    }
                }
            },
            _ => {
                let token = self.peek();
                return Err(ParserError(format!("Expected Type Definiton at line {}, col {}", token.line+1, token.col+1)));
            }
        }
    }

    fn parse_assign(&mut self) -> Result<Stmt, ParserError> {
        let identifier = self.expect_identifer()?;

        self.expect(TokenKind::Assign)?;

        let value = self.parse_expr()?;

        self.expect(TokenKind::Semi)?;

        Ok(Stmt::Assign { identifier, value })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParserError> {
        self.expect(TokenKind::While)?;
        self.expect(TokenKind::LPar)?;

        let condition = self.parse_expr()?;

        self.expect(TokenKind::RPar)?;

        let body = self.parse_block()?;

        Ok(Stmt::While { condition, body })
    }

    fn parse_if(&mut self) -> Result<Stmt, ParserError> {
        self.expect(TokenKind::If)?;
        self.expect(TokenKind::LPar)?;

        let condition = self.parse_expr()?;

        self.expect(TokenKind::RPar)?;
        
        let then_branch = self.parse_block()?;

        if self.peek().kind == TokenKind::Else {
            self.expect(TokenKind::Else)?;
            
            let else_branch = self.parse_block()?;

            return Ok(Stmt::If { condition, then_branch, else_branch: Some(else_branch) })
        } 
        else {
            return  Ok(Stmt::If { condition, then_branch, else_branch: None });
        }
    }

    fn parse_print(&mut self) -> Result<Stmt, ParserError> {
        self.expect(TokenKind::Print)?;
        self.expect(TokenKind::LPar)?;

        let value = self.parse_expr()?;

        self.expect(TokenKind::RPar)?;
        self.expect(TokenKind::Semi)?;

        Ok(Stmt::Print(value))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {

        let mut tree = self.parse_summand()?;

        loop {
            match self.peek().kind {
                TokenKind::Equal => {
                    self.expect(TokenKind::Equal)?;
                    let rhs = self.parse_summand()?;

                    tree = Expr::Binary { op: BinaryOp::Equal, left: Box::new(tree), right: Box::new(rhs) }
                },
                TokenKind::NotEqual => {
                    self.expect(TokenKind::NotEqual)?;
                    let rhs = self.parse_summand()?;

                    tree = Expr::Binary { op: BinaryOp::NotEqaul, left: Box::new(tree), right: Box::new(rhs) }
                },
                TokenKind::LessThan => {
                    self.expect(TokenKind::LessThan)?;
                    let rhs = self.parse_summand()?;

                    tree = Expr::Binary { op: BinaryOp::LessThan, left: Box::new(tree), right: Box::new(rhs) }
                },
                TokenKind::LessThanEqual => {
                    self.expect(TokenKind::LessThanEqual)?;
                    let rhs = self.parse_summand()?;

                    tree = Expr::Binary { op: BinaryOp::LessThanEqual, left: Box::new(tree), right: Box::new(rhs) }
                },
                TokenKind::GreaterThan => {
                    self.expect(TokenKind::GreaterThan)?;
                    let rhs = self.parse_summand()?;

                    tree = Expr::Binary { op: BinaryOp::GreaterThan, left: Box::new(tree), right: Box::new(rhs) }
                },
                TokenKind::GreaterThanEqual => {
                    self.expect(TokenKind::GreaterThanEqual)?;
                    let rhs = self.parse_summand()?;

                    tree = Expr::Binary { op: BinaryOp::GreaterThanEqual, left: Box::new(tree), right: Box::new(rhs) }
                },
                _ => {
                    return Ok(tree);
                },
            }
        }
    }

    fn parse_summand(&mut self) -> Result<Expr, ParserError> {

        let mut tree = self.parse_factor()?;

        loop {
            match self.peek().kind {
                TokenKind::Plus => {
                    self.expect(TokenKind::Plus)?;
                    let rhs = self.parse_factor()?;

                    tree = Expr::Binary { op: BinaryOp::Add, left: Box::new(tree), right: Box::new(rhs) };
                },
                TokenKind::Minus => {
                    self.expect(TokenKind::Minus)?;
                    let rhs = self.parse_factor()?;

                    tree = Expr::Binary { op: BinaryOp::Sub, left: Box::new(tree), right: Box::new(rhs) };
                },
                _ => {
                    return Ok(tree);
                }
            }
        }
    }

    fn parse_factor(&mut self) -> Result<Expr, ParserError> {
        let mut tree = self.parse_term()?;

        loop {
            match self.peek().kind {
                TokenKind::Mul => {
                    self.expect(TokenKind::Mul)?;
                    let rhs = self.parse_term()?;

                    tree = Expr::Binary { op: BinaryOp::Mul, left: Box::new(tree), right: Box::new(rhs) };
                },
                TokenKind::Div => {
                    self.expect(TokenKind::Div)?;
                    let rhs = self.parse_term()?;

                    tree = Expr::Binary { op: BinaryOp::Div, left: Box::new(tree), right: Box::new(rhs) };
                },
                TokenKind::Modulo => {
                    self.expect(TokenKind::Modulo)?;
                    let rhs = self.parse_term()?;

                    tree = Expr::Binary { op: BinaryOp::Modulo, left: Box::new(tree), right: Box::new(rhs) };

                }
                _ => {
                    return Ok(tree);
                },
            }
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        self.expect(TokenKind::LCurl)?;

        let mut statements = Vec::new();

        while self.peek().kind != TokenKind::RCurl {
            statements.push(self.parse_stmt()?);
        }

        self.expect(TokenKind::RCurl)?;

        Ok(statements)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParserError> {
        match self.peek().kind {
            TokenKind::Int => {
                self.parse_decl()
            },
            TokenKind::If => {
                self.parse_if()
            },
            TokenKind::While => {
                self.parse_while()
            },
            TokenKind::Identifier(_) => {
                self.parse_assign()
            }
            TokenKind::Print => {
                self.parse_print()
            },
            TokenKind::Return => {
                self.expect(TokenKind::Return)?;
                let expr =  self.parse_expr()?;
                self.expect(TokenKind::Semi)?;

                Ok(Stmt::Return(expr))
            }
            _ => {
                let token = self.peek();
                Err(ParserError(format!("Expected Statement, found {} at line {}, col {}", token.kind, token.line+1, token.col+1)))
            }
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {

        let mut program: Program = Vec::new();

        while self.peek().kind != TokenKind::EOF {
            program.push(self.parse_stmt()?);
        }

        Ok(program)
    }
}