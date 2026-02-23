# Chapter 18 · NesT Examples

[← NesT Functions & Built-ins](17-nest-builtins.md) · [Back to Table of Contents](../README.md)

---

Complete, ready-to-run `.nest` programs. Save as a `.nest` file and run with `nes run <filename>`.

---

## 1 · Hello World

```nest
println("Hello, World!");
```

---

## 2 · Variables & Types

```nest
let name = "NesT";
let version = 5;
let pi = 3.14159;
let ready = true;

println("Name:    " + name);
println("Version: " + str(version));
println("Pi:      " + str(pi));
println("Ready:   " + str(ready));
println("Types:   " + type(name) + ", " + type(version) + ", " + type(pi) + ", " + type(ready));
```

---

## 3 · Calculator

```nest
fn calculate(a, op, b) {
    if op == "+" { return a + b; }
    if op == "-" { return a - b; }
    if op == "*" { return a * b; }
    if op == "/" { return a / b; }
    if op == "%" { return a % b; }
    return 0;
}

println("=== NesT Calculator ===");
println("10 + 3  = " + str(calculate(10, "+", 3)));
println("10 - 3  = " + str(calculate(10, "-", 3)));
println("10 * 3  = " + str(calculate(10, "*", 3)));
println("10 / 3  = " + str(calculate(10, "/", 3)));
println("10 % 3  = " + str(calculate(10, "%", 3)));
```

---

## 4 · Fibonacci

```nest
fn fib(n) {
    if n <= 1 { return n; }
    return fib(n - 1) + fib(n - 2);
}

println("=== Fibonacci Sequence ===");
for i in 0..20 {
    println("fib(" + str(i) + ") = " + str(fib(i)));
}
```

---

## 5 · Factorial

```nest
fn factorial(n) {
    if n <= 1 { return 1; }
    return n * factorial(n - 1);
}

println("=== Factorials ===");
for i in 0..13 {
    println(str(i) + "! = " + str(factorial(i)));
}
```

---

## 6 · FizzBuzz

```nest
println("=== FizzBuzz ===");
for i in 1..101 {
    if i % 15 == 0 {
        println("FizzBuzz");
    } else {
        if i % 3 == 0 {
            println("Fizz");
        } else {
            if i % 5 == 0 {
                println("Buzz");
            } else {
                println(i);
            }
        }
    }
}
```

---

## 7 · Prime Checker

```nest
fn is_prime(n) {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 { return false; }
    let i = 3;
    while i * i <= n {
        if n % i == 0 { return false; }
        i = i + 2;
    }
    return true;
}

println("=== Primes up to 100 ===");
let count = 0;
for n in 2..101 {
    if is_prime(n) {
        print(str(n) + " ");
        count = count + 1;
    }
}
println("");
println("Found " + str(count) + " primes.");
```

---

## 8 · Number Guessing Game

```nest
# Simple guessing game
let secret = 42;
let attempts = 0;
let found = false;

println("=== Guess the Number (1-100) ===");

while !found {
    print("Your guess: ");
    let guess = int(input());
    attempts = attempts + 1;

    if guess == secret {
        println("Correct! You got it in " + str(attempts) + " attempts!");
        found = true;
    } else {
        if guess < secret {
            println("Too low!");
        } else {
            println("Too high!");
        }
    }
}
```

---

## 9 · Power Table

```nest
println("=== Power Table ===");
println("n    n^2    n^3    n^4");
println("---  -----  -----  ------");

for n in 1..11 {
    let n2 = pow(n, 2);
    let n3 = pow(n, 3);
    let n4 = pow(n, 4);
    println(str(n) + "    " + str(n2) + "    " + str(n3) + "    " + str(n4));
}
```

---

## 10 · Collatz Conjecture

```nest
fn collatz_steps(n) {
    let steps = 0;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
        steps = steps + 1;
    }
    return steps;
}

println("=== Collatz Conjecture ===");
for i in 1..26 {
    println("collatz(" + str(i) + ") = " + str(collatz_steps(i)) + " steps");
}
```

---

## 11 · Temperature Converter

```nest
fn c_to_f(c) {
    return c * 9 / 5 + 32;
}

fn f_to_c(f) {
    return (f - 32) * 5 / 9;
}

println("=== Temperature Converter ===");
println("Celsius → Fahrenheit:");
for c in 0..11 {
    let temp = c * 10;
    println(str(temp) + "°C = " + str(c_to_f(temp)) + "°F");
}
```

---

## 12 · GCD & LCM

```nest
fn gcd(a, b) {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

fn lcm(a, b) {
    return a * b / gcd(a, b);
}

println("=== GCD & LCM ===");
println("gcd(48, 18) = " + str(gcd(48, 18)));
println("gcd(100, 75) = " + str(gcd(100, 75)));
println("lcm(4, 6) = " + str(lcm(4, 6)));
println("lcm(12, 8) = " + str(lcm(12, 8)));
```

---

[← NesT Functions & Built-ins](17-nest-builtins.md) · [Back to Table of Contents](../README.md)
