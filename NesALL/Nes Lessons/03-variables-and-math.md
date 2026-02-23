# Lesson 3 — Variables & Math

## Creating Variables

Use `let` to create shell variables:

```nes
let name = Nes
let version = 4.0
let greeting = Hello World
```

## Using Variables

Prefix with `$` to use them:

```nes
let name = Michal
echo Hello $name
# → Hello Michal

let dir = src
ls $dir
# → lists contents of src/
```

Variables are expanded everywhere — in commands, arguments, even file paths.

---

## Environment Variables

Shell variables (`let`) are local to Nes. To set real environment variables:

```nes
export EDITOR = code        # set env variable
set MY_VAR = 42             # set env-only variable
env                         # list all env variables
unset myvar                 # remove a shell variable
```

Check a variable exists:

```nes
echo $PATH                  # prints PATH env variable
echo $USERNAME              # prints your username
```

If a `$name` isn't found in shell vars, Nes checks environment variables automatically.

---

## Reading User Input

```nes
read name
# (waits for you to type something and press Enter)
echo You typed: $name
```

This is most useful in scripts:

```nes
# ask.nes
echo What is your name?
read name
echo Nice to meet you, $name!
```

---

## Math with calc

The `calc` command evaluates math expressions:

```nes
calc 2 + 3          # → 5
calc 10 * 5         # → 50
calc 100 / 3        # → 33.333...
calc 2 ^ 10         # → 1024
calc (5 + 3) * 2    # → 16
calc 17 % 5         # → 2 (modulo)
```

Supports: `+` `-` `*` `/` `^` (power) `%` (modulo) and parentheses.

---

## Aliases

Create shortcuts for long commands:

```nes
alias ll = ls -l
alias g = grep
alias cls = clear

# List all aliases
alias

# Now use them
g hello file.txt
```

---

## History

```nes
history               # show all commands you've typed
```

---

## Exercise

1. Create a variable `myname` with your name
2. Create a variable `age` with your age
3. Print `Hello, <name>! You are <age> years old.`
4. Use `calc` to compute: `(age * 365)` — approximate days alive
5. Create an alias `h` for `history`

---

## Practice Script

```nes
# lesson3.nes — Variables and math

let pi = 3.14159
let radius = 5

echo Circle Calculator
echo Radius: $radius

# calc uses raw expressions, not variables (yet)
# but you can build commands:
calc $radius * $radius * $pi

echo ---
echo Available aliases:
alias greet = echo Hello from Nes!
alias
greet
```

---

**Prev:** [Lesson 2 — Files & Text](02-files-and-text.md)
**Next:** [Lesson 4 — Control Flow](04-control-flow.md)
