# Lesson 5 — Real Scripts

Now you know all the pieces. Let's put them together into real,
useful `.nes` scripts.

---

## Script Structure

A good Nes script has:

```nes
# 1. Description comment at the top
# 2. Variable setup
# 3. Logic (commands, if/for blocks)
# 4. Cleanup / final output
```

---

## Example 1: Project Scaffolder

Creates a new project directory with standard folders:

```nes
# scaffold.nes — Create a project structure

echo What is your project name?
read name

mkdir $name
mkdir $name/src
mkdir $name/docs
mkdir $name/tests

echo # $name > $name/README.md

echo Project "$name" created!
echo Structure:
tree $name
```

Run: `nes run scaffold.nes`

---

## Example 2: File Backup

Copies important files into a backup folder:

```nes
# backup.nes — Back up source files

let backup_dir = backup
mkdir $backup_dir

for f in files src
  cp src/$f $backup_dir/$f
  echo Backed up: $f
end

echo === Backup complete ===
count $backup_dir
echo files backed up.
```

---

## Example 3: System Report

Generates a report and saves it to a file:

```nes
# report.nes — System report generator

let out = report.txt

echo System Report > $out
echo ============= >> $out
echo User: >> $out
whoami >> $out
echo Host: >> $out
hostname >> $out
echo OS: >> $out
os >> $out
echo Date: >> $out
date >> $out
echo Time: >> $out
time >> $out
echo >> $out
echo Files in current directory: >> $out
ls >> $out

echo Report saved to $out
cat $out
```

---

## Example 4: Health Checker

Checks if required files exist in a project:

```nes
# healthcheck.nes — Verify project integrity

let errors = 0

for f in Cargo.toml src/main.rs README.md
  if not exists $f
    echo [FAIL] Missing: $f
    let errors = 1
  else
    echo [ OK ] $f
  end
end

if $errors == 1
  echo Some files are missing!
else
  echo All files present.
end
```

---

## Example 5: Quick Math Table

```nes
# multiply.nes — Print a multiplication table

echo Multiplication table (1-5):
echo

for i in range 1 5
  for j in range 1 5
    calc $i * $j
  end
end
```

---

## Tips

1. **Comments** — Use `#` liberally. Future you will thank you.
2. **Variables** — Set up data at the top of your script.
3. **Existence checks** — Always check with `if exists` before reading files.
4. **Keep it simple** — Nes is best for quick automation. For complex logic, use a full language.
5. **System fallback** — Any command Nes doesn't know gets passed to `cmd.exe`, so you can use `git`, `cargo`, `python`, etc. in scripts.

---

## Challenge

Write a script called `organize.nes` that:

1. Creates folders: `docs/`, `images/`, `code/`
2. Lists all files in the current directory
3. For each file, uses `typeof` to check if it's a file
4. Prints a summary with `count` for each folder
5. Saves the summary to `organize-report.txt`

---

## Command Reference Card

| Category | Commands                                                             |
| -------- | -------------------------------------------------------------------- |
| Navigate | `cd` `ls` `ll` `pwd` `tree` `find` `which`                           |
| Files    | `cat` `head` `tail` `wc` `touch` `mkdir` `rm` `cp` `mv` `hex` `size` |
| Text     | `echo` `grep`                                                        |
| System   | `whoami` `hostname` `os` `env` `time` `date` `open` `clear`          |
| Shell    | `let` `set` `unset` `export` `alias` `history` `run` `read`          |
| Control  | `if`/`else`/`end` `for`/`end` `sleep` `exists` `count` `typeof`      |
| Math     | `calc`                                                               |
| Flow     | `&&` `>` `>>` `\|`                                                   |

---

**Prev:** [Lesson 4 — Control Flow](04-control-flow.md)

Congratulations — you've completed all 5 lessons! You now know how to use
Nes interactively and write `.nes` scripts. Go build something cool.
