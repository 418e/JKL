<div align="center">

![Logo](https://tronlang.org/tron.svg)

</div>

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

<span>

![npm](https://img.shields.io/npm/v/tron-lang)

</span>

<div align="center">

# Tron Programming Language



An Open Source, Fast and Simple Programming Language written in Rust

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

## Acknowledgements

- [Official Website](https://tronlang.org)
- [Blog](https://blog.tronlang.org)
- [Documentation](https://docs.tronlang.org)

## Installation

Before starting the installation, please make sure that you have already installed [Rust](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If rust is already installed you are free to install Tron in your project:

```bash
npm install tron-lang
```

add the following code in the `package.json`:

```json
  "scripts": {
    "install": "cd node_modules/tron-lang \n cargo build",
    "start": "cd node_modules/tron-lang \n cargo run"
  }
```

Create a new configuration file `tron.toml` in the root of the project:

```yml
name = "ProjectName"
entry = "main"
version = "1.0.0"
authors = "YOU"
license = "MIT"
decor = "default"
pointer = "default"
env = "prod"
experimental = "false"
credits = "false"
warnings = "true"
bench = "false"
```

create the `main.tron` file in the root of the project:

```rs
print "Hello, Tron!";
```

and run the `npm start` command in the terminal
dev
## Support

For support, email `tronlang@proton.me` or `pi_bmi@proton.me`.

## Authors

- [@418e](https://www.github.com/418e)
