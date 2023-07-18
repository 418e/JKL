# JekoLang

## Simple programming language written in Rust.

# Installation

You must have already installed [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) before cloning this repository.
create file `main`. Insert the code bellow in the `main` and run `cargo run main` in the terminal.
```

print "Hello World!";

```

# Basics

After installing Rust and Cargo, you can already download/clone this repository and run the following command in the terminal:

```
cargo run
```

# Usage

JekoLang is a High-Level, dynamically typed, functional and object oriented, general purpose programming language with automatic memory management.

# Data Types

JekoLang has data types you are already familiar with.

## Booleans. - True and False

## Numbers - 12345 (An integer) , 123.45 (A decimal number)

## Strings - "Hello, world!", "", "12345"

## Nil - same as null

# Expressions

## Arithmetic - + , - , \*, /

## Comparasion and equality - < , >, <=, >=, != ,== ...

## Logical Operators = or, and, !

# Variables

variables in JekoLang are dynamically typed so that you don't have to worry about specifying types for each variable.

```
let name = "John";
let nil;

print name;
```

# Control Flow

## If statement

```
if (condition){
    print "yes";
} else {
    print "no";
}
```

## While loop

```
let i = 1;
while (i < 10) {
    print i;
    i = i + 1;
}
```

## For loop

```
for (let i = 1; i < 10; i = i + 1) {
    print i;
}
```

# Functions
```
fn Double(a){
    print a * 2;
    return a * 2;
}

Double(7);
```

# Classes

```
class Drive {
    start(){
        print "Engine started";
    }

    drive(where){
        print "Driving to the " + where + ".";
    }
}

// You can store them in the variables as well as in the functions
let someVariable = Drive;
someFunction(Drive);
```

## Instantiation and initialization

```
Drive.with = "a dog";

class Drive {
    drive(where){
        print "Driving to the " + where + "with" + this.with + ".".
    }

// ...
}
```

```
 class Drive {
    init(with){
        this.with = with
    }

    // ...
 }

 var drivewithwife = Drive("wife");
 drivewithwife.drive("Store");
```
