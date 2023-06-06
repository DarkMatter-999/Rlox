use crate::token::{Token, TokenType, TokenType::*};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            line: self.line,
        });

        &self.tokens
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

            _ => panic!("invalid token recieved '{}'", c),
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token: TokenType) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token {
            token_type: token,
            lexeme: text.to_string(),
            line: self.line,
        });
    }

    fn add_token_str(&mut self, text: String) {
        self.tokens.push(Token {
            token_type: STRING,
            lexeme: text,
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
}
