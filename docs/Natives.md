# Natives

Natives are native libraries, you can include in your project:

## Included

|        Native        |            Description            |
| :------------------: | :-------------------------------: |
|     typeof(any)      |       returns type of value       |
| len(string or array) | returns length of string or array |
|    input(string)     |        request user input         |

```rs
print typeof(null);
// null
print typeof(true);
// boolean
print typeof(32);
// number
print typeof("hi");
// string
print typeof([1,2,3]);
// array
```

```rs
let name = input("whats your name brother");
print name + "? Cool name bro!";
```

## Math

```tron
include "#math";
```

|       Native       |           Description            |
| :----------------: | :------------------------------: |
|    sin(number)     |   returns sinus of the number    |
|    cos(number)     |  returns cosinus of the number   |
|    tan(number)     |   returns tanges of the number   |
|    asin(number)    |  returns arcsinus of the number  |
|    acos(number)    | returns arccosinus of the number |
|    atan(number)    | returns arctanges of the number  |
|   round(number)    |      returns rounded number      |
|   floor(number)    |     returnes floored number      |
| to_degrees(number) |   converts number into degrees   |
| to_radians(number) |   converts number into radians   |

```rs
let sin30 = sin(30);
let cos30 = cos(30);
let tan30 = tan(30);
let asin30 = asin(30);
let acos30 = acos(30);
let atan30 = atan(30);
let floored = floor(0.5);
let rounded = round(0.5)
let degin30 = to_degrees(30);
let radin30 = to_radians(30);
```

## Array

```tron
include "#array";
```

|       Native        |           Description            |
| :-----------------: | :------------------------------: |
|  push(array, any)   | pushes new element to the array  |
| join(array, string) |   returns new joined elemement   |
|     pop(array)      | removes last element from array  |
|    shift(array)     | removes first element from array |

```rs
let x = [1,2,3];

print len(x);
// 3
```

```rs
let x = [1,2,3];
x = push(x, 4);
print x;

// [1,2,3,4]
```

```rs
let x = [1,2,3,4];
print join(x, "_");

// 1_2_3_4
```

```rs
let x = [1,2,3,4];
x = pop(x);
x = shift(x);

print x;
// [2, 3]
```
