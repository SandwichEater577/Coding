// CW10: Struktury (struct) — Podsumowanie S2
// =============================================
// PRAKTYKA: `struct` pozwala tworzyć własne typy danych z nazwanymi polami.
// To jak tworzenie własnego "szablonu" dla danych.
//
// struct Gracz {
//     imie: String,
//     hp: i32,
//     level: i32,
// }
//
// let g = Gracz { imie: String::from("Hero"), hp: 100, level: 1 };

struct Gracz {
    imie: String,
    klasa: String,
    hp: i32,
    max_hp: i32,
    atak: i32,
    level: i32,
}

fn pokaz_gracza(g: &Gracz) {
    println!("╔══════════════════════════╗");
    println!("║ {} (Lv. {})", g.imie, g.level);
    println!("║ Klasa: {}", g.klasa);
    println!("║ HP: {}/{}", g.hp, g.max_hp);
    println!("║ Atak: {}", g.atak);
    println!("╚══════════════════════════╝");
}

fn czy_zyje(g: &Gracz) -> bool {
    g.hp > 0
}

fn zadaj_obrazenia(g: &mut Gracz, dmg: i32) {
    g.hp = g.hp - dmg;
    if g.hp < 0 {
        g.hp = 0;
    }
    println!("{} otrzymuje {} obrażeń! HP: {}/{}", g.imie, dmg, g.hp, g.max_hp);
}

fn ulecz(g: &mut Gracz, heal: i32) {
    g.hp = g.hp + heal;
    if g.hp > g.max_hp {
        g.hp = g.max_hp;
    }
    println!("{} uleczony o {}! HP: {}/{}", g.imie, heal, g.hp, g.max_hp);
}

fn level_up(g: &mut Gracz) {
    g.level = g.level + 1;
    g.max_hp = g.max_hp + 20;
    g.hp = g.max_hp;  // pełne leczenie przy level up
    g.atak = g.atak + 5;
    println!("🎉 {} awansuje na level {}!", g.imie, g.level);
}

fn praktyka_cw10() {
    let mut bohater = Gracz {
        imie: String::from("Michał"),
        klasa: String::from("Warrior"),
        hp: 100,
        max_hp: 100,
        atak: 25,
        level: 1,
    };

    pokaz_gracza(&bohater);

    // Symulacja walki
    zadaj_obrazenia(&mut bohater, 30);
    zadaj_obrazenia(&mut bohater, 45);
    ulecz(&mut bohater, 20);

    println!("\nCzy żyje? {}", czy_zyje(&bohater));

    level_up(&mut bohater);
    pokaz_gracza(&bohater);
}

// WYJAŚNIENIE:
// `struct Nazwa { pole: typ, ... }` = definicja struktury
// `Nazwa { pole: wartość, ... }`    = tworzenie instancji
// `g.pole`                          = dostęp do pola
// `&Gracz`                          = pożyczka (read-only reference)
// `&mut Gracz`                      = pożyczka z prawem do zmiany
// `String::from("tekst")`           = tworzy String z &str
//
// ZADANIE (łączy WSZYSTKO z S2):
// 1) Stwórz struct `Przedmiot` z polami: nazwa (String), wartosc (i32), rzadkosc (&str)
// 2) Napisz funkcję `pokaz_przedmiot(p: &Przedmiot)` — ładnie drukuje przedmiot
// 3) Stwórz struct `Bohater` z polami: imie, hp, atak, level, zloto (i32)
// 4) Napisz funkcje:
//    - `kup_przedmiot(b: &mut Bohater, p: &Przedmiot)` — odejmuje złoto, drukuje info
//    - `walcz(b: &mut Bohater, wrog_hp: i32, wrog_atak: i32)` — prosta walka w pętli
//    - `pokaz_bohatera(b: &Bohater)` — ładnie drukuje stan bohatera
// 5) W main() stwórz bohatera, kilka przedmiotów, kup je, walcz z wrogiem.
//    Użyj if/else, match, for/while, mut, Vec — pokaż wszystko co umiesz!
//
// To Twoje wielkie zadanie podsumowujące S2! 💪

fn main() {
    // Twój kod tutaj!
    // np.
    // let mut hero = Bohater { ... };
    // let miecz = Przedmiot { ... };
    // kup_przedmiot(&mut hero, &miecz);
    // walcz(&mut hero, 50, 10);
    // pokaz_bohatera(&hero);
}
