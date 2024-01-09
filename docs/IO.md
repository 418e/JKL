# Input/Output

I/O is essential part of every programming language. Taking user inputs and making outputs is the we talk to computers, and computers to us.


## Input

There are two ways you can take user input in tron:

### `in` statement

```rs
in "Your name please.";
```

It prints "Your name please." and asks user to enter text in the terminal. Bad part about this statement is that it doesnt save the data user inputs.

### `input()` Native

if you want to save input provided by user you can use `input()` Native function:

```rs
let name = input("Your name please.");

print name;

// whatever name you enter
```

## Output

### Print

```rs
print "hi";
```

You have already seen this statement a lot. All it does is that it prints its' value in the terminal 


read next: [Math](./Math.md)