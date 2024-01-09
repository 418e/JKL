# Control Flow

In the Tron, interpreter is reading content line by line, from top to bottom and this is called Flow. Flow can be controlled using logic gates and loops.

## If Statement

When you want to implement following logic: if `something` happens do `this`, you can use if statement:

```rs
let i = 10;

if i > 5 do
    print "i is more than 5";
end

// i is more than 5
```

In the example above, if `i` is more than `5`, "i is more than 5" will be printed. If you want to handle what happens if `i` isn't more than `5`, you can use `else` keyword:

```rs
let i = 3;

if i > 5 do
    print "i is more than 5";
end else do
    print "i isn't more than 5";
end

// i isnt't more than 5
```

Sometimes, you have to check multiple variations, to avoid writing if statement multiple types, we have `elif` keyword for that:

```rs
let i = 0;

if i > 0 do
    print "i is more than 0";
end else if i == 0 do
    print "i equals 0";
end else do
    print "i is less than 0";
end

// i equals 0
```

## while loop

If you can create loops using `while`:

```rs
let i = 0;

while i < 6 do
    print i;
    i = i + 1;
end

// 0
// 1
// 2
// 3
// 4
// 5
```

while `statement` is true codeblock is executing. You can also break the loop using `break` keyword.

```rs
let i = 0;
let j = 3;

while i < 6 do
    print i;

    if i == j do
        break;
    end

    i = i + 1;
end

// 0
// 1
// 3
```

## For loop

There also another way to create loop:

```rs
for let i = 0; i < 5; i = i + 1 do
   print i;
end
```

For loops work in the following way:

```
for statement1; statement2; statement3 {
    codeblock
}
```

Before loop starts, `statement1` executes, While loop runs `statement2` is being executed and when loop is finished, `statement3` is executed. It will continue while `statement2` will become true. Unfortunatly, we don't have `break` keyword in for loops. (for now)


read next: [Functions](./functions.md)