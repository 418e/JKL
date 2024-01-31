# 2.6.0 - Feb 1

- if statement can now handle multiple expressions
- else if statement can now handle multiple expressions
- while statement can now handle multiple expressions
- assignent can now handle multiple variables
- variables with uppercase name now will be immutable
- updates now will be released once in two weeks

# 2.5.0 - Jan 15

- added type system
- added 5 types: `number`, `string`, `bool`, `null`, `array`
- added type checking for variables and function paramters:

```
let i: number = 0;
// let i = 0; is still valid

fn sum(x: number , y: number) {
// fn sum(x,y) is still valid
    return x + y;
}
```

# 2.4.1 - Jan 14

- fixed bugs
- new operators: `**`(`**3` = 9) and `&`(`&9` = 3)
- new keywords:

> - `And` : `&&`
> - `Or`: `||`
> - `Else if`: `??`
> - `multiply`: `times`
> - `>`: `more`, `more than`
> - `<`: `less`, `less than`
> - `/`: `slash`

- removed `#` keyword for `else if`

# 2.4.0 - Jan 13

- added short functions

```
fn main(x) : x + 1;
print main(6);

// 7
```

- added sleep native: `sleep(time)`:

```
sleep(2000);

// will create delay for 2s
```

- added ternary native: `ternary(statement, value1, value2):`:

```
let i = 0;
let x = ternary(i > 1, 4, 5);
print x;

// 5
// if statement is true, return value1, if not: value2
```

- added cmd native: `cmd("command")`:

```
let x = cmd("echo hi");
print x;

// "hi"
```

- removed cmd functions

# 2.3.1 - Jan 13

- performance improvements
- binary size reduction
- removed external libraries (except rand)

# 2.3.0 - Jan 12

- new cli command `tron update`
- added new `wait/before` statement:

```
wait 1000 {
  print 1;
}
```

will print 1 after 1000ms delay

```
wait 1000 {
  print 1;
} before 200 {
  print 0;
}
```

will print 0, with 200ms delay 1000/200 times, and then print 1;

wait keywords: wait, hold pause
before keywords: before, until, during

- removed `.` and `:` as end/start
- removed `#` comments
  `?>>` elif => `#`

# 2.2.0 - Jan 11

- added variable immutability
- removed try/catch statement, input statement and #array library
- numbers changed from f64 to f32 type
- added `then` (start bracket), `as` (assign operator), `exe`(cmd function), `execute`(cmd function), `run`(cmd function), `cmd`(cmd function)
- removed `incr` (increment), `decr`(decrement), `what if`(elif), `whatif`(elif), `dec`(variable), `state`(variable), `respond`(return), `append`(return)

# 2.1.1 - Jan 11

- `len()` now returns numbers
- `pow()`, `root()`, `random()`, `min()`, `max()`, `log()`, `log2`, `log10`, `ceil` to math library
- changed output style
- released cmds, `fn name <- `

```
fn echo <- "ls";
let response = echo();
say response;
```

# 2.1.0 - Jan 10

### ✨ Syntatic Sweetness

Developers can now choose, which keyword they want to use:

| name        | variants                                                      |
| :---------- | :------------------------------------------------------------ |
| function    | fn, fun, function, def, define                                |
| block start | `{`, do, start, doing, begin, `:`                             |
| block end   | `}`, end, done, stop, `.`                                     |
| if          | if, `?`                                                       |
| else        | else, otherwise, if, nor, `?>`                                |
| elif        | else if, elif, what if, whatif, ?>>                           |
| true        | affirmative, true, yes                                        |
| false       | negative, false, falsy, no                                    |
| null        | null, nil                                                     |
| print       | print, say, shout, log, out, output, tell                     |
| input       | input, in, inp                                                |
| error       | panic, alaram, throw, error, err                              |
| import      | include, import, require, use, payload, unload, lib           |
| exit        | exit, kill, terminate                                         |
| return      | return, respond, append                                       |
| variable    | let, var, const, state, declare, dec, `$`                     |
| while       | while, loop                                                   |
| bench       | bench, test, measure, time                                    |
| plus        | plus, `+`                                                     |
| minus       | minus, `-`                                                    |
| multiply    | multiply, multiplied by, `*`                                  |
| divide      | divide, divided by, `/`                                       |
| assign      | `=`, `assign`, `equal`, `equals`, `is`, `are`, `assigned to`, |
| comments    | // or #                                                       |

# 2.0.0 - Jan 10

## ✨ New features

- libraries
  > ability to add external functions
- standart libraries
  > use `include "#math"` or `include "#array"` to import math and array native function.
- Tronlang extension is back!

## 📝 Changes

- changed syntax:

code blocks

```rs
// before

fn greet(){
    print "hi";
}

// now

fn greet() do
    print "hi";
end
```

else if statement

```rs
// before

if smthng {

} elif smthng {

} else {

}

// now

if smthng do

end else if smthng do

end else do

end
```

input statement

```rs
// before

in("whats your name");

// now

input("whats your name");
```

- enchanced interpreter
- optimized statements
- better error handling

## 🐛 Fixed

- Fixed memory issues
- Fixed `include` statement
- Fixed parsing errors
- Fixed error handling errors
- Fixed environment timing errors
