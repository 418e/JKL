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
let name = input("name please.");
print "Hello " + name;
```

```rust
fn add(a,b) do
  return a + b;
end

print add(1,5);
```

```rs
  let x = 0;

  if x > 0 do
    print "x > 0";
  end else if x < 0 do
    print "x < 0";
  end else do
    print "x = 0";
  end

```

## Installation

```bash
curl -o tron https://tronlang.org/tron-lang
sudo mv tron /usr/local/bin/
sudo chmod +x /usr/local/bin/tron
```

## Extension

Install from [marketplace](https://marketplace.visualstudio.com/items?itemName=TronLang.tron-lang), or type `tronlang` in vscode extension search window.

### features

- syntax highlighting
- basic completion
- `.tron` icon

soon:

- autocompletion
- better pattern matching

## Initializing Project

```rs
print "Hello, world!";
```

create `main.tron` and then run the following command in the cli

```bash
tron main.tron
```

Happy hacking!
