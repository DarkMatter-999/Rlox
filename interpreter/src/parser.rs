use crate::{
    expr::{Binary, BinaryOperator, Expr, ExprType, Literal, Unary, UnaryOperator},
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

    pub fn parse(&mut self) -> Expr {
        return self.expression();
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }
    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparision();

        while self.match_tok(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = match self.previous().token_type {
                TokenType::BANG_EQUAL => BinaryOperator::BangEq,
                TokenType::EQUAL_EQUAL => BinaryOperator::Equal,
                _ => panic!("Invalid token at {}", self.current),
            };
            let right: Expr = self.comparision();

            expr = Expr {
                node: ExprType::binary(expr, operator, right),
            };
        }

        return expr;
    }
    fn comparision(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.match_tok(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = match self.previous().token_type {
                TokenType::GREATER => BinaryOperator::GreaterThan,
                TokenType::GREATER_EQUAL => BinaryOperator::GreaterThanEq,
                TokenType::LESS => BinaryOperator::LessThan,
                TokenType::LESS_EQUAL => BinaryOperator::LessThanEq,
                _ => panic!("Invalid token at {}", self.current),
            };
            let right: Expr = self.comparision();

            expr = Expr {
                node: ExprType::binary(expr, operator, right),
            };
        }

        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.match_tok(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = match self.previous().token_type {
                TokenType::MINUS => BinaryOperator::Minus,
                TokenType::PLUS => BinaryOperator::Plus,
                _ => panic!("Invalid token at {}", self.current),
            };
            let right: Expr = self.factor();

            expr = Expr {
                node: ExprType::binary(expr, operator, right),
            };
        }

        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.match_tok(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = match self.previous().token_type {
                TokenType::SLASH => BinaryOperator::Slash,
                TokenType::STAR => BinaryOperator::Star,
                _ => panic!("Invalid token at {}", self.current),
            };
            let right: Expr = self.unary();

            expr = Expr {
                node: ExprType::binary(expr, operator, right),
            };
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.match_tok(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = match self.previous().token_type {
                TokenType::BANG => UnaryOperator::Bang,
                TokenType::MINUS => UnaryOperator::Minus,
                _ => panic!("Invalid token at {}", self.current),
            };
            let right: Expr = self.unary();

            let expr = Expr {
                node: ExprType::unary(operator, right),
            };

            return expr;
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        match self.peek().token_type {
            TokenType::FALSE => {
                self.advance();
                return Expr {
                    node: ExprType::Literal(Literal::False),
                };
            }
            TokenType::TRUE => {
                self.advance();
                return Expr {
                    node: ExprType::Literal(Literal::True),
                };
            }
            TokenType::NIL => {
                self.advance();
                return Expr {
                    node: ExprType::Literal(Literal::None),
                };
            }
            TokenType::NUMBER => {
                self.advance();
                let n = match self.previous().lexeme {
                    token::Literal::Number(n) => n,
                    _ => panic!("Number not recieved {:?}", self.previous()),
                };
                return Expr {
                    node: ExprType::Literal(Literal::Number(n)),
                };
            }
            TokenType::STRING => {
                self.advance();
                let s = match self.previous().lexeme {
                    token::Literal::StringLit(s) => s,
                    _ => panic!("String not recieved"),
                };
                return Expr {
                    node: ExprType::Literal(Literal::String(s)),
                };
            }
            TokenType::LEFT_PAREN => {
                self.advance();
                let expr = self.expression();
                self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression");
                return Expr {
                    node: ExprType::grouping(expr),
                };
            }
            _ => {
                self.error(self.peek(), "Expect expression");
                panic!("Error");
            }
        }
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
