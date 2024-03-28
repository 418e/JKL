use crate::expressions::Expression;
use crate::utils::TronError;
use std::collections::HashMap;
use std::string::String;

/// Returns a `HashMap` containing keywords as keys and corresponding `TokenType` values.
///
/// This function is used to initialize a mapping of keywords to their corresponding token types
/// The `HashMap` is used by the scanner during the tokenization process to identify and categorize
/// keywords in the source code.
///
/// # Return Value
///
/// A `HashMap` where each key is a string slice (`&'static str`) representing a keyword, and each
/// value is a `TokenType` enum variant that represents the type of token associated with the keyword.
///
/// # Example
///
/// ```rust
/// let keywords_map = get_keywords_hashmap();
/// assert_eq!(keywords_map.get("else"), Some(&TokenType::Else));
/// ```
/// In this example, the `get_keywords_hashmap()` function is called to obtain a `HashMap` of keywords.
/// The `HashMap` is then queried to check if the keyword "else" is correctly mapped to the `TokenType::Else`
/// variant.
///
/// # Usage
///
/// The `get_keywords_hashmap()` function is typically called once during the initialization of the scanner
/// to set up the keyword mapping. This mapping is then used throughout the scanning process to identify
/// and categorize keywords in the source code.
///
/// # Note
///
/// The `get_keywords_hashmap()` function is a crucial part of the scanner's initialization process, as
/// it sets up the foundation for correctly identifying and categorizing keywords in the language.
/// Any changes to the keywords or their associated token types should be reflected in this function.
///
/// ### Last Updated: (v3.0.0)
pub fn get_keywords_hashmap() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("else", Else),
        ("false", False),
        ("for", For),
        ("fn", Function),
        ("if", If),
        ("null", Null),
        ("nor", Nor),
        ("xor", Xor),
        ("use", Use),
        ("return", Return),
        ("true", True),
        ("let", Variable),
        ("while", While),
        ("else if", Elif),
        ("break", Break),
        ("switch", Switch),
        ("case", Case),
        ("default", Default),
    ])
}
/// Enum list of tokens (`TokenType`) used in the interpreter.
///
/// Each variant of the `TokenType` enum represents a specific type of token, such as keywords, punctuation symbols, operators, literals,
/// and other token types. This enum is used throughout the scanner and parser to categorize and handle different types of tokens
/// found in the source code.
///
/// # Usage
///
/// The `TokenType` enum is used extensively throughout the Tron language's scanner and parser to categorize and handle
/// different types of tokens found in the source code. Each variant of the enum corresponds to a specific type of token,
/// allowing the scanner and parser to correctly identify and process these tokens as part of the language's syntax.
///
/// # Note
///
/// When adding new keywords or modifying existing ones, it's important to update the `TokenType` enum and the
/// `get_keywords_hashmap()` function accordingly to ensure that the scanner can correctly identify and categorize
/// these keywords in the source code.
///
/// ### Last Updated: (v3.0.0)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    /// - `Colon`: Represents the colon symbol (`:`).
    Colon,
    /// - `LeftBrace`: Represents the left brace (`{`).
    LeftBrace,
    /// - `RightBrace`: Represents the right brace (`}`).
    RightBrace,
    /// - `LeftParen`: Represents the left parenthesis (`(`).
    LeftParen,
    /// - `RightParen`: Represents the right parenthesis (`)`).
    RightParen,
    /// - `LeftBracket`: Represents the left bracket (`[`).
    LeftBracket,
    /// - `RightBracket`: Represents the right bracket (`]`).
    RightBracket,
    /// - `Comma`: Represents the comma symbol (`,`).
    Comma,
    /// - `Dot`: Represents the dot symbol (`.`).
    Dot,
    /// - `Minus`: Represents the minus symbol (`-`).
    Minus,
    /// - `Plus`: Represents the plus symbol (`+`).
    Plus,
    /// - `Semicolon`: Represents the semicolon symbol (`;`).
    Semicolon,
    /// - `Slash`: Represents the slash symbol (`/`).
    Slash,
    /// - `Star`: Represents the star symbol (`*`).
    Star,
    /// - `Power`: Represents the power operator (`^`).
    Power,
    /// - `Percent`: Represents the percent symbol (`%`).
    Percent,
    /// - `Bang`: Represents the bang symbol (`!`).
    Bang,
    /// - `BangEqual`: Represents the bang-equal operator (`!=`).
    BangEqual,
    /// - `Equal`: Represents the equal symbol (`=`).
    Equal,
    /// - `EqualEqual`: Represents the double-equal symbol (`==`).
    EqualEqual,
    /// - `Greater`: Represents the greater-than symbol (`>`).
    Greater,
    /// - `GreaterEqual`: Represents the greater-than-or-equal-to symbol (`>=`).
    GreaterEqual,
    /// - `Less`: Represents the less-than symbol (`<`).
    Less,
    /// - `LessEqual`: Represents the less-than-or-equal-to symbol (`<=`).
    LessEqual,
    /// - `Increment`: Represents the increment operator (`++`).
    Increment,
    /// - `Decrement`: Represents the decrement operator (`--`).
    Decrement,
    /// - `Identifier`: Represents an identifier (e.g., variable names, function names).
    Identifier,
    /// - `StringLit`: Represents a string literal (`"`).
    StringLit,
    /// - `Number`: Represents a numeric literal.
    Number,
    /// - `And`: Represents the logical AND operator.
    And,
    /// - `Else`: Represents the `else` keyword.
    Else,
    /// - `False`: Represents the `false` keyword.
    False,
    /// - `Function`: Represents the `function` keyword.
    Function,
    /// - `For`: Represents the `for` keyword.
    For,
    /// - `If`: Represents the `if` keyword.
    If,
    /// - `Elif`: Represents the `else if` keyword.
    Elif,
    /// - `Null`: Represents the `null` keyword.
    Null,
    /// - `Or`: Represents the logical OR operator.
    Or,
    /// - `Nor`: Represents the logical NOR operator.
    Nor,
    /// - `Xor`: Represents the logical XOR operator.
    Xor,
    /// - `Return`: Represents the `return` keyword.
    Return,
    /// - `True`: Represents the `true` keyword.
    True,
    /// - `Variable`: Represents the `let` keyword.
    Variable,
    /// - `While`: Represents the `while` keyword.
    While,
    /// - `Eof`: Represents the end of file.
    Eof,
    /// - `Use`: Represents the `use` keyword.
    Use,
    /// - `Break`: Represents the `break` keyword.
    Break,
    /// - `Switch`: Represents the `switch` keyword.
    Switch,
    /// - `Case`: Represents the `case` keyword.
    Case,
    /// - `Default`: Represents the `default` keyword.
    Default,
    /// - `Question`: Represents the questioan mark (`?`).
    Question,
    /// - `Line`: Represents the line Symbold (`|`)
    Line,
}
use TokenType::*;
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The `LiteralValue` enum in Rust represents the literal values that can be associated with tokens.
///
/// Each variant of the `LiteralValue` enum corresponds to a specific type of literal value, such as floating-point numbers,
/// strings, or other types of values that can be directly represented in the source code. This enum is used by the scanner
/// and parser to store and handle literal values as part of the language's syntax.
///
/// # Variants
///
/// - `NumericValue(f32)`: Represents a floating-point literal value.
/// - `StringValue(String)`: Represents a string literal value.
///
/// # Usage
///
/// The `LiteralValue` enum is used to store the actual values of literals in the source code. For example, when a number or a string
/// is encountered in the source code, it is represented as a `LiteralValue` variant. This allows the scanner and parser
/// to associate the literal value with the corresponding token and use it during the interpretation or compilation process.
///
/// # Example
///
/// ```rust
/// let number_literal = LiteralValue::NumericValue(42.0);
/// let string_literal = LiteralValue::StringValue("Hello, Tron!".to_string());
/// ```
///
/// In this example, `number_literal` represents a floating-point number literal with the value `42.0`, and `string_literal`
/// represents a string literal with the value `"Hello, Tron!"`.
///
/// # Note
///
/// When adding new literal types or modifying existing ones, it's important to update the `LiteralValue` enum and the scanner's logic
/// accordingly to ensure that the scanner can correctly identify and categorize these literal values in the source code.
///
/// ### Last Updated: (v3.0.0)
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    /// Represents a floating-point literal value.
    NumericValue(f32),
    /// Represents a string literal value.
    StringValue(String),
}
use LiteralValue::*;

#[derive(Debug, Clone)]
pub enum Statement {
    /// The `ExpressionStatement` variant in the `Statement` enum represents a statement in the code that
    /// consists of a single expression. It is used to handle statements where the primary purpose is to
    /// evaluate an expression without any additional side effects or operations.
    ///
    /// # Fields
    ///
    /// - `expression`: This field holds the expression that is being evaluated or executed as part of
    ///   this statement. The type of this field is `Expression`, which can represent various types of
    ///   expressions in the Tron language, such as arithmetic operations, function calls, or variable
    ///   assignments.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// # Example
    ///
    /// ```
    /// let x = 5;
    /// print(x + 3);
    /// ```
    ///
    /// In this example, the `ExpressionStatement` evaluates the expression `x + 3`, which adds the value of
    /// the variable `x` to `3`. The result of this expression is printed.
    ///
    /// # Usage
    ///
    /// When parsing Tron code, the scanner and parser will identify expressions that do not have any
    /// side effects or operations associated with them (such as assignments or function calls) and
    /// represent them as `ExpressionStatement` variants. These statements are then evaluated by the
    /// interpreter or compiler as part of the execution or compilation process.
    ///
    /// ### Last Updated: (v3.0.0)
    ExpressionStatement { expression: Expression, line: usize },
    /// The `UseStatement` variant in the `Statement` enum represents an use statement.
    ///
    /// Use statements are used to include external modules or libraries into the current scope of the program.
    /// They allow the programmer to use functions, classes, or other code elements defined in other files or modules.
    ///
    /// # Fields
    ///
    /// - `expression`: This field holds the expression that represents the import statement. It typically
    ///   includes the path or name of the module or library to be imported.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// # Example
    ///
    /// ```
    /// use "math.tron";
    /// ```
    /// In this example, the `UseStatement` is used to use the "math.tron" file.
    ///
    /// # Usage
    ///
    /// When parsing Tron code, the scanner and parser will identify use statements and represent them as
    /// `UseStatement` variants. These statements are then processed by the interpreter or compiler to include the
    /// specified modules or libraries into the current scope.
    ///
    /// ### Last Updated: (v3.0.0)
    UseStatement { expression: Expression, line: usize },
    /// The `VariableStatement` variant in the `Statement` enum represents a variable declaration statement.
    ///
    /// Variable declaration statements are used to introduce new variables into the current scope of the program.
    /// They allow the programmer to define variables with a specified name, value, and optionally, a type.
    ///
    /// # Fields
    ///
    /// - `name`: This field holds the `Token` that represents the name of the variable being declared.
    /// - `value_type`: `Token` that represents the type of the value being declared.
    /// - `value`: This field holds the `Expression` that represents the initial value of the variable being declared.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// # Example
    ///
    /// ```
    /// let x: number = 0;
    /// ```
    ///
    /// In this example, the `VariableStatement` declares a variable named `x` with an initial value of `5`.
    ///
    /// # Usage
    ///
    /// When parsing Tron code, the scanner and parser will identify variable declaration statements and represent them as
    /// `VariableStatement` variants. These statements are then processed by the interpreter or compiler to declare the
    /// specified variables in the current scope.
    ///
    /// ### Last Updated: (v3.1.0)
    VariableStatement {
        name: Token,
        value_type: Token,
        value: Expression,
        line: usize,
    },
    /// The `BlockStatement` variant in the `Statement` enum represents a block of statements.
    ///
    /// A block statement is a sequence of statements enclosed within a pair of braces (`{}`). It is used to group multiple statements
    /// together, typically for control flow structures such as loops, conditionals, and function bodies.
    ///
    /// # Fields
    ///
    /// - `statements`: This field holds a vector of boxed `Statement` enums, which represent the sequence of statements
    ///   contained within the block. Each element in the vector is a boxed `Statement`, allowing for a dynamic list of
    ///   statements that can include any type of statement.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// # Example
    ///
    /// ```
    /// {
    /// let x = 5;
    /// let y = 10;
    /// print(x + y);
    /// }
    /// ```
    ///
    /// In this example, the `BlockStatement` contains three statements: two variable declarations and a print statement.
    ///
    /// # Usage
    ///
    /// When parsing Tron code, the scanner and parser will identify blocks of statements and represent them as
    /// `BlockStatement` variants. These blocks are then processed by the interpreter or compiler to execute the contained
    /// statements in sequence.
    ///
    /// ### Last Updated: (v3.0.0)
    BlockStatement {
        statements: Vec<Box<Statement>>,
        line: usize,
    },
    /// The `WhileStatement` variant in the `Statement` enum represents a while loop statement.
    ///
    /// A while loop statement is used to repeatedly execute a block of code as long as a given condition is true.
    /// It consists of a condition expression and a body block that contains the statements to be executed.
    ///
    /// # Fields
    ///
    /// - `conditions`: This field holds a vector of `Expression` enums, which represent the conditions that must be true for the loop to continue.
    /// - `body`: This field holds a boxed `Statement` enum, which represents the body of the loop that is executed repeatedly.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// # Example
    ///
    /// ```
    /// while x < 10 {
    ///  print(x);
    ///  x = x + 1;
    /// }
    /// ```
    ///
    /// In this example, the `WhileStatement` contains a condition `x < 10` and a body block with two statements:
    /// a print statement and an assignment statement.
    ///
    /// # Usage
    ///
    /// When parsing Tron code, the scanner and parser will identify while loop statements and represent them as
    /// `WhileStatement` variants. These statements are then processed by the interpreter or compiler to execute the contained
    /// statements in a loop until the condition is no longer true.
    ///
    /// ### Last Updated: (v3.0.0)
    WhileStatement {
        conditions: Vec<Expression>,
        body: Box<Statement>,
        line: usize,
    },
    /// The `IfStatement` variant in the `Statement` enum represents an if statement.
    ///
    /// An if statement is used to conditionally execute a block of code based on the evaluation of a condition.
    /// It can also include optional else branches and else-if branches to handle multiple conditions.
    ///
    /// # Fields
    ///
    /// - `conditions`: This field holds a vector of `Expression` enums, which represent the conditions that must be true for the if statement to execute its body.
    /// - `then_branch`: This field holds a boxed `Statement` enum, which represents the body of the if statement that is executed if the condition is true.
    /// - `elif_branches`: This field holds a vector of tuples, where each tuple contains a vector of `Expression` enums representing the conditions for an else-if branch, and a boxed `Statement` enum representing the body of that branch.
    /// - `else_branch`: This field is an optional boxed `Statement` enum, which represents the body of the else branch that is executed if none of the conditions are true.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    /// # Example
    ///
    /// ```
    /// if x < 10 {
    ///  print("x is less than 10");
    /// } else if x == 10 {
    ///  print("x is equal to 10");
    /// } else {
    ///  print("x is greater than 10");
    /// }
    /// ```
    /// In this example, the `IfStatement` contains a condition `x < 10` and a body block with a print statement. It also includes an else-if branch for the condition `x == 10` and an else branch for when `x` is greater than `10`.
    ///
    /// # Usage
    ///
    /// When parsing Tron code, the scanner and parser will identify if statements and represent them as
    /// `IfStatement` variants. These statements are then processed by the interpreter or compiler to execute the appropriate branch based on the evaluation of the conditions.
    ///
    /// ### Last Updated: (v3.0.0)
    IfStatement {
        conditions: Vec<Expression>,
        then_branch: Box<Statement>,
        elif_branches: Vec<(Vec<Expression>, Box<Statement>)>,
        else_branch: Option<Box<Statement>>,
        line: usize,
    },
    /// The `FunctionStatement` variant in the `Statement` enum represents a function declaration statement.
    ///
    /// A function declaration statement is used to define a new function with a specified name, parameters, body, and optionally, an output type.
    /// It allows the programmer to encapsulate a sequence of statements into a reusable piece of code that can be called by name.
    ///
    /// # Fields
    ///
    /// - `name`: This field holds the `Token` that represents the name of the function being declared.
    /// - `params`: This field is a vector of tuples, where each tuple contains a `Token` representing the name of a parameter and `Token` representing the type of the parameter.
    /// - `body`: This field holds a vector of boxed `Statement` enums, which represent the sequence of statements that make up the body of the function.
    /// - `output_type`: `Token` that represents the return type of the function.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// # Example
    ///
    /// ```
    /// fn add(a: number, b: number): number{
    ///     return a + b;
    /// }
    /// ```
    ///
    /// In this example, the `FunctionStatement` declares a function named `add` with two parameters `a` and `b`, and a body that returns the sum of `a` and `b`.
    ///
    /// # Usage
    ///
    /// When parsing Tron code, the scanner and parser will identify function declaration statements and represent them as
    /// `FunctionStatement` variants. These statements are then processed by the interpreter or compiler to declare the specified functions in the current scope.
    ///
    /// ### Last Updated: (v3.1.0)
    FunctionStatement {
        name: Token,
        params: Vec<(Token, Token)>,
        body: Vec<Box<Statement>>,
        output_type: Token,
        line: usize,
    },
    /// The `ReturnStatement` variant in the `Statement` enum represents a return statement.
    ///
    /// A return statement is used to exit a function and optionally return a value to the caller. It is typically the last statement in a function body.
    ///
    /// # Fields
    ///
    /// - `keyword`: This field holds the `Token` that represents the `return` keyword.
    /// - `value`: This field is an optional `Expression` that represents the value to be returned by the function. If no value is provided, the function returns `null` or the default value for the return type.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// # Example
    ///
    /// ```
    /// fn add(a,b){
    ///     return a + b;
    /// }
    /// ```
    ///
    /// In this example, the `ReturnStatement` is used to return the sum of `a` and `b` from the `add` function.
    ///
    /// # Usage
    ///
    /// When parsing Tron code, the scanner and parser will identify return statements and represent them as
    /// `ReturnStatement` variants. These statements are then processed by the interpreter or compiler to return the specified value from the function.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// ### Last Updated: (v3.0.0)
    ReturnStatement {
        keyword: Token,
        value: Option<Expression>,
        line: usize,
    },
    /// The `BreakStatement` variant in the `Statement` enum represents a break statement.
    ///
    /// A break statement is used to exit the current loop or switch statement prematurely. It is typically used within the body of a loop or switch
    /// to terminate the loop or switch when a certain condition is met.
    ///
    /// # Fields
    ///
    /// - `keyword`: This field holds the `Token` that represents the `break` keyword.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// # Example
    ///
    /// ```
    /// while x < 10 {
    ///  x = x + 1;
    ///  if x == 5 {
    ///     break;
    ///  }
    /// }
    /// ```
    ///
    /// In this example, the `BreakStatement` is used to exit the loop when the variable `x` is equal to `5`.
    ///
    /// # Usage
    ///
    /// When parsing Tron code, the scanner and parser will identify break statements and represent them as
    /// `BreakStatement` variants. These statements are then processed by the interpreter or compiler to exit the current loop or switch statement.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// ### Last Updated: (v3.0.0)
    BreakStatement { keyword: Token, line: usize },
    /// The `SwitchStatement` variant in the `Statement` enum represents a switch statement.
    ///
    /// A switch statement is used to perform different actions based on different conditions. It evaluates an expression and executes the corresponding case block.
    ///
    /// # Fields
    ///
    /// - `condition`: This field holds the `Expression` that represents the condition to be evaluated in the switch statement.
    /// - `case_branches`: This field is a vector of tuples, where each tuple contains an `Expression` representing the case condition and a vector of `Statement` enums representing the statements to be executed for that case.
    /// - `default_branch`: This field is an optional vector of `Statement` enums that represents the default case block to be executed if none of the case conditions match.
    /// - `line`: This field represents the line number in the source code where the statement was found.
    ///
    /// # Example
    ///
    /// ```
    /// switch(x){
    ///  case 1 {
    ///   print("x is 1");
    ///  }
    ///  case 2 {
    ///   print("x is 2");
    ///  }
    ///  default {
    ///   print("x is neither 1 nor 2")
    ///  }
    /// }
    /// ```
    ///
    /// In this example, the `SwitchStatement` evaluates the value of `x` and executes the corresponding case block. If `x` is not `1` or `2`, it executes the default case block.
    ///
    /// # Usage
    ///
    /// When parsing Tron code, the scanner and parser will identify switch statements and represent them as
    /// `SwitchStatement` variants. These statements are then processed by the interpreter or compiler to execute the appropriate case block based on the evaluation of the condition.
    ///
    /// ### Last Updated: (v3.0.0)
    SwitchStatement {
        condition: Expression,
        case_branches: Vec<(Expression, Vec<Statement>)>,
        default_branch: Option<Vec<Statement>>,
        line: usize,
    },
}

/// The `Token` struct in Rust represents a token in the Tron.
///
/// Each `Token` contains information about the type of token, the lexeme (the actual text of the token), an optional literal value, and the line number where the token was found.
///
/// # Fields
///
/// - `token_type`: This field holds the `TokenType` enum variant that represents the type of the token.
/// - `lexeme`: This field holds the `String` that represents the actual text of the token in the source code.
/// - `literal`: This field is an optional `LiteralValue` enum variant that represents the literal value associated with the token, if any.
/// - `line_number`: This field holds the `usize` that represents the line number in the source code where the token was found.
///
/// # Example
///
/// ```
/// let token = Token {
///  token_type: TokenType::Identifier,
///  lexeme: "x".to_string(),
///  literal: None,
///  line_number: 1
/// }
/// ```
///
/// In this example, a `Token` is created to represent an identifier with the name "x" on line 1 of the source code.
///
/// # Usage
///
/// The `Token` struct is used throughout the Tron language's scanner and parser to represent individual tokens in the source code.
/// Each token is categorized by its type, which can be a keyword, operator, identifier, literal, or other types of tokens.
/// The `literal` field is used to store the actual value of literals, such as numbers or strings, while the `lexeme` field stores the text of the token.
///
/// ### Last Updated: (v3.0.0)
#[derive(Debug, Clone, PartialEq)]
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
/// The `Scanner` struct in Rust is responsible for tokenizing the source code of the Tron.
///
/// It reads the source code character by character and categorizes them into tokens based on the language's syntax rules.
/// The `Scanner` maintains the current position in the source code and provides methods to scan tokens,
/// handle errors, and manage the scanning process.
///
/// # Fields
///
/// - `source`: This field holds the source code as a `String`.
/// - `tokens`: This field is a vector of `Token` enums that will hold the tokens generated by the scanner.
/// - `start`: This field holds the starting position of the current token being scanned.
/// - `current`: This field holds the current position in the source code.
/// - `line`: This field holds the current line number in the source code.
/// - `keywords`: This field is a `HashMap` that maps keywords to their corresponding `TokenType` values.
///
/// # Usage
///
/// The `Scanner` struct is used to tokenize the source code of the Tron language. It is typically instantiated with the source code
/// and then used to scan tokens, which are then used by the parser to construct the abstract syntax tree (AST).
///
/// # Example
///
/// ```
/// let source_code = "let x = 5";
/// let mut scanner = Scanner::new(source_code);
/// let tokens = scanner.scan_tokens();
/// ```
///
/// In this example, a `Scanner` is created with the source code `"let x = 5;"`, and then the `scan_tokens` method is called to tokenize the code.
///
/// ### Last Updated: (v3.0.0)
#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}
impl Scanner {
    /// The `new()` function is a constructor for the `Scanner` struct.
    ///
    /// It takes a reference to a string slice (`&str`) representing the source code to be tokenized and returns a new instance of the `Scanner`.
    /// The `new()` function initializes the `Scanner` with the source code, an empty vector of tokens, the starting position set to 0, the current position set to 0,
    /// the line number set to 1, and a `HashMap` of keywords mapped to their corresponding `TokenType` values.
    ///
    /// # Parameters
    ///
    /// - `source`: A reference to a string slice (`&str`) that contains the source code to be tokenized.
    ///
    /// # Return Value
    ///
    /// A new instance of the `Scanner` struct with the source code and an empty vector of tokens.
    ///
    /// # Usage
    ///
    /// The `new()` function is typically called when you want to create a new `Scanner` to tokenize a piece of Tron code.
    /// It sets up the initial state of the `Scanner` so that it can begin the tokenization process.
    ///
    /// # Example
    ///
    /// Here is an example of how the `new()` function might be used:
    ///
    /// ```
    /// let source_code = "let x = 5";
    /// let mut scanner = Scanner::new(source_code);
    /// ```
    ///
    /// In this example, the `new()` function is called with the source code `"let x = 5;"`, creating a new `Scanner` that is ready to tokenize this code.
    ///
    /// ### Last Updated: (v3.0.0)
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
    /// The `scan_tokens()` method of the `Scanner` struct is responsible for tokenizing the source code of the Tron language.
    ///
    /// It iterates over the source code, character by character, and categorizes them into tokens based on the language's syntax rules.
    /// The method returns a `Result` containing a vector of `Token` enums if successful, or an error message if an error occurs during the tokenization process.
    ///
    /// # Return Value
    ///
    /// A `Result` containing a vector of `Token` enums if the tokenization is successful, or an error message if an error occurs.
    ///
    /// # Usage
    ///
    /// The `scan_tokens()` method is typically called after the `Scanner` has been initialized with the source code.
    /// It processes the source code and generates a list of tokens that can be used by the parser to construct the abstract syntax tree (AST).
    ///
    /// # Example
    ///
    /// Here is an example of how the `scan_tokens()` method might be used:
    ///
    ///
    /// ```
    /// let source_code = "let x = 5";
    /// let mut scanner = Scanner::new(source_code);
    /// let tokens = scanner.scan_tokens();
    /// ```
    ///
    /// In this example, the `scan_tokens()` method is called on a `Scanner` instance to tokenize the source code `"let x = 5;"`.
    pub fn scan_tokens(mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => {
                    TronError::throw("E1004", self.line, vec![msg]);
                }
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
    /// The `scan_token()` method of the `Scanner` struct is responsible for scanning a single token from the source code.
    ///
    /// It reads the next character in the source code and determines the type of token it represents. The method then adds the token to the list of tokens.
    /// The method returns a `Result` with `Ok(())` if the token is successfully scanned, or an error message if an error occurs during the scanning process.
    ///
    /// # Return Value
    ///
    /// A `Result` with `Ok(())` if the token is successfully scanned, or an error message if an error occurs.
    ///
    /// # Usage
    ///
    /// The `scan_token()` method is called repeatedly by the `scan_tokens()` method to tokenize the entire source code.
    /// It processes one character at a time and categorizes it into a token based on the language's syntax rules.
    ///
    /// ### Last Updated: (v3.0.0)
    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            '[' => self.add_token(LeftBracket),
            ']' => self.add_token(RightBracket),
            ',' => self.add_token(Comma),
            '%' => self.add_token(Percent),
            ':' => self.add_token(Colon),
            '.' => self.add_token(Dot),
            '&' => self.add_token(And),
            '|' => {
                let token = if self.char_match('|') { Or } else { Line };
                self.add_token(token);
            }
            '?' => self.add_token(Question),
            '-' => {
                let token = if self.char_match('-') {
                    Decrement
                } else {
                    Minus
                };
                self.add_token(token);
            }
            '+' => {
                let token = if self.char_match('+') {
                    Increment
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
                if c.is_digit(10) {
                    self.number()?;
                } else if c.is_alphabetic() || c == '@' || c == '_' || c == '$' {
                    self.identifier();
                } else {
                    TronError::throw("E1002", self.current, vec![c.to_string()]);
                }
            }
        }
        Ok(())
    }
    /// The `identifier()` method of the `Scanner` struct is responsible for scanning an identifier from the source code.
    ///
    /// It reads the source code character by character until it encounters a character that is not part of an identifier.
    /// The method then adds the identifier to the list of tokens.
    ///
    /// # Usage
    ///
    /// The `identifier()` method is called internally by the `Scanner` when it encounters a character that could be the start of an identifier.
    /// It processes the source code and generates a token of type `Identifier` if the sequence of characters forms a valid identifier.
    ///
    /// ### Last Updated: (v3.0.0)
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric()
            || self.peek() == '@'
            || self.peek() == '_'
            || self.peek() == '$'
        {
            self.advance();
        }
        let substring = &self.source[self.start..self.current];
        if let Some(&t_type) = self.keywords.get(substring) {
            self.add_token(t_type);
        } else {
            self.add_token(Identifier);
        }
    }
    /// The `number()` method of the `Scanner` struct is responsible for scanning a number literal from the source code.
    ///
    /// It reads the source code character by character until it encounters a character that is not part of a number literal.
    /// The method then adds the number literal to the list of tokens.
    ///
    /// # Usage
    ///
    /// The `number()` method is called internally by the `Scanner` when it encounters a character that could be the start of a number literal.
    /// It processes the source code and generates a token of type `Number` if the sequence of characters forms a valid number literal.
    ///
    /// ### Last Updated: (v3.0.0)
    fn number(&mut self) -> Result<(), String> {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let substring = &self.source[self.start..self.current];
        let value = substring.parse::<f32>();
        match value {
            Ok(value) => self.add_token_lit(Number, Some(NumericValue(value))),
            Err(_) => {
                panic!("\n Could not parse number: {}", substring)
            }
        }
        Ok(())
    }
    /// The `peek_next` method of the `Scanner` struct is used to look ahead at the next character in the source code without advancing the current position.
    ///
    /// This method is particularly useful when the scanner needs to check the character following the current one to determine the type of token being scanned.
    /// It returns the next character in the source code if it exists, or a null character (`'\0'`) if the end of the source code has been reached.
    ///
    /// # Return Value
    ///
    /// - `char`: The next character in the source code, or a null character (`'\0'`) if the end of the source code has been reached.
    ///
    /// # Usage
    ///
    /// The `peek_next` method is called internally by the `Scanner` when it needs to look ahead at the next character to determine the type of token being scanned.
    /// It is used in conjunction with other methods like `peek` and `advance` to handle multi-character tokens and to ensure that the scanner does not go beyond the end of the source code.
    ///
    /// # Notes
    ///
    /// - The `peek_next` method does not advance the current position in the source code. It only returns the next character without modifying the scanner's state.
    /// - The method is designed to be safe and will not panic if called at the end of the source code. Instead, it will return a null character (`'\0'`).
    /// - This method is a crucial part of the scanner's functionality, as it allows the scanner to make decisions based on the upcoming characters in the source code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }
    /// The `string` method of the `Scanner` struct is responsible for scanning a string literal from the source code.
    ///
    /// It reads the source code character by character until it encounters a character that is not part of a string literal.
    /// The method then adds the string literal to the list of tokens.
    ///
    /// # Usage
    ///
    /// The `string` method is called internally by the `Scanner` when it encounters a character that could be the start of a string literal.
    /// It processes the source code and generates a token of type `StringLit` if the sequence of characters forms a valid string literal.
    ///
    /// # Return Value
    ///
    /// - `Result<(), String>`: Returns `Ok(())` if the string literal is successfully scanned, or an error message if an error occurs.
    ///
    /// # Notes
    ///
    /// - The `string` method handles the scanning of string literals enclosed in double quotes (`"`).
    /// - It ensures that the string literal is properly terminated by a double quote. If the end of the source code is reached before a closing double quote is found, the method will panic with an "unterminated string" error.
    /// - The method increments the line number if a newline character (`\n`) is encountered within the string literal.
    /// - The scanned string literal is added to the list of tokens with its associated literal value.
    ///
    /// ### Last Updated: (v3.0.0)
    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            TronError::throw("E1001", self.current, vec![]);
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_lit(StringLit, Some(StringValue(value.to_string())));
        Ok(())
    }
    /// The `is_at_end` method of the `Scanner` struct checks if the scanner has reached the end of the source code.
    ///
    /// This method is used to determine if there are no more characters left to scan in the source code. It returns `true` if the current position is equal to or greater than the length of the source code, indicating that the end of the source code has been reached.
    ///
    /// # Usage
    ///
    /// The `is_at_end` method is called internally by the `Scanner` to check if the end of the source code has been reached before attempting to scan the next character.
    /// It is used in conjunction with other methods like `peek` and `advance` to ensure that the scanner does not go beyond the end of the source code.
    ///
    /// # Return Value
    ///
    /// - `bool`: Returns `true` if the end of the source code has been reached, or `false` otherwise.
    ///
    /// # Notes
    ///
    /// - The `is_at_end` method is a crucial part of the scanner's functionality, as it allows the scanner to make decisions based on the current position in the source code.
    /// - It is used to prevent out-of-bounds errors when scanning the source code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    /// The `peek` method of the `Scanner` struct is used to look at the next character in the source code without advancing the current position.
    ///
    /// This method is particularly useful when the scanner needs to check the character following the current one to determine the type of token being scanned.
    /// It returns the next character in the source code if it exists, or a null character (`'\0'`) if the end of the source code has been reached.
    ///
    /// # Usage
    ///
    /// The `peek` method is called internally by the `Scanner` when it needs to look ahead at the next character to determine the type of token being scanned.
    /// It is used in conjunction with other methods like `peek_next` and `advance` to handle multi-character tokens and to ensure that the scanner does not go beyond the end of the source code.
    ///
    /// # Return Value
    ///
    /// - `char`: The next character in the source code, or a null character (`'\0'`) if the end of the source code has been reached.
    ///
    /// # Notes
    ///
    /// - The `peek` method does not advance the current position in the source code. It only returns the next character without modifying the scanner's state.
    /// - The method is designed to be safe and will not panic if called at the end of the source code. Instead, it will return a null character (`'\0'`).
    /// - This method is a crucial part of the scanner's functionality, as it allows the scanner to make decisions based on the upcoming characters in the source code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }
    /// The `char_match` method of the `Scanner` struct is used to check if the next character in the source code matches a specified character.
    ///
    /// This method is particularly useful when the scanner needs to verify if the current character is followed by a specific character to determine the type of token being scanned.
    /// It returns `true` if the next character matches the specified character, and `false` otherwise. If the next character matches, the current position is advanced by one.
    ///
    /// # Usage
    ///
    /// The `char_match` method is called internally by the `Scanner` when it needs to check if the next character matches a specific character.
    /// It is used in conjunction with other methods like `peek` and `advance` to handle multi-character tokens and to ensure that the scanner does not go beyond the end of the source code.
    ///
    /// # Return Value
    ///
    /// - `bool`: Returns `true` if the next character matches the specified character, and `false` otherwise.
    ///
    /// # Notes
    ///
    /// - The `char_match` method advances the current position in the source code if the next character matches the specified character.
    /// - The method is designed to be safe and will not panic if called at the end of the source code. Instead, it will return `false`.
    /// - This method is a crucial part of the scanner's functionality, as it allows the scanner to make decisions based on the upcoming characters in the source code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn char_match(&mut self, ch: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != ch {
            return false;
        }
        self.current += 1;
        true
    }
    /// The `advance` method of the `Scanner` struct is used to move the current position in the source code to the next character.
    ///
    /// This method is called internally by the `Scanner` to progress through the source code one character at a time. It returns the current character and then increments the current position.
    ///
    /// # Usage
    ///
    /// The `advance` method is used in conjunction with other methods like `peek` and `char_match` to handle multi-character tokens and to ensure that the scanner does not go beyond the end of the source code.
    /// It is called repeatedly by the `scan_tokens` method to tokenize the entire source code.
    ///
    /// # Return Value
    ///
    /// - `char`: Returns the current character in the source code before advancing the position.
    ///
    /// # Notes
    ///
    /// - The `advance` method is a crucial part of the scanner's functionality, as it allows the scanner to progress through the source code character by character.
    /// - It is used to update the current position in the source code, which is essential for the scanning process.
    ///
    /// ### Last Updated: (v3.0.0)
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }
    /// The `add_token` method of the `Scanner` struct is used to add a token to the list of tokens that have been scanned from the source code.
    ///
    /// This method takes a `TokenType` as an argument and creates a new `Token` with the current lexeme and line number. It then adds this token to the `tokens` vector.
    ///
    /// # Usage
    ///
    /// The `add_token` method is called internally by the `Scanner` when it has identified a complete token in the source code.
    /// It is used to record the token's type, lexeme, and line number for later use by the parser.
    ///
    /// # Parameters
    ///
    /// - `token_type`: The `TokenType` enum variant that represents the type of the token being added.
    ///
    /// # Notes
    ///
    /// - The `add_token` method is a crucial part of the scanner's functionality, as it allows the scanner to build a list of tokens that can be used by the parser to construct the abstract syntax tree (AST).
    /// - It is used to ensure that the scanner's output is a sequence of tokens that accurately represents the source code.
    ///
    /// ### Last Updated: (v3.0.0)
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }
    /// The `add_token_lit` method of the `Scanner` struct is used to add a token with an associated literal value to the list of tokens that have been scanned from the source code.
    ///
    /// This method takes a `TokenType` and an `Option<LiteralValue>` as arguments and creates a new `Token` with the current lexeme, line number, and the provided literal value. It then adds this token to the `tokens` vector.
    ///
    /// # Usage
    ///
    /// The `add_token_lit` method is called internally by the `Scanner` when it has identified a complete token with an associated literal value in the source code.
    /// It is used to record the token's type, lexeme, line number, and literal value for later use by the parser.
    ///
    /// # Parameters
    ///
    /// - `token_type`: The `TokenType` enum variant that represents the type of the token being added.
    /// - `literal`: An `Option<LiteralValue>` that represents the literal value associated with the token, if any.
    ///
    /// # Notes
    ///
    /// - The `add_token_lit` method is a crucial part of the scanner's functionality, as it allows the scanner to build a list of tokens with associated literal values that can be used by the parser to construct the abstract syntax tree (AST).
    /// - It is used to ensure that the scanner's output is a sequence of tokens that accurately represents the source code, including any literal values that are part of the tokens.
    ///
    /// ### Last Updated: (v3.0.0)
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
