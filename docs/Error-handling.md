# Error-handling

Errors happen, it is inevitable, but by easily spoting and handling errors will reduce stress and risk of bugs.


## Try/Catch

Try/Catch is used when you want to check if something causes any errors:

```rs
try do
    somefunction();
end catch do
    print "somefunction() failed to run.";
end
```

If `somefunction()` contains any errors, catch block will be executed instead of try block.

## Panic

If you want to exist appliaction with custom message, you can use `panic` statement:

```rs
try do
    somefunction();
end catch do
    panic "somefunction() failed to run.";
end
```

Panic leaves the message and then closes the application.

## Exit

If you want to exit without any messages, you can use `exit` keyword:

```rs
try do
    somefunction();
end catch do
    exit;
end
```

Please note that `exit` and `panic` can also be used outside of try/catch statement.

read next: [Include](./include.md)