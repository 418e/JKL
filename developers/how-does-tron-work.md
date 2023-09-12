# How does Tron interpreter work

## What is an interpreter?

An interpreter is a program that translates high-level programming languages into machine code that a computer can understand and execute or another programming language. The process of interpreting a program involves several steps, including lexical analysis, parsing, semantic analysis, and code generation and execution.

### Lexical Analysis

This is the first stage of the interpretation process. The interpreter reads the source code and breaks it down into individual components known as tokens. Each token represents a particular element of the code, such as a keyword, an identifier, an operator, or a punctuation mark

### Parsing

After lexical analysis, the interpreter begins parsing. During this stage, the interpreter checks the tokens for syntax errors. It ensures that the tokens follow the correct grammatical structure of the programming language. If the syntax is correct, the parser generates an abstract syntax tree (AST) or some other form of intermediate representation of the code

### Semantic Analysis

After generating the AST, the interpreter performs semantic analysis. The interpreter checks the program for semantic errors, such as type mismatches or undeclared variables. If the semantic analysis is successful, the interpreter proceeds to the next stage

### Code Generation and Execution

In the final stage, the interpreter translates the AST into machine code or some form of intermediate code. It then executes this code line by line. If it encounters a function call, it translates and executes all the lines in the function before returning to the next line in the calling function. If an error occurs during execution, the interpreter stops and reports the error

## How does Tron Interpret?

- `environment.rs`contains code related to setting up the enviroment in which the Tron runs. This includes global functions and variables, configuration settings and etc.

- `expr.rs` contains expressions. An expression is a piece of code that produces a value when executed.

- `function.rs` contains code that defines functions.

- `interpreter.rs` is a file were source code is turned into rust code and directly executed.

- `main.rs` is the entry point of the application.

- `parser.rs` contains a parser, which cheks for syntactic errors and builds a data structure implicit in the input tokens.

- `resolver.rs` file is responsible for resolving dependencies, paths and names in the code.

- `scanner.rs` contains a scanner (lexer). A scanner breaks up a sequence of input characters into pieces called tokens, which it categorizes as different types.

- `stmt.rs` contains code related to statements. A statement is a piece of code that performs an action.


