```diff
E0000: System Errors
- E0001: failed to run file
- E0002: failed to run command
- E0003: unsupported platform
E1000: Scanner Errors
- E1001: unterminated string
- E1002: unrecognized character: {character}
- E1003: unsupported character: {character}
- E1004: failed to scan tokens: \n {message}
E2000: Parser Errors
- E2001: failed to parse statements: \n {message}
- E2002: failed to parse block statement
- E2003: unexpected token: {token | message}
- E2004: function can't have more than 32 arguments
- E2005: invalid assignment target
E3000: Resolver Errors
- E3001: failed to resolve {statement_name} statement: incorrect type
- E3002: variable {variable_name} already exists
- E3003: failed to read local variable
- E3004: failed to resolve a variable in a too deep level
- E3005: failed to define a variable in a too deep level
- E3006: return isn't allowed outside of a function
- E3007: break isn't allowed outside of a loop
E4000: Interpreter Errors
- E4001: {function_name}() is expecting {arity} arguments, but got {args.len}
- E4002: {function_name}({arg_name}: {arg_type})
- E4003: {statement} {name} is expecting {value_type} type, but got {type}
- E4004: failed to execute command: \n {message}
- E4005: failed to find library: {library}
- E4006: failed to make function
- E4007: failed to unwrap {unwraping_value} as {unwrap_target}
- E4008: failed to create type from {invalid_type}
- E4009: array index is out of bounds
- E4010: failed to perform operation on array
- E4011: variable {variable_name} has not been declared
- E4012: immutable variables can't be re-declared
- E4013: failed to call
- E4014: function call argument count doesn't match parameter count
- E4015: {operator} is not implemented for {target}
- E4016: invalid operator: {operator}
- E4017: invalid function output type
- E4018: {function} requires at least {arguments} arguments   --- throw
- E4019: {function} requires more than {arguments} arguments --- throw
- E4020: {function} requires exactly {arguments} arguments --- throw
- E4021: {function} expects {type} type as {argument} argument --- throw
```
