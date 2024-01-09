# Include

Sometimes project gets larger, and containing all the code inside one file becomes hard to managed. Only solution is to distribute your code across different files.

```rs
// say.tron

print "hello world!";
```

```rs
// main.tron

include "/say.tron";

print "hello to you to";

// hello world!
// hello to you to
```

No mater where you use `include` it will execute content inside it:

```rs
// main.tron

print "hello to you to";

include "/say.tron";


// hello to you to
// hello world!
```

read next: [Input/Output](./IO.md)