use colored::Colorize;
use hashbrown::HashMap;
use std::string::String;
fn is_digit(ch: char) -> bool {
    ch as u8 >= b'0' && ch as u8 <= b'9'
}
fn is_alpha(ch: char) -> bool {
    let uch = ch as u8;
    (uch >= b'a' && uch <= b'z') || (uch >= b'A' && uch <= b'Z' ) || (ch == '_')
}
fn is_alpha_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}
fn get_keywords_hashmap() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("and", And),
        ("else", Else),
        ("false", False),
        ("for", For),
        ("fn", Fun),
        ("if", If),
        ("null", Nil),
        ("or", Or),
        ("nor", Nor),
        ("xor", Xor),
        ("print", Print),
        ("in", Input),
        ("panic", Errors),
        ("include", Import),
        ("exit", Exits),
        ("return", Return),
        ("true", True),
        ("let", Var),
        ("while", While),
        ("bench", Bench),
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
    pub fn scan_tokens(mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => return Err(msg.red().to_string()),
            }
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });
        Ok(self.tokens)
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '%' => self.add_token(Percent),
            '$' => self.add_token(Dollar),
            ':' => {
                let token = if self.chars_match("sin") {
                    Sin
                } else if self.chars_match("cos") {
                    Cos
                } else if self.chars_match("asin") {
                    ASin
                } else if self.chars_match("acos") {
                    ACos
                } else if self.chars_match("atan") {
                    ATan
                } else if self.chars_match("tan") {
                    Tan
                } else if self.chars_match("rnd") {
                    Round
                } else if self.chars_match("flr") {
                    Floor
                } else if self.chars_match("todeg") {
                    ToDeg
                } else if self.chars_match("torad") {
                    ToRad
                } else if self.chars_match("in") {
                    In
                } else if self.chars_match("par") {
                    Parse
                } else if self.chars_match("num") {
                    Num
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
            '.' => {
                let tkns: TokenType =
                    if self.char_match('s') && self.char_match('i') && self.char_match('n') {
                        Sin
                    } else {
                        Dot
                    };
                self.add_token(tkns)
            }
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
                    return Err(format!(
                        "Error 119: Unrecognized char at line {}: {}",
                        self.line, c
                    )
                    .red()
                    .to_string());
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
    fn number(&mut self) -> Result<(), String> {
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
            Err(_) => {
                return Err(format!("Error 120: Could not parse number: {}", substring)
                    .red()
                    .to_string())
            }
        }
        Ok(())
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }
    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err("Error 120: Unterminated string"
                .to_string()
                .red()
                  .to_string());
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        if &self.source[self.start + 1..self.start + 2] == "{"
            && &self.source[self.current - 2..self.current - 1] == "}"
        {
            let vector = &self.source[self.start + 2..self.current - 2];
            let parts = vector.split(',');
            for part in parts {
                println!("{}", part);
            }
        }
        self.add_token_lit(StringLit, Some(StringValue(value.to_string())));
        Ok(())
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }
    fn char_match(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.source.chars().nth(self.current).unwrap() != ch {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }
    fn chars_match(&mut self, chars: &str) -> bool {
        if self.is_at_end() {
            return false;
        } else if !chars.contains(self.source.chars().nth(self.current).unwrap()) {
            return false;
        } else {
            self.current += chars.len();
            return true;
        }
    }
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }
    fn add_token_lit(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line_number: self.line,
        });
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    DoubleComma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Power,
    Root,
    Random,
    Percent,
    Dollar,
    Sin,
    Cos,
    Tan,
    ASin,
    ACos,
    ATan,
    Round,
    Floor,
    Parse,
    In,
    Num,
    ToDeg,
    ToRad,
    Cube,
    CubicRoot,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Increment,
    Decrement,
    PlusEqual,
    MinusEqual,
    Pipe,
    Gets,
    Identifier,
    StringLit,
    Number,
    And,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Nor,
    Xor,
    Print,
    Input,
    Errors,
    Return,
    True,
    Var,
    While,
    Bench,
    Eof,
    Import,
    Exits,
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
    pub fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
