# Data types and Variables

In Tron there are two types of data types: simples and complexs. Simple data types have simple structures, while complexs one tend to be little bit more complex.

## Simples

We have 4 simple data types:

### Strings

String is set of characters, surrounded with `"` characters:

```rs
let string = "Hello, World";
```

You can join strings by `+` operator:

```rs
let name = "John";

print "Hello " + name;

// Hello John
```

### Numbers

Numbers can be integers and floating numbers but 32 bit sized floating numbers are always returned even if you type integer.

```
let number = 2;
```

You can perfom arithmetical calculations on numbers:

```
print 3 + 2;
print 4 + 4 * 2;

// 5
// 12
```

Tron also handles grouping:

```print
print 2 * (5+3) + 12;

// 28
```

You can perform various operations on them using [Native functions](./Natives.md), you will learn more about them in [Math](./Math.md) and [Native function](./Natives.md) chapters.

### Booleans

Booleans have only 2 state : true or false.

```rs
let truevalue = true;
let falsevalue = false;

print truevalue;
print falsevalue;

// true
// false
```

### Nulls

Empty values are called Nulls, but you will rarely meet them.

## Complexs

For now, we only have one Complex type: Array

### Arrays

Arrays are set of elements seperated by `,`:

```rs
let array = [1,2,3,4,5,6];
```

You can call element inside array by their index:

```rs
let array = [1,2,3,4,5,6];

print array[0];

// 1
```

Just like in the most of programming languages, indexing starts with 0 in tron.

read next: [Comments](./comments.md)
