// CW7: Zmienne - tekst
// =====================
// PRAKTYKA: Zmienne mogą być tekstem ("stringami")
fn praktyka_cw7() {
    let imie = "Michał";
    let nazwisko = "Kostkowski";
    let wiek = 25;
    let miasto = "Kraków";
    
    println!("Cześć, jestem {}!", imie);
    println!("Moje nazwisko to {}", nazwisko);
    println!("Mam {} lat", wiek);
    println!("Mieszkam w {}", miasto);
}

// WYJAŚNIENIE:
// Tekst pisze się w "cudzysłowach": "tekst"
// Liczby bez cudzysłowów: 25
//
// ZADANIE:
// Utwórz zmienne na imię, nazwisko, wiek, miasto.
// Wydrukuj je w sensownych zdaniach.

fn main() {
    let imie = "Michał";
    let nazwisko = "Kostkowski"; 
    let wiek = "15"; 
    let miasto = "Warszawa";

    println!("Cześć, mam na imie {}", imie);
    println!("Moje nazwisko to {}", nazwisko);
    println!("Mam {} lat", wiek);
    println!("I mieszkam w {}", miasto);
}
