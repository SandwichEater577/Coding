# Lesson 4 — Control Flow

This is where Nes becomes a real programming language. You can use
`if`/`else`/`end` for decisions and `for`/`end` for loops.

---

## If / Else / End

Basic structure:

```nes
if <condition>
  # commands when true
end
```

With an else branch:

```nes
if <condition>
  # commands when true
else
  # commands when false
end
```

### Conditions

| Condition           | Meaning                    |
| ------------------- | -------------------------- |
| `exists <path>`     | file or directory exists   |
| `not exists <path>` | does NOT exist             |
| `$a == $b`          | strings are equal          |
| `$a != $b`          | strings are not equal      |
| `$a > $b`           | a is greater (numeric)     |
| `$a < $b`           | a is less (numeric)        |
| `$a >= $b`          | greater or equal (numeric) |
| `$a <= $b`          | less or equal (numeric)    |
| `not <condition>`   | negates any condition      |

### Examples

```nes
# Check if a file exists
if exists README.md
  echo README found!
else
  echo No README
end

# Compare variables
let lang = rust
if $lang == rust
  echo Great choice!
end

# Numeric comparison
let score = 85
if $score >= 90
  echo Grade: A
else
  echo Keep going!
end

# Negation
if not exists build
  mkdir build
  echo Created build directory
end
```

---

## For Loops

### Loop over items

```nes
for item in apple banana cherry
  echo Fruit: $item
end
```

### Loop over a number range

```nes
for i in range 1 5
  echo Count: $i
end
# Prints: Count: 1, Count: 2, ... Count: 5
```

### Loop over files in a directory

```nes
for file in files src
  echo Found: $file
end
```

### Loop over lines in a file

```nes
for line in lines todo.txt
  echo - $line
end
```

---

## Nesting Blocks

You can put `if` inside `for`, or `for` inside `if`:

```nes
for file in files .
  if exists $file
    typeof $file
  end
end
```

```nes
# Only loop if directory exists
if exists logs
  for f in files logs
    echo Log: $f
  end
else
  echo No logs directory
end
```

---

## Interactive Blocks

When you type `if` or `for` in the interactive shell, Nes shows
a `...>` continuation prompt until you type `end`:

```
C:\project nes> for i in range 1 3
 ...> echo Hello $i
 ...> end
Hello 1
Hello 2
Hello 3
```

---

## Exercise

1. Write an `if` that checks if `Cargo.toml` exists and prints a message
2. Write a `for` loop that prints numbers 1 to 10
3. Write a `for` loop that lists all files in the current directory
   and checks if each one is a `file` or `dir` using `typeof`
4. Combine them: loop over files, and only print the ones that are directories

---

## Practice Script

```nes
# lesson4.nes — Control flow

echo === Checking project structure ===

# Check required files
for f in Cargo.toml src README.md
  if exists $f
    echo [OK] $f
  else
    echo [MISSING] $f
  end
end

echo === Listing source files ===
if exists src
  for f in files src
    echo   - $f
  end
else
  echo No src/ directory!
end

echo === Counting ===
for i in range 1 5
  echo $i...
end
echo GO!
```

---

**Prev:** [Lesson 3 — Variables & Math](03-variables-and-math.md)
**Next:** [Lesson 5 — Real Scripts](05-real-scripts.md)
