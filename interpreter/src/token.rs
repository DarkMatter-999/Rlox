use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, Clone)]
pub enum Literal {
    StringLit(String),
    Number(f64),
    True,
    False,
    None,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: u32,
}

impl Token {
    pub fn in_types(&self, types: Vec<TokenType>) -> bool {
        for typ in types {
            if self.token_type == typ {
                return true;
            }
        }

        return false;
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match *self {
            Literal::None => match *other {
                Literal::None => true,
                _ => false,
            },
            Literal::True => match *other {
                Literal::True => true,
                _ => false,
            },
            Literal::False => match *other {
                Literal::False => true,
                _ => false,
            },
            Literal::Number(ref a) => match *other {
                Literal::Number(ref b) => a.eq(b),
                _ => false,
            },
            Literal::StringLit(ref a) => match *other {
                Literal::StringLit(ref b) => a.eq(b),
                _ => false,
            },
        }
    }
}

impl PartialOrd<Self> for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Literal::None, &Literal::None) => Some(Ordering::Equal),
            (&Literal::StringLit(ref l), &Literal::StringLit(ref r)) => l.partial_cmp(r),
            (&Literal::Number(ref l), &Literal::Number(ref r)) => l.partial_cmp(r),
            (&Literal::True, Literal::True) => true.partial_cmp(&true),
            (&Literal::False, Literal::False) => false.partial_cmp(&false),
            (&Literal::True, Literal::False) => true.partial_cmp(&false),
            (&Literal::False, Literal::True) => false.partial_cmp(&true),
            _ => None,
        }
    }
}
