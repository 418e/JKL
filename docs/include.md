# Include

Sometimes project gets larger, and containing all the code inside one file becomes hard to managed. Only solution is to distribute your code across different files.

```rs
// say.tron

fn say(string) do
    print string;
end
```

```rs
// main.tron
include "say.tron";

say("Hello world!")

// hello world!
```

It is highly recomended to write `include` statement at the top of the file.

```rs
// main.tron


say("Hello world!")

include "say.tron";


// say isn't declared
```

read next: [Input/Output](./IO.md)
