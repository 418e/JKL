# Functions

Functions are defined in Tron using `fn` keywoard with following syntax:

```rs
fn functionname(arguments) start
    codeblock
end
```

For example:

```rs
fn myFunction() start
    print 5;
end
```

## Calling funcitons

When function `myFunction` is called, "5" will be printed. Functions are called by writting function name with closed brackets:

```rs
fn myFunction() start
    print 5;
end

myFunction();

// 5
```

## Return statement

Functions return values using `return` keyword:

```rs
fn myFunction() start
    return 5;
end

print myFunction();

// 5
```
Every time `myFunction()` is called, it will return "5";


## Passing arguments

Arguments are placed inside brackets and are separated by comma (`,`):

```rs
fn sum(x, y) do
    return x + y;
end

print sum(1, 2);

// 3
```

Don't forget to pass arguments when calling the function.

## Name Rules

Function names can only have alphanumeric characters (latin characters and numbers) or underscore (`_`). Some words are reserved, including `Native` functions, you will learn more about them in the [`Native`](./Natives.md) chapter.

read next: [Error Handling](./Error-handling.md)






