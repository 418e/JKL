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
fn add(a,b){
  return a + b;
}

print add(1,5);
```

```rs
  let x = 0;

  if x > 0 {
    print "x > 0";
  } elif x < 0 {
    print "x < 0";
  } else {
    print "x = 0";
  }

```

## Installation

```bash
curl -o tron https://tronlang.org/tron-lang
sudo mv tron /usr/local/bin/
sudo chmod +x /usr/local/bin/tron
```


## Initializing Project
```
mkdir tronproject
cd tronproject
tron config
```

create `main.tron` and then run the following command in the cli

```bash
tron main.tron
```

Happy hacking!