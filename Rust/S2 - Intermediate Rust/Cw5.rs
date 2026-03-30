// CW5: Funkcje — podstawy
// =========================
// PRAKTYKA: Funkcja to blok kodu z nazwą, który możesz wywoływać wielokrotnie.
//
// fn nazwa(parametr: typ) {
//     // kod
// }
//
// Wywołanie: nazwa(argument);

fn powitaj(imie: &str) {
    println!("Cześć, {}!", imie);
}

fn wydrukuj_linie(dlugosc: i32) {
    for _ in 0..dlugosc {
        print!("=");
    }
    println!();  // nowa linia na końcu
}

fn pokaz_kwadrat(bok: i32) {
    let pole = bok * bok;
    let obwod = 4 * bok;
    println!("Kwadrat o boku {}:", bok);
    println!("  Pole: {}", pole);
    println!("  Obwód: {}", obwod);
}

fn praktyka_cw5() {
    powitaj("Michał");
    powitaj("Ola");
    powitaj("Rust");

    wydrukuj_linie(20);

    pokaz_kwadrat(5);
    pokaz_kwadrat(10);
}

// WYJAŚNIENIE:
// `fn nazwa()` = deklaracja funkcji bez parametrów
// `fn nazwa(x: i32)` = funkcja z jednym parametrem typu i32 (integer 32-bit)
// `fn nazwa(tekst: &str)` = funkcja z parametrem tekstowym
// `&str` = "pożyczony" tekst (string slice) — na razie zapamiętaj że tak się pisze tekst
// `_` w `for _ in 0..n` = nie potrzebuję zmiennej iteracji, ignoruję ją
//
// ZADANIE:
// 1) Napisz funkcję `przedstaw_sie(imie: &str, wiek: i32)` która drukuje:
//    "Cześć! Jestem [imie] i mam [wiek] lat."
// 2) Napisz funkcję `pokaz_prostokat(a: i32, b: i32)` która drukuje pole i obwód.
// 3) Napisz funkcję `gwiazdki(n: i32)` która drukuje n gwiazdek w jednej linii.
// 4) Wywołaj każdą funkcję w main() z różnymi argumentami.

fn main() {
    // Twój kod tutaj!
    // np. przedstaw_sie("Michał", 15);
    //     pokaz_prostokat(5, 3);
    //     gwiazdki(10);
}
