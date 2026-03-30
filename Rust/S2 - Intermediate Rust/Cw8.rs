// CW8: Krotki (tuples) i tablice (arrays)
// =========================================
// PRAKTYKA: Krotka to grupa różnych wartości. Tablica to lista JEDNAKOWYCH wartości.
//
// Krotka (tuple):
// let osoba = ("Michał", 15, true);   ← różne typy!
// osoba.0 = "Michał", osoba.1 = 15, osoba.2 = true
//
// Tablica (array):
// let liczby = [10, 20, 30, 40, 50];  ← ten sam typ!
// liczby[0] = 10, liczby[1] = 20, ... (indeks od 0!)

fn praktyka_cw8() {
    // === KROTKI ===
    let gracz = ("Warrior", 100, 25);  // (klasa, HP, atak)
    println!("=== Gracz ===");
    println!("Klasa: {}", gracz.0);
    println!("HP: {}", gracz.1);
    println!("Atak: {}", gracz.2);

    // Destrukturyzacja — rozpakowywanie krotki
    let (klasa, hp, atak) = gracz;
    println!("{} ma {} HP i {} ataku", klasa, hp, atak);

    // === TABLICE ===
    let oceny = [5, 4, 3, 5, 6, 4];
    println!("\n=== Oceny ===");
    println!("Pierwsza ocena: {}", oceny[0]);
    println!("Ostatnia ocena: {}", oceny[5]);
    println!("Liczba ocen: {}", oceny.len());

    // Iterowanie po tablicy
    println!("\nWszystkie oceny:");
    for ocena in oceny {
        print!("{} ", ocena);
    }
    println!();

    // Suma elementów
    let mut suma = 0;
    for ocena in oceny {
        suma = suma + ocena;
    }
    println!("Suma ocen: {}", suma);

    // Tablica z tym samym elementem
    let zera = [0; 5];  // [0, 0, 0, 0, 0]
    println!("\nTablica zer: {:?}", zera);  // {:?} = debug print (drukuje całą tablicę)
}

// WYJAŚNIENIE:
// KROTKA: let t = (a, b, c); dostęp: t.0, t.1, t.2
// TABLICA: let a = [1, 2, 3]; dostęp: a[0], a[1], a[2]
// KROTKA mieszane typy, TABLICA ten sam typ
// `{:?}` = "debug format" — drukuje tablice, krotki itp. w czytelny sposób
// `.len()` = długość tablicy
// `[0; 5]` = tablica 5 elementów, wszystkie = 0
//
// ZADANIE:
// 1) Utwórz krotkę `przedmiot` z nazwą, oceną i czy zdany (bool).
//    Wydrukuj każde pole osobno.
// 2) Utwórz tablicę 7 temperatur (dni tygodnia).
//    Użyj pętli for, żeby znaleźć najwyższą temperaturę.
//    Podpowiedź: `let mut max = temperatury[0]; for t in temperatury { if t > max { max = t; } }`
// 3) Utwórz tablicę 5 liczb i oblicz ich średnią.
//    Podpowiedź: suma / liczba elementów (uważaj na typy — użyj `as f64` do konwersji)
// 4) (bonus) Utwórz 3 krotki — 3 graczy (imię, HP, atak).
//    Wydrukuj je ładnie w tabelce.

fn main() {
    // Twój kod tutaj!
}
