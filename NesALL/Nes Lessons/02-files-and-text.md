# Lesson 2 — Files & Text

## Creating Files

```nes
touch notes.txt         # create an empty file
mkdir projects          # create a directory
mkdir -p a/b/c          # create nested directories
```

## Writing to Files

Use `>` to write (overwrites) and `>>` to append:

```nes
echo Hello > greeting.txt        # write "Hello" to file
echo World >> greeting.txt       # append "World" to file
```

---

## Reading Files

```nes
cat greeting.txt        # print entire file
head 5 log.txt          # first 5 lines
tail 5 log.txt          # last 5 lines
wc greeting.txt         # count lines, words, bytes
hex greeting.txt        # hex dump (for binary files)
```

---

## Copying, Moving, Deleting

```nes
cp greeting.txt backup.txt       # copy
mv backup.txt archive.txt        # rename/move
rm archive.txt                   # delete file
rm old_folder                    # delete folder (recursive)
```

---

## Searching

```nes
grep hello greeting.txt          # find lines containing "hello"
find main.rs                     # find files by name
```

- `grep` highlights matches in red
- `find` searches recursively from the current directory

---

## File Info

```nes
size .                   # size of current directory
size src                 # size of a specific folder
exists greeting.txt      # prints "true" or "false"
typeof greeting.txt      # prints "file", "dir", or "none"
count .                  # number of items in directory
```

---

## Chaining & Redirects

```nes
# Run two commands in sequence (second runs only if first succeeds)
mkdir build && echo Build directory created

# Redirect output to a file
ls > filelist.txt

# Append output to a file
date >> log.txt

# Pipe one command into another
echo hello | grep hell
```

---

## Exercise

1. Create a folder called `test_area`
2. Inside it, create a file `notes.txt` with some text using `echo > `
3. Append a second line with `echo >>`
4. Read it with `cat`
5. Check its word count with `wc`
6. Copy it to `backup.txt`
7. Clean up: delete the whole `test_area` folder

---

## Practice Script

```nes
# lesson2.nes — File operations
mkdir workspace
echo Project started > workspace/readme.txt
echo Version 1.0 >> workspace/readme.txt
cat workspace/readme.txt
wc workspace/readme.txt
size workspace
echo Cleaning up...
rm workspace
echo Done!
```

---

**Prev:** [Lesson 1 — First Steps](01-first-steps.md)
**Next:** [Lesson 3 — Variables & Math](03-variables-and-math.md)
