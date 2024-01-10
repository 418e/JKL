# 2.1.0 - Jan 10

### ✨ Syntatic Sweetness

Developers can now choose, which keyword they want to use:

| name | variants |
| :- | :- |
| function | fn, fun, function, def, define |
| block start | `{`, do, start, doing, begin, `:` |
| block end | `}`, end, done, stop, `.` |
| if | if, `?` |  
| else | else, otherwise, if, nor, `?>` |  
| elif | else if, elif, what if, whatif, ?>> |  
| true | affirmative, true, yes |  
| false | negative, false, falsy, no |  
| null | null, nil |  
| print | print, say, shout, log, out, output, tell |  
| input | input, in, inp |  
| error | panic, alaram, throw, error, err |  
| import | include, import, require, use, payload, unload, lib |  
| exit | exit, kill, terminate |  
| return | return, respond, append |  
| variable | let, var, const, state, declare, dec, `$` |  
| while | while, loop |  
| bench | bench, test, measure, time |  
| plus | plus, `+` |  
| minus | minus, `-` |  
| multiply | multiply, multiplied by, `*` |  
| divide | divide, divided by, `/` |  
| assign | `=`, `assign`, `equal`, `equals`, `is`, `are`, `assigned to`, |  
| comments | // or # |

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
