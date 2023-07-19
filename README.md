# JKL Programmin Language

## Simple programming language written in Rust.
## v0.1.4
# Installation

Download and install [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you don't have them and then clone this repository.
create file `main.jkl` in the root. <br>
`main.jkl`

```

print "Hello World!";

```

you can run the code using `cargo`:

```bash
cargo run main
```

# Usage

JKL is a High-Level, dynamically typed, functional and object oriented, general purpose programming language with automatic memory management.

# Data Types

- Booleans
- Numbers
- Strings
- Nil (same as Null)

# Expressions

### Arithmetic

- `-`
```
print 8 - 4; // 4 
```
- `+`
```
print 2 + 2; // 4
```
- `*`
```
print 3 * 7; // 21 
```
- `/`
```
print 21 / 3; // 7
```
- `++`
```
print ++12; // 13
```
- `--`
```
print --12; // 11
```
- `^` 
```
print ^9 // 81
```
- `&` 
```
print &49 // 7
```

### Comparasion and equality

- `>`
- `<`
- `>=`
- `<=`
- `==`
- `!`
- `!=`
- `+=` [Soon]
- `-=` [Soon]
### Logical Operators
- `and`
- `or`

### Generating random numbers
- `@`
```
print 100@; // generates random number from 0 to 100
print 10@50; // generates random number from 10 to 50
```
# Variables

```
let StringVariable = "String";
let IntegerVariable = 23;
let PointerVariable = 3.1415;
let booleanVariable = true;
let NilVariable;
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
