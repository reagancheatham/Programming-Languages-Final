Olive Oil (or just Oil for short) is a language implemented with Rust based on Robert Nystrom's [_Crafting Interpreters_](https://craftinginterpreters.com/).

As Oil is implemented in Rust, it is run by simply calling 'cargo run' in terminal. If a parameter is passed, then Oil will attempt to execute the contents of the passed file (i.e. `cargo run -- hello_word.txt`). 

**File locations are relative to the cargo root.**

|Keyword|Description|Example|
|-------|-----------|-------|
|`true`|Literal for boolean true|Trivial|
|`false`|Literal for boolean false|Trivial|
|`null`|Literal for a null/nothing value|Trivial|
|`if`|Performs a logical branch based on the truthiness of its condition|`if (5 > 3) { ... }`|
|`else`|Defines a second logical branch to execute if an `if` statement is false|`if (5 > 3) { ... } else { ... }`|
|`and`|Performs a logical `AND` on the truthiness values to its left and right|`true and false;` (evaluates to false)|
|`or`|Performs a logical `OR` on the truthiness values to its left and right|`true or false;` (evaluates to true)|
|`var`|Defines a variable. Optionally accepts an initializing expression|`var a = 5;`|
|`while`|Defines a loop that continues until its condition is false|`while (input != "Hello!") { ... }`|
|`for`|Defines a loop with sugar for an initializer, condition, and increment|`for (var i = 0; i < target; i = i + 1) { ... }`|
|`read_input`|Reads input from standard input into a variable. Pauses execution until received|`var a; read_input a;`|
|`print`|Prints a value into standard output|`print 5 + 10` (prints 15)|

Other important notes:
----------------------
- Oil does not have types. `var` defines a dynamic variable that is evaluated at runtime.
- All values in Oil have truthiness:
  - `true` = true
  - `false` = false
  - `null` = false
  - any string = true
  - any number = true
- Oil does not see whitespace or newlines. Multiple statements can be put on one line.
- If the branch of an `if` statement is one line, brackets can be excluded.
- `for` statements have three components, and any of them can be excluded:
  - |Component|Description|Example|
    |---------|-----------|-------|
    |`initializer`|Initializes a var to use in the loop|`for(var i = 0; ; ;) { ... }`|
    |`condition`|Defines the condition for when to stop the loop|`for(; counter > 5; ;) { ... }`|
    |`increment`|Performs an operation at the end of each loop|`for(; ; counter = counter + 1) { ... }`|
- Strings can be negated to reverse them i.e.: `print -"hello"; // outputs 'olleh'`
