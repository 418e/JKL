# Todos

## features

- structures

```rs
struct User {
  name: String,
  age?: Integer,
}

let john = User {
  name: "john"
};

john.name; // "john"
```

- function `self` parameter

```rs
fn sqr(self){
    return self * self;
}
print 2.sqr(); // 4
```

- for iterables (for item in items{})
- inline if statement
- inline switch statement
- type statement
- enum statement
- implementations
- type casting

## std

- states

```rs
state("darkmode", false, bool); // declare state (type is optional)
set_state("darkmode", true); // change state value
get_state("darkmode"); // get state
del_state("darkmode"); // delete state
def_state("darkmode"); // change state to the default value
```

- latex

```rs
latex("a^2 + b^2 = c^2"); // returns latex
```

- type conversions
- array operations
- type specific operations
- type specific inputs
- colorfull outputs

## types

```ts
float: f32,
int: i32,
number: u32,
string: String,
char: char,
parsable: String,
bool: bool,
array: vec![],
T[]: Vec![T],
T: T,
null: null,
uknown: uknown
{param: T}: Struct{param: T},
```

## cli

```bash
trun run <filename> --bench --strict --ignore
tron version
tron latest
tron error <error_id>
tron keyword <keyword>
tron build <filename> --outFile --outDir --watch
tron new <projectname>
tron login
tron init
tron publish
```

## devx

- unused variable/fn error
- better extension (keyword completion and code formatting)
- automatic semicolon assignment
- better error messages
