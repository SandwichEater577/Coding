// CW1: Wypisywanie i formatowanie (miło i na spokojnie)
// ======================================================
// PRAKTYKA: podstawy `println!`, placeholder `{}` i proste wartości
// - `println!` = wypisz i dodaj nową linię (newline)
// - `{}` = miejsce na wartość (np. liczba, tekst)
// - po przecinku podajesz wartości w kolejności, które trafią w `{}`

fn praktyka_cw1() {
    // Najprostsze wypisywanie
    println!("Hello, World!");
    println!("Witaj w Ruscie!");

    // Formatowanie z wieloma placeholderami
    println!("{} + {} = {}", 2, 2, 2 + 2);

    // Możesz wstawiać dowolne wyrażenia
    println!("1 + 1 + 2 = {}", 1 + 1 + 2);

    // Dla porównania: `print!` nie dodaje nowej linii
    // (zostawiamy jako komentarz, żeby nie mieszać w wyjściu)
    // print!("bez newline");
    // println!(" ← teraz przejście do nowej linii");
}

// ZADANIE (Twoja kolej):
// 1) Zmień tekst „Hello, World!” na coś swojego (np. imię, nastrój).
// 2) Zmień „Witaj w Ruscie!” na zdanie o Tobie (np. „Uczę się Rusta od zera”).
// 3) Dodaj jedną linię z formatowaniem, np.: println!("Moja liczba: {}", 42).
// 4) (opcjonalnie) Spróbuj `print!` i potem `println!`, by zobaczyć różnicę.
//    Uwaga: `println` bez `!` nie istnieje; musi być `println!` (to makro).
//    `{}` wstawia wartości zgodnie z kolejnością argumentów po przecinku.

fn main() {
    println!("Hello, World!");
    println!("Witaj w Ruscie!");
    print!("Dwa plus siedem to {}", 2 + 7);
    println!(" ← teraz przejście do nowej linii");
}