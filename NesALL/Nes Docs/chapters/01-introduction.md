# Chapter 1 · Introduction

[← Table of Contents](../README.md) · [Next: Installation →](02-installation.md)

---

## What is Nes?

**Nes** (short for _Nestea_) is a custom command-line shell and programming language for Windows, built from scratch in Rust. It combines the familiarity of Unix-style commands (`ls`, `cat`, `grep`, `tree`) with native Windows integration — all in a single, fast, portable binary.

Nes v5.1 has two modes:

- **NesC** (Nes Commands) — an interactive shell with 40+ built-in commands, pipes, redirects, variables, and `.nes` scripting
- **NesT** (Nes Typed) — a programming language with real types, expressions, functions, and `.nest` source files

## Why Nes?

| Problem                                     | Nes Solution                                             |
| ------------------------------------------- | -------------------------------------------------------- |
| `cmd.exe` syntax is arcane and painful      | Clean, minimal syntax with `$variables` (NesC)           |
| PowerShell is verbose and slow to start     | Instant startup, short commands                          |
| Unix commands don't exist on Windows        | 30+ built-in Unix-style commands                         |
| `.bat` files are ugly                       | `.nes` scripts are readable                              |
| No built-in calculator                      | `calc` with full expression support                      |
| Need a real language without external tools | NesT — types, functions, control flow in the same binary |

## Features at a Glance

### NesC — Shell & Commands (40+)

Navigation, file management, text processing, system info — all built-in without external dependencies. Variables, pipes, redirection, chaining, aliases, if/else/end, for/end loops.

### NesT — Programming Language

A typed language with `int`, `float`, `str`, `bool`, arithmetic expressions, if/else `{}`, for/while loops, user-defined functions, 24 built-in functions, and optional directives.

### Variable System

Shell variables (`let`), environment variables (`set`, `export`), and automatic `$expansion` in NesC. Typed `let x = 5;` declarations in NesT.

### Pipes & Redirection (NesC)

```nes
ls | grep .rs
echo hello > file.txt
cat log.txt >> archive.txt
```

### Command Chaining (NesC)

```nes
mkdir build && cd build && echo ready
```

### Math Evaluator (NesC)

```nes
calc (2+3)*4^2
```

### Functions (NesT)

```nest
fn factorial(n) {
    if n <= 1 { return 1; }
    return n * factorial(n - 1);
}
println(factorial(10));
```

### Script Execution

```
nes run deploy.nes       # NesC script
nes run program.nest     # NesT program
```

### System Fallback

Any command Nes doesn't know gets passed to `cmd.exe` — so `git`, `cargo`, `python`, and everything else just works.

## Design Philosophy

1. **Fast** — written in Rust with aggressive optimizations (LTO, single codegen unit, stripped symbols)
2. **Portable** — single `.exe`, no installer, no dependencies
3. **Familiar** — if you know Unix basics, you know NesC
4. **Dual-purpose** — shell commands when you need them, a real language when you need more
5. **Practical** — solves real problems, doesn't try to be everything

---

[← Table of Contents](../README.md) · [Next: Installation →](02-installation.md)
