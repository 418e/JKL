<div align="center">

![Logo](https://tronlang.org/tron.svg)

</div>



<div align="center">

# Tron Programming Language

An Open Source, Fast and Simple Programming Language written in Rust

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

</div>

## Usage/Examples

```rust
let name = :in_"Please enter your name: ";
print "Hello " + name;
```

```rust
fn add(a,b){
  return a + b;
}

print add(1,5);
```

```rs
let age = :num_"Please enter your age: ";

if age < 18 {
    print "minor";
    exit;
} else {
    print "adult";
    run "adultsonly.tron";
}
```

[See More](https://github.com/418e/Tron-Examples)

## Installation

Before starting the installation, please make sure that you have already installed [Rust](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If rust is already installed you are free to install Tron in your project:

```bash
mkdir tron-project
cd tron-project
curl https://tronlang.org/tron-lang --output tron
```

add the configuration file `tron.toml` and then

```toml 
name = "TronProject"
entry = "main"
version = "0.0.1"
authors = "YOU"
license = "MIT"
decor = "default"
pointer = "default"
env = "prod"
experimental = "false"
credits = "false"
warnings = "true"
```

create `src/main.tron` and then run the following command in the cli

```bash
./tron
```

## Acknowledgements

- [Official Website](https://tronlang.org)
- [Blog](https://blog.tronlang.org)
- [Documentation](https://github.com/418e/Tron/wiki)

## Authors

- [@418e](https://www.github.com/418e)

