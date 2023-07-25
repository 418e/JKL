use std::collections::HashMap;
use std::string::String;

// defining digital characters
fn is_digit(ch: char) -> bool {
    ch as u8 >= '0' as u8 && ch as u8 <= '9' as u8
}
// defininig alphabetical charachters
fn is_alpha(ch: char) -> bool {
    let uch = ch as u8;
    (uch >= 'a' as u8 && uch <= 'z' as u8) || (uch >= 'A' as u8 && uch <= 'Z' as u8) || (ch == '_')
}
// defining alphanumeric characters
fn is_alpha_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}
// keywords
fn get_keywords_hashmap() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("and", And),
        ("class", Class),
        ("else", Else),
        ("false", False),
        ("for", For),
        ("fn", Fun),
        ("if", If),
        ("?", IfShort),
        ("nil", Nil),
        ("or", Or),
        ("print", Print),
        ("input", Input),
        ("error", Errors),
        ("return", Return),
        ("super", Super),
        ("this", This),
        ("true", True),
        ("let", Var),
        ("while", While),
    ])
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<&'static str, TokenType>,
}
// scanner
impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keywords_hashmap(),
        }
    }
    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        // Error handler
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }
        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if errors.len() > 0 {
            let mut joined = "".to_string();
            for error in errors {
                joined.push_str(&error);
                joined.push_str("\n");
            }
            return Err(joined);
        }

        Ok(self.tokens.clone())
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }
    // scanning tokens
    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c = self.advance();
        // you can only use characters below for coding in .tron
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '%' => self.add_token(Percent),
            ':' => {
                let token = if self.char_match('s')
                    && self.char_match('i')
                    && self.char_match('n')
                    && self.char_match('_')
                {
                    Sin
                } else if self.char_match('i') && self.char_match('n') && self.char_match('_') {
                    In
                } else if self.char_match('c')
                    && self.char_match('o')
                    && self.char_match('s')
                    && self.char_match('_')
                {
                    Cos
                }else if self.char_match('l')
                    && self.char_match('o')
                    && self.char_match('g')
                    && self.char_match(':')
                {
                Log
                }else if self.char_match('l')
                    && self.char_match('o')
                    && self.char_match('g')
                    && self.char_match('2')
                    && self.char_match('_')
                {
                    Log2
                }else if self.char_match('l')
                    && self.char_match('o')
                    && self.char_match('g')
                    && self.char_match('1')
                    && self.char_match('0')
                    && self.char_match('_')
                {
                    Log10
                }else if self.char_match('l')
                    && self.char_match('n')
                    && self.char_match('_')
                {
                    Ln
                }else if self.char_match('a')
                    && self.char_match('s')
                    && self.char_match('i')
                    && self.char_match('n')
                    && self.char_match('_')
                {
                    ASin
                }else if self.char_match('a')
                    && self.char_match('s')
                    && self.char_match('i')
                    && self.char_match('n')
                    && self.char_match('h')
                    && self.char_match('_')
                {
                    ASinH
                }else if self.char_match('s')
                    && self.char_match('i')
                    && self.char_match('n')
                    && self.char_match('h')
                    && self.char_match('_')
                {
                    SinH
                }else if self.char_match('a')
                    && self.char_match('c')
                    && self.char_match('o')
                    && self.char_match('s')
                    && self.char_match('_')
                {
                    ACos
                }else if self.char_match('a')
                    && self.char_match('c')
                    && self.char_match('o')
                    && self.char_match('s')
                    && self.char_match('h')
                    && self.char_match('_')
                {
                    ACosH
                }else if self.char_match('c')
                    && self.char_match('o')
                    && self.char_match('s')
                    && self.char_match('h')
                    && self.char_match('_')
                {
                    CosH
                }else if self.char_match('a')
                    && self.char_match('t')
                    && self.char_match('a')
                    && self.char_match('n')
                    && self.char_match('_')
                {
                    ATan
                }else if self.char_match('a')
                    && self.char_match('t')
                    && self.char_match('a')
                    && self.char_match('n')
                    && self.char_match('h')
                    && self.char_match('_')
                {
                    ATanH
                }else if self.char_match('t')
                    && self.char_match('a')
                    && self.char_match('n')
                    && self.char_match('h')
                    && self.char_match('_')
                {
                    TanH
                }else if self.char_match('h')
                    && self.char_match('y')
                    && self.char_match('p')
                    && self.char_match('o')
                    && self.char_match('t')
                    && self.char_match(':')
                {
                    Hypot
                } else if self.char_match('t')
                    && self.char_match('a')
                    && self.char_match('n')
                    && self.char_match('_')
                {
                    Tan
                } else if self.char_match('r')
                    && self.char_match('n')
                    && self.char_match('d')
                    && self.char_match('_')
                {
                    Round
                } else if self.char_match('f')
                    && self.char_match('l')
                    && self.char_match('r')
                    && self.char_match('_')
                {
                    Floor
                } else if self.char_match('t')
                    && self.char_match('o')
                    && self.char_match('d')
                    && self.char_match('e')
                    && self.char_match('g')
                    && self.char_match('_')
                {
                    ToDeg
                } else if self.char_match('t')
                    && self.char_match('o')
                    && self.char_match('r')
                    && self.char_match('a')
                    && self.char_match('d')
                    && self.char_match('_')
                {
                    ToRad
                } else {
                    DoubleComma
                };
                self.add_token(token)
            }
            '^' => {
                let token = if self.char_match('^') { Cube } else { Power };
                self.add_token(token)
            }
            '&' => {
                let token = if self.char_match('&') {
                    CubicRoot
                } else {
                    Root
                };
                self.add_token(token)
            }
            '.' => self.add_token(Dot),
            '@' => self.add_token(Random),
            '-' => {
                let token = if self.char_match('-') {
                    Decrement
                } else if self.char_match('=') {
                    MinusEqual
                } else {
                    Minus
                };
                self.add_token(token);
            }
            '+' => {
                let token = if self.char_match('+') {
                    Increment
                } else if self.char_match('=') {
                    PlusEqual
                } else {
                    Plus
                };
                self.add_token(token);
            }
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let token = if self.char_match('=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.char_match('=') {
                    EqualEqual
                } else if self.char_match('>') {
                    Pipe
                } else {
                    Equal
                };

                self.add_token(token);
            }
            '<' => {
                let token = if self.char_match('=') {
                    LessEqual
                } else if self.char_match('-') {
                    Gets
                } else {
                    Less
                };

                self.add_token(token);
            }
            '>' => {
                let token = if self.char_match('=') {
                    GreaterEqual
                } else {
                    Greater
                };

                self.add_token(token);
            }
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            '\'' => self.string()?,

            c => {
                if is_digit(c) {
                    self.number()?;
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(format!("Unrecognized char at line {}: {}", self.line, c));
                }
            }
        }

        Ok(())
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let substring = &self.source[self.start..self.current];
        if let Some(&t_type) = self.keywords.get(substring) {
            self.add_token(t_type);
        } else {
            self.add_token(Identifier);
        }
    }

    fn number(self: &mut Self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let substring = &self.source[self.start..self.current];
        let value = substring.parse::<f64>();
        match value {
            Ok(value) => self.add_token_lit(Number, Some(FValue(value))),
            Err(_) => return Err(format!("Could not parse number: {}", substring)),
        }

        Ok(())
    }
    // character next to the self (right)
    fn peek_next(self: &Self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }
    fn string(self: &mut Self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_lit(StringLit, Some(StringValue(value.to_string())));
        Ok(())
    }
    // character next to the self (left)
    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }
    fn char_match(self: &mut Self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != ch {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        c
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = self.source[self.start..self.current].to_string();

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line_number: self.line,
        });
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]

// define token types here first before implementing above
pub enum TokenType {
    // Single-char tokens
    LeftParen,   // (
    RightParen,  // )
    LeftBrace,   // {
    RightBrace,  // }
    Comma,       // ,
    DoubleComma, // :
    Dot,         // .
    Minus,       // -
    Plus,        // +
    Semicolon,   // ;
    Slash,       // /
    Star,        // *
    Power,       // ^
    Root,        // &
    Random,      // @
    Percent,     // %

    // One Or Two Chars
    Sin,          // :sin_
    Cos,          // :cos_
    Tan,          // :tan_
    Round,        // :rnd_
    Floor,        // :flr_
    In,           // :in_
    ToDeg,        // :todeg_
    ToRad,        // :torad_
    Log,          // :log:
    Log2,         // :log2_
    Log10,        // :log10_
    Ln,           //:ln_
    ASin,         //:asin_
    SinH,         //:sinh_
    ASinH,        //:asinh_
    ACos,         // :acos_
    CosH,         // :cosh_
    ACosH,        // :acosh_
    ATan,         // :atan_
    TanH,         // :tanh_
    ATanH,        // :atanh_
    Hypot,        // :hypot:
    Cube,         // ^^
    CubicRoot,    // &&
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=
    Increment,    // ++
    Decrement,    // --
    PlusEqual,    // -=
    MinusEqual,   // +=
    Pipe,         // =>
    Gets,         // <-

    // Literals
    Identifier,
    StringLit,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    IfShort,
    Nil,
    Or,
    Print,
    Input,
    Errors,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}
use TokenType::*;
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, Clone)]
pub enum LiteralValue {
    FValue(f64),
    StringValue(String),
}
use LiteralValue::*;
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize,
}
impl Token {
    pub fn to_string(self: &Self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
