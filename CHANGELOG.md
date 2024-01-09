## 2.0.0 - Jan 10

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