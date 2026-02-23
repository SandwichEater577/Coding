// CW4: Pętla while i loop
// =========================
// PRAKTYKA: `while` powtarza kod DOPÓKI warunek jest prawdziwy.
// `loop` powtarza kod W NIESKOŃCZONOŚĆ (aż zrobisz `break`).
//
// while warunek {
//     // kod
// }
//
// loop {
//     // kod
//     if warunek { break; }
// }

fn praktyka_cw4() {
    // while — odliczanie paliwa
    println!("=== Paliwo ===");
    let mut paliwo = 5;
    while paliwo > 0 {
        println!("Paliwo: {} litrów", paliwo);
        paliwo = paliwo - 1;
    }
    println!("Brak paliwa! Stoisz.");

    // loop z break — szukanie liczby
    println!("\n=== Szukam liczby podzielnej przez 7 i 3 ===");
    let mut n = 1;
    loop {
        if n % 7 == 0 && n % 3 == 0 {
            println!("Znalazłem: {}", n);
            break;  // przerwij pętlę
        }
        n = n + 1;
    }

    // mut + while — podwajanie
    println!("\n=== Podwajanie ===");
    let mut wartosc = 1;
    while wartosc < 1000 {
        println!("{}", wartosc);
        wartosc = wartosc * 2;
    }
}

// WYJAŚNIENIE:
// `while warunek { ... }` — powtarza dopóki warunek == true
// `loop { ... break; }`  — nieskończona pętla, `break` ją przerywa
// `&&` = AND (i), `||` = OR (lub)
// `continue` = przeskocz resztę iteracji i idź do następnej
//
// ZADANIE:
// 1) Symuluj "życie w grze": zacznij od `mut hp = 100`,
//    w każdej turze odejmij losowe obrażenia (np. 15, 22, 10...),
//    użyj while hp > 0, wydrukuj HP po każdej turze.
// 2) Użyj `loop` + `break` żeby znaleźć pierwszą liczbę > 100
//    która jest podzielna przez 13.
// 3) (bonus) Zrób "rakietę": odliczaj od 10, ale na 5 wydrukuj
//    "SILNIKI ZAPALONE!" i kontynuuj. Na 0 wydrukuj "LIFTOFF!"

fn main() {
    // Twój kod tutaj!
}
