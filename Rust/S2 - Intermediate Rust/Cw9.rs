// CW9: Vec — dynamiczne wektory
// ================================
// PRAKTYKA: Tablica `[1,2,3]` ma stały rozmiar. Vec (wektor) może rosnąć!
//
// let mut lista = Vec::new();     ← pusty wektor
// lista.push(10);                 ← dodaj element
// lista.push(20);
// lista.len()                     ← ile elementów
// lista[0]                        ← dostęp po indeksie
//
// Można też: let lista = vec![1, 2, 3];  ← makro vec! tworzy gotowy wektor

fn praktyka_cw9() {
    // Tworzenie i dodawanie
    let mut zakupy: Vec<&str> = Vec::new();
    zakupy.push("Chleb");
    zakupy.push("Mleko");
    zakupy.push("Masło");
    zakupy.push("Jajka");

    println!("=== Lista zakupów ===");
    for (i, produkt) in zakupy.iter().enumerate() {
        println!("{}. {}", i + 1, produkt);
    }
    println!("Razem: {} produktów", zakupy.len());

    // Usuwanie
    zakupy.pop();  // usuwa ostatni element
    println!("\nPo usunięciu ostatniego:");
    println!("{:?}", zakupy);

    // vec! makro
    let mut wyniki = vec![85, 92, 78, 95, 88];
    println!("\n=== Wyniki ===");
    println!("Wyniki: {:?}", wyniki);

    // Dodawanie
    wyniki.push(100);
    println!("Po dodaniu 100: {:?}", wyniki);

    // Suma i średnia
    let mut suma = 0;
    for w in &wyniki {       // &wyniki = pożyczamy wektor (nie zabieramy go)
        suma = suma + w;
    }
    let srednia = suma as f64 / wyniki.len() as f64;
    println!("Suma: {}", suma);
    println!("Średnia: {:.1}", srednia);  // {:.1} = 1 miejsce po przecinku

    // contains — czy zawiera
    if wyniki.contains(&100) {
        println!("Jest setka! 🎉");
    }
}

// WYJAŚNIENIE:
// `Vec<i32>` = wektor liczb, `Vec<&str>` = wektor tekstów
// `.push(x)` = dodaj x na koniec
// `.pop()`   = usuń ostatni element
// `.len()`   = ile elementów
// `.contains(&x)` = czy zawiera x
// `&wyniki`  = pożyczamy wektor (jeszcze go nie "zjadamy")
// `vec![1,2,3]` = szybkie tworzenie wektora z elementami
// `.iter().enumerate()` = iteruj z indeksem (i, element)
// `{:.1}` = format z 1 miejscem po przecinku
//
// ZADANIE:
// 1) Utwórz Vec<&str> z listą 5 ulubionych gier/filmów. Wydrukuj je numerowaną listą.
// 2) Utwórz Vec<i32>, dodaj 10 liczb za pomocą push(). Oblicz sumę i średnią.
// 3) Napisz "filtrowanie": utwórz Vec liczb, potem stwórz NOWY Vec
//    tylko z liczbami > 50.
//    Podpowiedź:
//    let mut duze = Vec::new();
//    for x in &wszystkie { if *x > 50 { duze.push(*x); } }
//    (`*x` = odczytaj wartość z referencji)
// 4) (bonus) Połącz z funkcjami: napisz fn suma_vec(v: &Vec<i32>) -> i32
//    która przyjmuje wektor i zwraca sumę.

fn main() {
    // Twój kod tutaj!
}
