<div align="center">

![Logo](https://tronlang.org/tron.svg)

</div>

<div align="center">

# Tron Programming Language

An Open Source, Fast and Sweet Programming Language written in Rust

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

</div>

In the Tron (_since `v2.1.0`_) you have mutliple choices when it comes to syntax. Which means that, different keywords might represent same thing, for example: `function`, `fn`, `fun`, `def` and `define`, all define function, you can use any one of them and will work.

## Keywords

|    name     |                           keywords                            |          example          |
| :---------: | :-----------------------------------------------------------: | :-----------------------: |
|  function   |                fn, fun, function, def, define                 |  fn add(){ print "hi"; }  |
| block start |               `{`, do, start, doing, begin, `:`               | def add(): say "hi"; end. |
|  block end  |                   `}`, end, done, stop, `.`                   | def add(): say "hi"; end. |
|     if      |                            if, `?`                            |                           |
|    else     |                else, otherwise, if, nor, `?>`                 |                           |
|    elif     |              else if, elif, what if, whatif, ?>>              |                           |
|    true     |                    affirmative, true, yes                     |                           |
|    false    |                  negative, false, falsy, no                   |                           |
|    null     |                           null, nil                           |                           |
|    print    |           print, say, shout, log, out, output, tell           |                           |
|    input    |                        input, in, inp                         |                           |
|    error    |               panic, alaram, throw, error, err                |                           |
|   import    |      include, import, require, use, payload, unload, lib      |                           |
|    exit     |                     exit, kill, terminate                     |                           |
|   return    |                    return, respond, append                    |                           |
|  variable   |           let, var, const, state, declare, dec, `$`           |                           |
|    while    |                          while, loop                          |                           |
|    bench    |                  bench, test, measure, time                   |                           |
|    plus     |                           plus, `+`                           |                           |
|    minus    |                          minus, `-`                           |                           |
|  multiply   |                 multiply, multiplied by, `*`                  |                           |
|   divide    |                    divide, divided by, `/`                    |                           |
|   assign    | `=`, `assign`, `equal`, `equals`, `is`, `are`, `assigned to`, |                           |
|  comments   |                            // or #                            |                           |

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

**\*note!**- you could use any syntax from above\*

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
