// CW6: Funkcje — zwracanie wartości
// ====================================
// PRAKTYKA: Funkcja może zwracać wynik za pomocą `-> typ`.
// Ostatnie wyrażenie BEZ średnika to wartość zwracana.
// Możesz też użyć `return wartość;` (ale idiomatycznie w Ruscie pomija się return).
//
// fn dodaj(a: i32, b: i32) -> i32 {
//     a + b        ← BEZ średnika = zwraca tę wartość
// }

fn dodaj(a: i32, b: i32) -> i32 {
    a + b
}

fn pole_prostokata(a: i32, b: i32) -> i32 {
    a * b
}

fn jest_pelnoletni(wiek: i32) -> bool {
    wiek >= 18
}

fn pozdrowienie(imie: &str) -> String {
    format!("Witaj, {}! Miłego dnia!", imie)
}

fn praktyka_cw6() {
    let wynik = dodaj(10, 20);
    println!("10 + 20 = {}", wynik);

    let pole = pole_prostokata(5, 3);
    println!("Pole prostokąta 5x3 = {}", pole);

    let pelnoletni = jest_pelnoletni(15);
    println!("Czy 15-latek jest pełnoletni? {}", pelnoletni);

    let pelnoletni2 = jest_pelnoletni(21);
    println!("Czy 21-latek jest pełnoletni? {}", pelnoletni2);

    let msg = pozdrowienie("Michał");
    println!("{}", msg);

    // Można też użyć wyniku bezpośrednio:
    println!("3 + 7 = {}", dodaj(3, 7));
}

// WYJAŚNIENIE:
// `-> i32`    = funkcja zwraca liczbę całkowitą
// `-> bool`   = zwraca true/false
// `-> String` = zwraca nowy tekst (owned string)
// `format!()` = jak println!, ale zamiast drukować, tworzy String
// BEZ średnika na końcu = to jest wartość zwracana
// Ze średnikiem = to jest instrukcja (nic nie zwraca)
//
// ZADANIE:
// 1) Napisz funkcję `kwadrat(n: i32) -> i32` — zwraca n * n
// 2) Napisz funkcję `obwod_kola(promien: f64) -> f64` — zwraca 2.0 * 3.14159 * promien
//    (f64 = liczba zmiennoprzecinkowa, np. 3.5, 2.0)
// 3) Napisz funkcję `max(a: i32, b: i32) -> i32` — zwraca większą z dwóch liczb
//    Podpowiedź: użyj if/else
// 4) Napisz funkcję `ocena_slowna(punkty: i32) -> &str` — zwraca "celujący"/"dobry" etc.
//    Podpowiedź: `-> &str` i zwracaj tekst w cudzysłowach
// 5) Wywołaj wszystkie w main() i wydrukuj wyniki

fn main() {
    // Twój kod tutaj!
}
