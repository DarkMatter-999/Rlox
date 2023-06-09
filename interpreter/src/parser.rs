use crate::{
    expr::Expr,
    token::{self, Token, TokenType, *},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, bool> {
        return self.expression();
    }

    fn expression(&mut self) -> Result<Expr, bool> {
        return self.equality();
    }
    fn equality(&mut self) -> Result<Expr, bool> {
        let mut expr: Expr = self.comparision()?;

        while self.match_tok(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right: Expr = self.comparision()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }
    fn comparision(&mut self) -> Result<Expr, bool> {
        let mut expr: Expr = self.term()?;

        while self.match_tok(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right: Expr = self.term()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, bool> {
        let mut expr: Expr = self.factor()?;

        while self.match_tok(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, bool> {
        let mut expr: Expr = self.unary()?;

        while self.match_tok(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right: Expr = self.unary()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, bool> {
        if self.match_tok(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right: Expr = self.unary()?;

            let expr = Expr::Unary(operator, Box::new(right));

            return Ok(expr);
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, bool> {
        if self.match_tok(vec![TokenType::FALSE, TokenType::TRUE, TokenType::NIL]) {
            self.advance();
            return Ok(Expr::Literal(self.peek()));
        }

        if self.match_tok(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::Literal(self.previous()));
        }

        if self.match_tok(vec![TokenType::LEFT_PAREN]) {
            self.advance();
            let expr = self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression");
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        return Err(false);
    }

    fn match_tok(&mut self, tokens: Vec<TokenType>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn consume(&mut self, token: TokenType, message: &str) -> Token {
        if self.check(token) {
            return self.advance();
        }
        self.error(self.peek(), message);
        return Token {
            token_type: token,
            lexeme: token::Literal::None,
            line: 0,
        };
    }
    fn error(&self, token: Token, message: &str) {
        if token.token_type == TokenType::EOF {
            println!("{} at end {}", token.line, message);
        } else {
            println!("{} at '{:?}' {}", token.line, token.lexeme, message);
        }
    }
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {}
            }

            self.advance();
        }
    }
}
