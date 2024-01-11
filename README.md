<div align="center">

![Tron Logo](https://tronlang.org/tron.svg)

# Welcome to Tron Programming Language

An open-source, fast, and expressive programming language crafted with ❤️ in Rust.

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

</div>

## Introduction

Tron is a modern programming language that prioritizes developer comfort and flexibility. With Tron, you can choose from a variety of syntax options to express your ideas in the way that feels most natural to you. Whether you prefer concise function declarations or more descriptive ones, Tron adapts to your style.

## Features

- **Multiple Syntax Choices**: Use different keywords to represent the same concept, allowing you to write code that aligns with your preferences.
- **Fast Execution**: Compiled with Rust's performance in mind, Tron programs run swiftly and efficiently.
- **Open Source**: Tron is developed openly on GitHub, and we welcome contributions from the community.

## Quick Start

To get started with Tron, install the Tron interpreter on your system and create your first Tron program:

```bash
curl -sSL https://tronlang.org/install.sh | bash
```

Create a file named `main.tron` with the following content:

```tron
print "Hello, world!";
```

Run your program with:

```bash
tron main.tron
```

You should see the output:

```
Hello, world!
```

## Examples

Here are some examples to show you the flexibility of Tron's syntax:

```rs
// Function declaration can be done in multiple ways
fn greet() {
  print "Hello";
}

fun greet() do
  print "Hello";
end

function greet() start
   print "Hello";
stop

def greet():
  say "Hello";
.

define greet() {
  out "Hello";
}
// you can even choose, what kind of brackets you want to use
```

```rs
// Conditional statements with different keywords if condition { ... } ? condition { ... }

if i > 0 {

} else if i == 0 {

} else {

}

? i > 0:

. ?>> i == 0:

. ?> :

.
```

```rs
// Declaring variables with different keywords

let i = 1;
$ i = 1;
declare i equals 1;
const i is 1;
var i assign 1;
```

And many more...

## Documentation

For detailed documentation, visit [Tron's official documentation](https://tronlang.org/docs).

## Community

Join the Tron community:

- [GitHub](https://github.com/tronlang/Tron)
- [Discord](https://discord.gg/8jSvkTSemE)
- [Twitter](https://twitter.com/tron_language)

## Contributing

We welcome contributions of all kinds. If you're interested in contributing to Tron, please read our [contribution guidelines](https://github.com/TronLang/Tron/CONTRIBUTING.md).

## License

Tron is licensed under the MIT License. See [LICENSE](https://github.com/TronLang/Tron/LICENSE) for more information.

Happy hacking with Tron!
