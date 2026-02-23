# Lesson 1 — First Steps

## What is Nes?

Nes (the nestea shell) is a command-line shell for Windows, written in Rust.
It has 30+ built-in commands and its own scripting language with `.nes` files.

---

## Starting Nes

```
nes                   → shows usage
nes enter-full        → interactive shell
nes help              → all commands
nes echo hello        → run a single command
nes run script.nes    → run a script
```

---

## Your First Commands

Open the interactive shell:

```
nes enter-full
```

Try these:

```nes
echo Hello World
pwd
ls
whoami
date
time
os
```

- `echo` prints text
- `pwd` prints the current directory
- `ls` lists files and folders (folders are blue)
- `whoami` shows your username
- `date` and `time` show the current date/time
- `os` shows your operating system

---

## Navigation

```nes
cd Documents          # go into Documents
cd ..                 # go up one level
cd -                  # go back to previous directory
cd                    # go to home directory
pwd                   # print where you are
```

---

## Looking Around

```nes
ls             # compact listing
ll             # detailed listing (sizes)
tree           # visual tree of folders
tree src       # tree of a specific folder
```

---

## Exercise

1. Open `nes enter-full`
2. Run `whoami` — what's your username?
3. Run `ls` — what files do you see?
4. Use `cd` to go into a folder, then `cd -` to come back
5. Run `tree` to see the folder structure
6. Type `exit` to quit

---

## Practice Script

Create a file called `lesson1.nes`:

```nes
# lesson1.nes — My first Nes script
echo === System Info ===
whoami
hostname
os
date
time
echo === Current Directory ===
pwd
ls
echo === Done! ===
```

Run it with:

```
nes run lesson1.nes
```

---

**Next:** [Lesson 2 — Files & Text](02-files-and-text.md)
