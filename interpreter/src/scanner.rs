use std::collections::HashMap;

use crate::{
    parser::Parser,
    token::{Literal::*, Token, TokenType, TokenType::*},
};

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    keywords: HashMap<&'static str, TokenType>,
}

fn fill_keywords(keywords: &mut HashMap<&str, TokenType>) {
    keywords.insert("and", AND);
    keywords.insert("class", CLASS);
    keywords.insert("else", ELSE);
    keywords.insert("false", FALSE);
    keywords.insert("for", FOR);
    keywords.insert("fun", FUN);
    keywords.insert("if", IF);
    keywords.insert("nil", NIL);
    keywords.insert("or", OR);
    keywords.insert("print", PRINT);
    keywords.insert("return", RETURN);
    keywords.insert("super", SUPER);
    keywords.insert("this", THIS);
    keywords.insert("true", TRUE);
    keywords.insert("var", VAR);
    keywords.insert("while", WHILE);
    keywords.insert("break", BREAK);
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();

        fill_keywords(&mut keywords);

        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        self.tokens.clone()
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '{' => self.add_token(LEFT_BRACE),
            '}' => self.add_token(RIGHT_BRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => {
                let tok = self.match_tok('=');
                if tok {
                    self.add_token(BANG_EQUAL);
                } else {
                    self.add_token(BANG);
                }
            }
            '=' => {
                let tok = self.match_tok('=');
                if tok {
                    self.add_token(EQUAL_EQUAL);
                } else {
                    self.add_token(EQUAL);
                }
            }
            '<' => {
                let tok = self.match_tok('=');
                if tok {
                    self.add_token(LESS_EQUAL);
                } else {
                    self.add_token(LESS);
                }
            }
            '>' => {
                let tok = self.match_tok('=');
                if tok {
                    self.add_token(GREATER_EQUAL);
                } else {
                    self.add_token(GREATER);
                }
            }
            '/' => {
                if self.match_tok('/') {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            }
            '\t' | '\r' | ' ' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' => self.identifier(),
            _ => panic!("invalid token recieved '{}'", c),
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token: TokenType) {
        self.tokens.push(Token {
            token_type: token,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });
    }

    fn add_token_str(&mut self, text: String) {
        self.tokens.push(Token {
            token_type: STRING,
            lexeme: text.clone(),
            literal: StringLit(text),
            line: self.line,
        });
    }

    fn add_token_num(&mut self, n: f64) {
        self.tokens.push(Token {
            token_type: NUMBER,
            lexeme: n.to_string(),
            literal: Number(n),
            line: self.line,
        });
    }

    fn match_tok(&mut self, c: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != c {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        } else {
            return self.source.chars().nth(self.current).unwrap();
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            panic!("Unterminated string at line {}", self.line);
        }

        self.advance();

        let s = &self.source[self.start + 1..self.current - 1];
        self.add_token_str(s.to_string());
    }

    fn number(&mut self) {
        while ('0'..='9').contains(&self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && ('0'..='9').contains(&self.peek_next()) {
            self.advance();
            while ('0'..='9').contains(&self.peek()) {
                self.advance();
            }
        }

        let f = &self.source[self.start..self.current];

        let f: f64 = f.parse().unwrap();

        self.add_token_num(f);
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = self.keywords.get(text);

        match token_type {
            Some(k) => self.add_token(k.clone()),
            _ => self.add_token_identifier(text.to_string()),
        }
    }

    fn add_token_identifier(&mut self, id: String) {
        self.tokens.push(Token {
            token_type: TokenType::IDENTIFIER,
            lexeme: id,
            literal: None,
            line: self.line,
        });
    }
}

pub trait StmtIterator {
    fn statements(self) -> Parser;
}

impl StmtIterator for Scanner {
    fn statements(self) -> Parser {
        Parser::new(self.tokens)
    }
}
