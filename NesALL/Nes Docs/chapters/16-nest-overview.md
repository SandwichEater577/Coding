# Chapter 16 · NesT Overview

[← Examples](15-examples.md) · [Next: NesT Functions & Built-ins →](17-nest-builtins.md)

---

## What is NesT?

**NesT** (Nes Typed) is the programming language side of Nes. While NesC is a shell for running commands, NesT is a proper language for writing programs — with types, expressions, functions, and structured control flow.

NesT files use the `.nest` extension and are run with:

```
nes run program.nest
```

## NesC vs NesT

| Feature         | NesC (.nes)                 | NesT (.nest)                 |
| --------------- | --------------------------- | ---------------------------- |
| Purpose         | Shell commands & scripting  | Programming                  |
| Variables       | `let name = hello`          | `let name = "hello";`        |
| Types           | Everything is a string      | int, float, str, bool        |
| Blocks          | `if`...`end`, `for`...`end` | `if` `{` `}`, `for` `{` `}`  |
| Semicolons      | No                          | Yes                          |
| Functions       | No                          | `fn name(args) { }`          |
| Expressions     | `calc` command              | Native: `let x = 2 + 3 * 4;` |
| String concat   | `$variable` expansion       | `"a" + "b"` operator         |
| I/O             | `echo`, `read`              | `println()`, `input()`       |
| System commands | Yes (30+ built-ins)         | No (pure language)           |

## Types

NesT has four value types plus `none`:

| Type    | Description           | Examples                |
| ------- | --------------------- | ----------------------- |
| `int`   | 64-bit signed integer | `0`, `42`, `-7`, `1000` |
| `float` | 64-bit float          | `3.14`, `0.5`, `-2.7`   |
| `str`   | UTF-8 string          | `"hello"`, `"it's me"`  |
| `bool`  | Boolean               | `true`, `false`         |
| `none`  | Absence of value      | (returned by `print()`) |

### Type Conversions

```nest
let x = int("42");       # str → int
let y = float("3.14");   # str → float
let s = str(100);        # int → str
let t = type(x);         # → "int"
```

## Variables

Variables are declared with `let` and reassigned with `=`:

```nest
let count = 0;
let name = "Nes";
let pi = 3.14159;
let active = true;

count = count + 1;     # reassign
name = "NesT";         # reassign
```

Variables are block-scoped — a variable declared inside `{ }` is not visible outside.

## Operators

### Arithmetic

| Op  | Description | Example  | Result |
| --- | ----------- | -------- | ------ |
| `+` | Add         | `3 + 2`  | `5`    |
| `-` | Subtract    | `10 - 4` | `6`    |
| `*` | Multiply    | `6 * 7`  | `42`   |
| `/` | Divide      | `15 / 4` | `3`    |
| `%` | Modulo      | `17 % 5` | `2`    |

**Note:** Division between two `int` values produces an `int` (truncated). If either operand is a `float`, the result is a `float`.

### Comparison

| Op   | Description      | Example  | Result  |
| ---- | ---------------- | -------- | ------- |
| `==` | Equal            | `5 == 5` | `true`  |
| `!=` | Not equal        | `5 != 3` | `true`  |
| `<`  | Less than        | `3 < 10` | `true`  |
| `>`  | Greater than     | `10 > 3` | `true`  |
| `<=` | Less or equal    | `5 <= 5` | `true`  |
| `>=` | Greater or equal | `5 >= 6` | `false` |

### Logical

| Op     | Description | Example           | Result  |
| ------ | ----------- | ----------------- | ------- |
| `&&`   | And         | `true && false`   | `false` |
| `\|\|` | Or          | `true \|\| false` | `true`  |
| `!`    | Not         | `!false`          | `true`  |

### String Concatenation

The `+` operator concatenates strings:

```nest
let full = "Hello" + ", " + "World!";
println(full);   # Hello, World!
```

To concatenate a number with a string, convert it first:

```nest
let msg = "Score: " + str(95);
```

## Control Flow

### If / Else

```nest
if condition {
    # true branch
} else {
    # false branch
}
```

Nested:

```nest
if score >= 90 {
    println("A");
} else {
    if score >= 80 {
        println("B");
    } else {
        println("C");
    }
}
```

### For Loop

For loops iterate over an integer range using `..` (exclusive upper bound):

```nest
for i in 0..5 {
    println(i);    # prints 0, 1, 2, 3, 4
}
```

The bounds can be expressions:

```nest
let n = 10;
for i in 0..n {
    println(i);
}
```

### While Loop

```nest
let count = 5;
while count > 0 {
    println(count);
    count = count - 1;
}
```

### Break and Continue

```nest
for i in 0..100 {
    if i == 10 {
        break;       # exit loop
    }
    if i % 2 == 0 {
        continue;    # skip even numbers
    }
    println(i);
}
```

## Comments

```nest
# This is a comment
// This is also a comment
let x = 42;  # inline comment
```

## Directives

Directives are optional key-value settings at the top of a `.nest` file. They configure interpreter behavior:

```nest
memoryAuto = true;

# your code starts here
let x = 10;
```

Directives are parsed before any code runs.

---

[← Examples](15-examples.md) · [Next: NesT Functions & Built-ins →](17-nest-builtins.md)
