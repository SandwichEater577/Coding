# Chapter 17 · NesT Functions & Built-ins

[← NesT Overview](16-nest-overview.md) · [Next: NesT Examples →](18-nest-examples.md)

---

## User-Defined Functions

Functions are declared with `fn`, take parameters, and can return values:

```nest
fn add(a, b) {
    return a + b;
}

let result = add(3, 7);
println(result);   # 10
```

### Rules

- Functions must be defined **before** they are called (no hoisting)
- Parameters are passed by value
- Functions without a `return` statement return `none`
- Functions can call other functions, including themselves (recursion)

### Recursion

```nest
fn factorial(n) {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

println(factorial(10));   # 3628800
```

### Multiple Parameters

```nest
fn clamp(value, lo, hi) {
    if value < lo { return lo; }
    if value > hi { return hi; }
    return value;
}

println(clamp(150, 0, 100));   # 100
```

### No Return Value

```nest
fn greet(name) {
    println("Hello, " + name + "!");
}

greet("World");   # Hello, World!
```

---

## Built-in Functions — 13

### I/O — 3 functions

#### `print(value)`

Prints a value **without** a trailing newline.

```nest
print("Loading");
print("...");
print("done!\n");
```

#### `println(value)`

Prints a value **with** a trailing newline.

```nest
println("Hello, World!");
println(42);
println(3.14);
```

#### `input()`

Reads a line of text from the user. Returns a `str`.

```nest
print("Enter your name: ");
let name = input();
println("Hello, " + name + "!");
```

---

### Conversion — 4 functions

#### `str(value)`

Converts any value to its string representation.

```nest
let s = str(42);        # "42"
let t = str(3.14);      # "3.14"
let u = str(true);      # "true"
```

#### `int(value)`

Converts a string or float to an integer. Fails on invalid input.

```nest
let x = int("42");      # 42
let y = int(3.9);       # 3
```

#### `float(value)`

Converts a string or int to a float.

```nest
let x = float("3.14");  # 3.14
let y = float(42);      # 42.0
```

#### `type(value)`

Returns the type name as a string.

```nest
println(type(42));       # "int"
println(type("hi"));    # "str"
println(type(3.14));    # "float"
println(type(true));    # "bool"
```

---

### Inspection — 1 function

#### `len(string)`

Returns the length of a string (number of bytes).

```nest
println(len("hello"));  # 5
println(len(""));        # 0
```

---

### Math — 5 functions

#### `abs(x)`

Absolute value.

```nest
println(abs(-7));    # 7
println(abs(3));     # 3
```

#### `sqrt(x)`

Square root. Returns an `int` if the result is a perfect square, otherwise a `float`.

```nest
println(sqrt(16));   # 4
println(sqrt(2));    # 1.4142135623730951
```

#### `min(a, b)`

Returns the smaller of two values.

```nest
println(min(3, 7));     # 3
println(min(-1, -5));   # -5
```

#### `max(a, b)`

Returns the larger of two values.

```nest
println(max(3, 7));     # 7
println(max(-1, -5));   # -1
```

#### `pow(base, exp)`

Raises `base` to the power `exp`.

```nest
println(pow(2, 8));     # 256
println(pow(10, 3));    # 1000
println(pow(2.5, 2));   # 6.25
```

---

## Summary Table

| Function     | Args | Returns       | Description           |
| ------------ | ---- | ------------- | --------------------- |
| `print(x)`   | 1    | `none`        | Print without newline |
| `println(x)` | 1    | `none`        | Print with newline    |
| `input()`    | 0    | `str`         | Read line from stdin  |
| `len(s)`     | 1    | `int`         | String length         |
| `type(x)`    | 1    | `str`         | Type name             |
| `str(x)`     | 1    | `str`         | Convert to string     |
| `int(x)`     | 1    | `int`         | Convert to int        |
| `float(x)`   | 1    | `float`       | Convert to float      |
| `abs(x)`     | 1    | `int`/`float` | Absolute value        |
| `sqrt(x)`    | 1    | `int`/`float` | Square root           |
| `min(a, b)`  | 2    | `int`/`float` | Minimum               |
| `max(a, b)`  | 2    | `int`/`float` | Maximum               |
| `pow(a, b)`  | 2    | `int`/`float` | Exponentiation        |

---

[← NesT Overview](16-nest-overview.md) · [Next: NesT Examples →](18-nest-examples.md)
