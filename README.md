[![AGPL License](https://img.shields.io/badge/license-Apache%20v2-blue.svg)](https://github.com/jkl-org/Tron/blob/main/LICENSE)

# Tron Programming Language

An open-source programming language written in Rust.

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

create new configuration file `tron.toml` in the root of the project:

```yml
name = "ProjectName"
entry = "main"
version = "0.0.1"
authors = "YOU"
decor = "default"
pointer = "default"
```

create the `main.tron` file in the root of the project:

```rs
print "Hello, Tron!";
```

and run the `npm start` command in the terminal

## Usage/Examples

```rs
let x = 3;
if x > 1 {
  print "passed";
} else {
  panic "failed";
}
```

## Acknowledgements

- [Home Page](https://tronlang.org/)
- [Documentation](https://docs.tronlang.org/)
- [News Page](https://news.tronlang.org/)

## Contact

- `tronlang@proton.me`.
- `pi_bmi@proton.me`.
