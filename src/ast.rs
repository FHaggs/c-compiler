use super::token::Token;

#[derive(Debug)]
pub struct Program {
    pub function_definition: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub body: Statement,
}

#[derive(Debug)]
pub enum Identifier {
    FunctionName(String),
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        if tok.is_some() {
            self.position += 1;
        }
        tok
    }

    fn expect(&mut self, expected: &Token) -> &Token {
        let token = self.advance().unwrap();
        if token != expected {
            panic!("Expected {:?}, found {:?}", expected, token);
        }
        token
    }
    fn expect_int_literal(&mut self) -> i32 {
        match self.advance() {
            Some(Token::IntLiteral(n)) => *n,
            other => panic!("Expected integer literal, found {:?}", other),
        }
    }
    fn expect_identifier(&mut self) -> String {
        match self.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            other => panic!("Expected identifier, found {:?}", other),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let function = self.parse_function();
        Program {
            function_definition: function,
        }
    }
    fn parse_function(&mut self) -> FunctionDefinition {
        self.expect(&Token::Int);
        let name = self.expect_identifier();
        self.expect(&Token::OpenParenthesis);
        self.expect(&Token::Void);
        self.expect(&Token::ClosedParenthesis);
        self.expect(&Token::OpenBraces);
        let body = self.parse_statement();
        self.expect(&Token::ClosedBraces);
        FunctionDefinition {
            name: Identifier::FunctionName(name),
            body,
        }
    }

    fn parse_statement(&mut self) -> Statement {
        self.expect(&Token::Return);
        let return_value = self.parse_expression();
        self.expect(&Token::Semicolon);
        Statement::Return(return_value)
    }

    fn parse_expression(&mut self) -> Expression {
        let val = self.expect_int_literal();
        Expression::Constant(val)
    }
}
