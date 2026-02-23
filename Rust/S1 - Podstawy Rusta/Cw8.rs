// CW8: Gra - zgadnij liczbę
// ===========================
// PRAKTYKA: Logika "za mało/za dużo"
fn praktyka_cw8() {
    let liczba_komputera = 42;
    
    println!("Myślę na liczbę...");
    println!("Ty zgadujesz: 30");
    
    if 30 < liczba_komputera {
        println!("Za mało!");
    }
    
    println!("Ty zgadujesz: 50");
    
    if 50 > liczba_komputera {
        println!("Za dużo!");
    }
    
    println!("Ty zgadujesz: 42");
    
    if 42 == liczba_komputera {
        println!("Trafiłeś!");
    }
}

// WYJAŚNIENIE:
// if = jeśli warunek jest prawdziwy, wykonaj kod
// < = mniejsze niż, > = większe niż, == = równe
//
// ZADANIE:
// Utwórz swoją grę: liczba komputera, 3-4 próby użytkownika, logika porównań.

fn main() {
    // STRUKTURA `main()` (podpowiedź, jak ma wyglądać minimalnie):
    // 1) Deklaracja ukrytej liczby (tu bez rand, typ np. `u8` lub `i32`):
    //      let wygenerowana_liczba: u8 = 41;
    // 2) Pętla prób (np. `for proba in 1..=3 { ... }`) — każda iteracja to jedna próba.
    // 3) Wewnątrz pętli:
    //      - wypisz prompt: println!("Próba {}: Podaj liczbę:", proba);
    //      - stwórz bufor: let mut wejscie = String::new();
    //      - czytaj stdin: std::io::stdin().read_line(&mut wejscie).expect(...);
    //      - parsuj bezpiecznie: match wejscie.trim().parse::<u8>() { Ok(n) => n, Err(_) => { println!("Niepoprawne"); continue; } }
    //      - (opcjonalnie) waliduj zakres: if ! (0..=128).contains(&n) { println!("Podaj 0..=128"); continue; }
    //      - porównaj z `wygenerowana_liczba` i wypisz "Za mało" / "Za dużo" / "Trafiłeś".
    //      - przy trafieniu `break` lub `return`.
    // 4) Po pętli: wypisz komunikat o przegranej i pokaż sekret.
    // UWAGI:
    //  - używaj `trim()` przed `parse()` (usuwa \n),
    //  - `u8` wymusza zakres 0..=255; dla maks 128 użyj `u8` i dodatkowej walidacji,
    //  - jeśli chcesz akceptować ujemne liczby, użyj `i32`.

    // Przykładowa deklaracja sekretu (zmień, jeśli chcesz):
    let wygenerowana_liczba: u8 = 41;

    println!("Zgadnij Liczbę!");
    println!("Podpowiedź: ta liczba ma 2 cyfry");

    for proba in 1..3 {
        println!("Próba {}: Podaj liczbę:", proba);
        let mut wejscie = String::new();
    }
}
