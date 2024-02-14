/*

    Tron Scanner

    - Scanner/Lexer

*/
use crate::panic;
use std::collections::HashMap;
use std::string::String;
// function to check if character is digital
fn is_digit(ch: char) -> bool {
    ch as u8 >= b'0' && ch as u8 <= b'9'
}
// function to check if character is alphabetical (or underscore)
fn is_alpha(ch: char) -> bool {
    let uch = ch as u8;
    (uch >= b'a' && uch <= b'z') || (uch >= b'A' && uch <= b'Z') || (ch == '_')
}
// function to check if character is either alphabetical or digital
fn is_alpha_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}
// list of keywords
fn get_keywords_hashmap() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        // start
        ("do", Start),
        ("doing", Start),
        ("start", Start),
        ("begin", Start),
        ("then", Start),
        // end
        ("end", End),
        ("done", End),
        ("stop", End),
        // and
        ("and", And),
        // else
        ("else", Else),
        ("otherwise", Else),
        ("if nor", Else),
        // false
        ("false", False),
        ("falsy", False),
        ("negative", False),
        ("no", False),
        // for
        ("for", For),
        // functions
        ("fn", Fun),
        ("fun", Fun),
        ("function", Fun),
        ("def", Fun),
        ("define", Fun),
        // if
        ("if", If),
        // null
        ("null", Nil),
        ("nil", Nil),
        // or
        ("or", Or),
        // nor
        ("nor", Nor),
        // xor
        ("xor", Xor),
        // print
        ("print", Print),
        ("say", Print),
        ("shout", Print),
        ("log", Print),
        ("out", Print),
        ("output", Print),
        ("tell", Print),
        // panic
        ("panic", Errors),
        ("alarm", Errors),
        ("throw", Errors),
        ("error", Errors),
        ("err", Errors),
        // include
        ("include", Import),
        ("import", Import),
        ("require", Import),
        ("use", Import),
        ("payload", Import),
        ("unload", Import),
        ("lib", Import),
        // exit
        ("exit", Exits),
        ("kill", Exits),
        ("terminate", Exits),
        // return
        ("return", Return),
        // true
        ("true", True),
        ("affirmative", True),
        ("yes", True),
        // variable
        ("let", Var),
        ("var", Var),
        ("const", Var),
        ("declare", Var),
        // while
        ("while", While),
        ("loop", While),
        // bench
        ("bench", Bench),
        ("test", Bench),
        ("measure", Bench),
        ("time", Bench),
        // elif
        ("else if", Elif),
        ("elif", Elif),
        // break
        ("break", Break),
        // plus
        ("plus", Plus),
        // minus
        ("minus", Minus),
        // mutliplication
        ("multiply", Star),
        ("multiplied by", Star),
        ("times", Star),
        // divide
        ("divide", Slash),
        ("divided by", Slash),
        ("slash", Slash),
        // increase
        ("increase", Increment),
        // decrease
        ("decrease", Decrement),
        // equal, assign
        ("equal", Equal),
        ("equals", Equal),
        ("assign", Equal),
        ("is", Equal),
        ("are", Equal),
        ("assigned to", Equal),
        ("assign", Equal),
        ("as", Equal),
        // more, morethan
        ("more", Greater),
        ("more than", Greater),
        // less, lessthan
        ("less", Less),
        ("less than", Less),
        // wait
        ("wait", Wait),
        ("hold", Wait),
        ("pause", Wait),
        // before
        ("before", Before),
        ("until", Before),
        ("during", Before),
    ])
}
/*

Scanner struct

source: input
tokens: list of tokens
start: starting point
current: current point
line: line
keywords: list of keywords from hashmap

*/
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}
// Scanner implementation
impl Scanner {
    // Scanner input
    // you see examples in main.rs and interpreter.rs
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
    // as you might have guessed, it scans tokens and returns vector of tokens
    pub fn scan_tokens(mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => panic(&format!("Scanner Error: {}", msg.to_string())),
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
    // Function checks if token is at end
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    // Token scanner, contains list of characters
    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(Start),
            '}' => self.add_token(End),
            '[' => self.add_token(LeftBracket),
            ']' => self.add_token(RightBracket),
            ',' => self.add_token(Comma),
            '%' => self.add_token(Percent),
            '$' => self.add_token(Var),
            ':' => self.add_token(Colon),
            '.' => self.add_token(Dot),
            '&' => {
                let token = if self.char_match('&') { And } else { Root2 };
                self.add_token(token);
            }
            '|' => {
                let token = if self.char_match('|') { Or } else { Or };
                self.add_token(token);
            }
            '?' => {
                let token = if self.char_match('>') {
                    Else
                } else if self.char_match('?') {
                    Elif
                } else {
                    If
                };
                self.add_token(token);
            }
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
            '*' => {
                let token = if self.char_match('*') { Power2 } else { Star };
                self.add_token(token)
            }
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
                    Return
                } else {
                    Equal
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.char_match('=') {
                    LessEqual
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
            '\n' => {
                self.line += 1;
            }
            '"' => self.string()?,
            '\'' => self.string()?,
            c => {
                if is_digit(c) {
                    self.number()?;
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    panic(&format!(
                        "\n Unrecognized char at line {}: {}",
                        self.line, c
                    ));
                }
            }
        }
        Ok(())
    }
    // Function checks if something is alphanumeric, if itsn't returns Identifier
    // Function parameters are good example
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
    // Number parser, recognizes floating point numbers
fn number(&mut self) -> Result<(), String> {
    while is_digit(self.peek()) {
        self.advance();
    }
    let is_float = if self.peek() == '.' && is_digit(self.peek_next()) {
        self.advance();
        while is_digit(self.peek()) {
            self.advance();
        }
        true
    } else {
        false
    };

    let substring = &self.source[self.start..self.current];
    let value = substring.parse::<f32>();
    match value {
        Ok(value) => {
            if is_float {
                self.add_token_lit(Number, Some(FValue(value)));
            } else {
                // Check if the parsed value is an integer
                if value.fract() ==   0.0 {
                    // Convert the f32 to i32 and create an Integer token
                    let int_value = value as i32;
                    self.add_token_lit(Integer, Some(IntegerValue(int_value)));
                } else {
                    // If it's not an integer, create a Number token
                    self.add_token_lit(Number, Some(FValue(value)));
                }
            }
        },
        Err(_) => panic(&format!(
            "\n Scanner Error: Could not parse number({})",
            substring
        )),
    }
    Ok(())
}
    // See next character
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }
    // String parser
    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            panic("\n Scanner Error: unterminated string");
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_lit(StringLit, Some(StringValue(value.to_string())));
        Ok(())
    }
    // This function is used for looking at the character at current position without advancing
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }
    // Checks self matchers character
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
    // Tells scanner to read next character
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }
    // adds token
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }
    // used to give tokens literals
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
// list of Tron token types
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Wait,
    Before,
    Colon,
    Start,
    End,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
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
    Power2,
    Root2,
    Increment,
    Decrement,
    PlusEqual,
    MinusEqual,
    Pipe,
    Identifier,
    StringLit,
    Number,
    Integer,
    And,
    Else,
    False,
    Fun,
    For,
    If,
    Elif,
    Nil,
    Or,
    Nor,
    Xor,
    Print,
    Errors,
    Return,
    True,
    Var,
    While,
    Bench,
    Eof,
    Import,
    Exits,
    Break,
}
use TokenType::*;
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// struct of Array
#[derive(Debug, Clone)]
pub struct ArrayElement {
    #[allow(dead_code)]
    token: Token,
}
// struct of LiteralValue
#[derive(Debug, Clone)]
pub enum LiteralValue {
    FValue(f32),
    IntegerValue(i32),
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
