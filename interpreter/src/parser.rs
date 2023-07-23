use crate::{
    error::{Error, ResultMSG},
    expr::{self, Boxed, Expr},
    stmt::Stmt,
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

    pub fn parse(&mut self) -> ResultMSG<Stmt> {
        return self.statement();
    }

    fn expression(&mut self) -> ResultMSG<Expr> {
        return self.assignment();
    }

    fn assignment(&mut self) -> ResultMSG<Expr> {
        let expr = self.equality()?;

        if self.match_tok(vec![TokenType::EQUAL]) {
            return match expr {
                Expr::Identifier(id) => Ok(Expr::Assignment(id, self.assignment()?.boxed())),
                _ => Err(Error::Parser(
                    self.peek().line,
                    "Unexpected Token".to_string(),
                    format!("{:?}", self.peek()),
                )),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ResultMSG<Expr> {
        let mut expr: Expr = self.comparision()?;

        while self.match_tok(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right: Expr = self.comparision()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }
    fn comparision(&mut self) -> ResultMSG<Expr> {
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

    fn term(&mut self) -> ResultMSG<Expr> {
        let mut expr: Expr = self.factor()?;

        while self.match_tok(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> ResultMSG<Expr> {
        let mut expr: Expr = self.unary()?;

        while self.match_tok(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right: Expr = self.unary()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> ResultMSG<Expr> {
        if self.match_tok(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right: Expr = self.unary()?;

            let expr = Expr::Unary(operator, Box::new(right));

            return Ok(expr);
        }

        return self.primary();
    }

    fn primary(&mut self) -> ResultMSG<Expr> {
        if self.match_tok(vec![TokenType::IDENTIFIER]) {
            return Ok(Expr::Identifier(self.previous().lexeme.clone()));
        }

        if self.match_tok(vec![TokenType::FALSE, TokenType::TRUE, TokenType::NIL]) {
            self.advance();
            return Ok(Expr::Literal(self.peek()));
        }

        if self.match_tok(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::Literal(self.previous()));
        }

        if self.match_tok(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression");
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        return Err(Error::Parser(
            self.peek().line,
            "Unexpected Token".to_string(),
            format!("{:?}", self.peek()),
        ));
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

    fn check_types(&mut self, types: &[TokenType]) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.peek() {
            t => t.in_types(types.to_vec()),
            _ => false,
        }
    }

    fn check_next(&mut self, types: &[TokenType]) -> Option<ResultMSG<Token>> {
        if self.check_types(types) {
            return Some(self.peek_next());
        }
        None
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

    fn peek_next(&self) -> ResultMSG<Token> {
        if self.is_at_end() {
            return Err(Error::Parser(
                self.peek().line,
                "End of File".to_string(),
                format!("{:?}", self.peek()),
            ));
        }
        return Ok(self.tokens[self.current + 1].clone());
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
            lexeme: "".to_string(),
            literal: token::Literal::None,
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

    fn statement(&mut self) -> ResultMSG<Stmt> {
        let v = self.match_tok(vec![
            TokenType::SEMICOLON,
            TokenType::PRINT,
            TokenType::VAR,
            TokenType::LEFT_BRACE,
            TokenType::IF,
            TokenType::WHILE,
            TokenType::FOR,
            TokenType::BREAK,
        ]);

        let mut n: Option<ResultMSG<Token>> = None;

        if v {
            n = Some(Ok(self.previous()));
        }

        if n.is_none() {
            return self.expr_statement();
        }

        let token = n.unwrap()?;

        match token.token_type {
            TokenType::SEMICOLON => Ok(Stmt::Empty),
            TokenType::PRINT => self.print_statement(),
            TokenType::VAR => self.declaration_statement(),
            TokenType::LEFT_BRACE => self.block_statement(),
            TokenType::IF => self.if_statement(),
            TokenType::WHILE => self.while_statement(),
            TokenType::FOR => self.for_statement(),
            TokenType::BREAK => self.break_statement(),
            _ => unreachable!(),
        }
    }

    fn print_statement(&mut self) -> ResultMSG<Stmt> {
        let expr: Expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        Ok(Stmt::Print(expr))
    }

    fn expr_statement(&mut self) -> ResultMSG<Stmt> {
        let expr: Expr = self.expression()?;
        Ok(Stmt::Expression(expr))
    }

    fn declaration_statement(&mut self) -> ResultMSG<Stmt> {
        let id: Token = self.consume(TokenType::IDENTIFIER, "Expect variable name.");

        if !self.match_tok(vec![TokenType::EQUAL]) {
            return Ok(Stmt::Declaration(id.lexeme, None));
        }

        let expr: Expr = self.expression()?;

        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");

        Ok(Stmt::Declaration(id.lexeme, Some(expr)))
    }

    fn block_statement(&mut self) -> ResultMSG<Stmt> {
        let mut stmts: Vec<Stmt> = Vec::new();

        while self.check_next(&[TokenType::RIGHT_BRACE]).is_none() {
            stmts.push(self.statement()?);
        }

        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block");

        Ok(Stmt::Block(stmts))
    }

    fn if_statement(&mut self) -> ResultMSG<Stmt> {
        self.check_next(&[TokenType::LEFT_PAREN]);
        let expr: Expr = self.expression()?;
        self.check_next(&[TokenType::RIGHT_PAREN]);

        let then_stmt: Box<Stmt> = self.statement()?.boxed();

        match self.check_next(&[TokenType::ELSE]) {
            Some(Err(e)) => Err(e),
            Some(Ok(_)) => Ok(Stmt::If(expr, then_stmt, Some(self.statement()?.boxed()))),
            None => Ok(Stmt::If(expr, then_stmt, None)),
        }
    }

    fn while_statement(&mut self) -> ResultMSG<Stmt> {
        let expr: Expr = self.expression()?;
        let body: Box<Stmt> = self.statement()?.boxed();
        Ok(Stmt::While(expr, body))
    }
    fn for_statement(&mut self) -> ResultMSG<Stmt> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.");

        let init: Option<Stmt> = match self.match_tok(vec![TokenType::SEMICOLON, TokenType::VAR]) {
            true => match self.previous().token_type {
                TokenType::SEMICOLON => None,
                TokenType::VAR => {
                    let dec = self.declaration_statement()?;
                    self.consume(
                        TokenType::SEMICOLON,
                        "Expect ';' after loop initialization.",
                    );

                    Some(dec)
                }
                _ => {
                    unreachable!()
                }
            },
            _ => Some(self.expr_statement()?),
        };

        let cond: Expr = match self.match_tok(vec![TokenType::SEMICOLON]) {
            true => Expr::Literal(Token {
                token_type: TokenType::TRUE,
                lexeme: "".to_string(),
                literal: Literal::True,
                line: self.peek().line,
            }),

            false => {
                let expr = self.expression()?;
                self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.");
                expr
            }
        };

        let inc: Option<Stmt> = match self.match_tok(vec![TokenType::RIGHT_PAREN]) {
            true => None,
            false => {
                let expr = self.expr_statement()?;

                self.consume(TokenType::RIGHT_PAREN, "Expect ')' after loop.");
                Some(expr)
            }
        };

        let mut body: Stmt = self.statement()?;

        if inc.is_some() {
            body = Stmt::Block(vec![body, inc.unwrap()]);
        }

        body = Stmt::While(cond, body.boxed());

        if init.is_some() {
            body = Stmt::Block(vec![init.unwrap(), body])
        }

        Ok(body)
    }

    fn for_statement1(&mut self) -> ResultMSG<Stmt> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.");

        let init: Option<Stmt> = match self.check_next(&[TokenType::SEMICOLON, TokenType::VAR]) {
            None => Some(self.expr_statement()?),
            Some(t) => match t?.token_type {
                TokenType::VAR => Some(self.declaration_statement()?),
                TokenType::SEMICOLON => None,
                _ => unreachable!(),
            },
        };

        println!("{:?}", self.peek());
        println!("{:?}", self.peek());
        self.advance();
        println!("{:?}", self.peek());

        let cond: Expr = match self.check_next(&[TokenType::SEMICOLON]) {
            None => {
                let expr = self.expression()?;
                self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.");
                expr
            }
            Some(t) => {
                println!("{:?}", t);
                match t?.token_type {
                    TokenType::SEMICOLON => Expr::Literal(Token {
                        token_type: TokenType::TRUE,
                        lexeme: "".to_string(),
                        literal: Literal::True,
                        line: self.peek().line,
                    }),
                    e => unreachable!("{:?}", e),
                }
            }
        };

        let inc: Option<Stmt> = match self.check_next(&[TokenType::RIGHT_PAREN]) {
            None => Some(self.expr_statement()?),
            Some(t) => {
                t?;
                None
            }
        };

        let mut body: Stmt = self.statement()?;

        if inc.is_some() {
            body = Stmt::Block(vec![body, inc.unwrap()]);
        }

        body = Stmt::While(cond, body.boxed());

        if init.is_some() {
            body = Stmt::Block(vec![init.unwrap(), body])
        }

        Ok(body)
    }

    fn break_statement(&mut self) -> ResultMSG<Stmt> {
        match self.check_next(&[TokenType::SEMICOLON]) {
            Some(token) => match token {
                Ok(t) => Ok(Stmt::Break(t.line)),
                Err(e) => Err(e),
            },
            None => unreachable!(),
        }
    }
}

impl Iterator for Parser {
    type Item = ResultMSG<Stmt>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            return None;
        }

        let res = self.statement();
        if res.is_err() {
            self.synchronize();
        }

        Some(res)
    }
}
