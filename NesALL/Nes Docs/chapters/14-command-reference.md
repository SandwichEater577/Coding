# Chapter 14 · Command Reference

[← System Fallback](13-system-fallback.md) · [Next: Examples →](15-examples.md)

---

Quick-lookup table of every built-in command and language feature.

---

# Part A — NesC (Shell)

## Navigation — 7 commands

| Command | Syntax           | Description                                     |
| ------- | ---------------- | ----------------------------------------------- |
| `cd`    | `cd [dir]`       | Change directory. No arg = home. `-` = previous |
| `ls`    | `ls [dir]`       | List directory (colored, sorted)                |
| `ll`    | `ll [dir]`       | Long listing with sizes                         |
| `pwd`   | `pwd`            | Print working directory                         |
| `tree`  | `tree [dir]`     | Visual directory tree                           |
| `find`  | `find [pattern]` | Recursive file search (substring)               |
| `which` | `which <name>`   | Locate executable on PATH                       |

## File Operations — 11 commands

| Command | Syntax            | Description                  |
| ------- | ----------------- | ---------------------------- |
| `cat`   | `cat <file>`      | Display file contents        |
| `head`  | `head [n] <file>` | First N lines (default 10)   |
| `tail`  | `tail [n] <file>` | Last N lines (default 10)    |
| `wc`    | `wc <file>`       | Count lines, words, bytes    |
| `touch` | `touch <file>`    | Create empty file            |
| `mkdir` | `mkdir <path>`    | Create directory (recursive) |
| `rm`    | `rm <path>`       | Delete file or directory     |
| `cp`    | `cp <src> <dst>`  | Copy file                    |
| `mv`    | `mv <src> <dst>`  | Move / rename                |
| `hex`   | `hex <file>`      | Hex dump (first 512 bytes)   |
| `size`  | `size <path>`     | Human-readable size          |

## Text — 2 commands

| Command | Syntax                  | Description                    |
| ------- | ----------------------- | ------------------------------ |
| `echo`  | `echo <text>`           | Print text                     |
| `grep`  | `grep <pattern> <file>` | Search file, highlight matches |

## System — 8 commands

| Command    | Syntax          | Description                |
| ---------- | --------------- | -------------------------- |
| `whoami`   | `whoami`        | Current username           |
| `hostname` | `hostname`      | Computer name              |
| `os`       | `os`            | OS and architecture        |
| `env`      | `env`           | List environment variables |
| `time`     | `time`          | Current date & time        |
| `date`     | `date`          | Current date               |
| `open`     | `open <path>`   | Open with system default   |
| `clear`    | `clear` / `cls` | Clear screen               |

## Shell — 8 commands

| Command   | Syntax               | Description                   |
| --------- | -------------------- | ----------------------------- |
| `let`     | `let name = value`   | Set shell variable            |
| `set`     | `set [key=val]`      | List or set env variable      |
| `unset`   | `unset <name>`       | Remove shell variable         |
| `export`  | `export key=val`     | Set shell + env variable      |
| `alias`   | `alias [name = cmd]` | Define or list aliases        |
| `history` | `history`            | Show command history          |
| `run`     | `run <file>`         | Execute a .nes or .nest file  |
| `read`    | `read <varname>`     | Read user input into variable |

## Control Flow — 6

| Feature  | Syntax                          | Description              |
| -------- | ------------------------------- | ------------------------ |
| `if`     | `if <condition>` ... `end`      | Conditional block        |
| `else`   | `else`                          | Alternate branch         |
| `for`    | `for <var> in <iter>` ... `end` | Loop over files/range    |
| `sleep`  | `sleep <seconds>`               | Pause execution          |
| `exists` | `exists <path>`                 | Check if file/dir exists |
| `count`  | `count <path>`                  | Count items in directory |
| `typeof` | `typeof <varname>`              | Print type of variable   |

## Math — 1 command

| Command | Syntax        | Description              |
| ------- | ------------- | ------------------------ |
| `calc`  | `calc <expr>` | Evaluate math expression |

## Exit — 2 commands

| Command | Syntax | Description    |
| ------- | ------ | -------------- |
| `exit`  | `exit` | Exit the shell |
| `quit`  | `quit` | Exit the shell |

## Operators

| Operator | Syntax               | Description                 |
| -------- | -------------------- | --------------------------- |
| Chain    | `cmd1 && cmd2`       | Run commands in sequence    |
| Pipe     | `cmd1 \| cmd2`       | Connect stdout → stdin      |
| Write    | `cmd > file`         | Redirect output (overwrite) |
| Append   | `cmd >> file`        | Redirect output (append)    |
| Quote    | `"text"` or `'text'` | Group words as one argument |

**NesC Total: 38+ built-in commands + 5 operators**

---

# Part B — NesT (Language)

Run NesT programs with `nes run <file.nest>`.

## Types

| Type    | Example         | Description           |
| ------- | --------------- | --------------------- |
| `int`   | `42`, `-7`      | 64-bit signed integer |
| `float` | `3.14`, `0.5`   | 64-bit float          |
| `str`   | `"hello"`       | String (UTF-8)        |
| `bool`  | `true`, `false` | Boolean               |

## Variable Declarations

```nest
let x = 10;
let name = "Nes";
x = x + 1;        // reassign
```

## Operators

| Category   | Operators                   |
| ---------- | --------------------------- |
| Arithmetic | `+` `-` `*` `/` `%`         |
| Comparison | `==` `!=` `<` `>` `<=` `>=` |
| Logical    | `&&` `\|\|` `!`             |
| String     | `+` (concatenation)         |

## Control Flow

```nest
// if / else
if x > 5 {
    println("big");
} else {
    println("small");
}

// for loop (range)
for i in 0..10 {
    println(i);
}

// while loop
while count > 0 {
    count = count - 1;
}
```

## Functions

```nest
fn add(a, b) {
    return a + b;
}

let result = add(3, 7);
```

## Built-in Functions — 13

| Function     | Description           |
| ------------ | --------------------- |
| `print(x)`   | Print without newline |
| `println(x)` | Print with newline    |
| `input()`    | Read line from stdin  |
| `len(s)`     | String length         |
| `type(x)`    | Type name as string   |
| `str(x)`     | Convert to string     |
| `int(x)`     | Convert to int        |
| `float(x)`   | Convert to float      |
| `abs(x)`     | Absolute value        |
| `sqrt(x)`    | Square root           |
| `min(a, b)`  | Minimum of two values |
| `max(a, b)`  | Maximum of two values |
| `pow(a, b)`  | a raised to power b   |

## Directives

Directives are optional settings placed at the top of a `.nest` file, before any code:

```nest
memoryAuto = true;
```

They configure the interpreter's behavior for that file.

## Comments

```nest
# This is a comment
// This is also a comment
```

## Flow Control Keywords

| Keyword    | Description                    |
| ---------- | ------------------------------ |
| `return`   | Return a value from a function |
| `break`    | Exit the current loop          |
| `continue` | Skip to next loop iteration    |

---

[← System Fallback](13-system-fallback.md) · [Next: Examples →](15-examples.md)
