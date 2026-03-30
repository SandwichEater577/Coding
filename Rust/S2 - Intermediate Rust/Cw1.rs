// CW1: Zmienne mutowalne (mut) i if/else
// ========================================
// PRAKTYKA: W S1 używałeś `let x = 5;` — to zmienne NIEZMIENNE (immutable).
// Żeby zmienić wartość zmiennej, musisz dodać `mut` (mutable = zmienny).
//
// let mut x = 5;   ← x można potem zmienić
// x = 10;          ← OK!
//
// let y = 5;
// y = 10;          ← BŁĄD! y jest immutable
//
// if/else działa tak:
// if warunek { ... } else { ... }

fn praktyka_cw1() {
    let mut punkty = 0;
    println!("Start: {} punktów", punkty);

    punkty = punkty + 10;
    println!("Po zadaniu 1: {} punktów", punkty);

    punkty = punkty + 5;
    println!("Po zadaniu 2: {} punktów", punkty);

    if punkty >= 10 {
        println!("Świetnie! Masz co najmniej 10 punktów!");
    } else {
        println!("Jeszcze trochę brakuje do 10...");
    }

    println!("Końcowy wynik: {}", punkty);
}

// WYJAŚNIENIE:
// `mut`   = pozwala zmienić wartość zmiennej później
// `if`    = jeśli warunek jest prawdziwy
// `else`  = w przeciwnym wypadku
// `>=`    = większe lub równe
// `<=`    = mniejsze lub równe
// `!=`    = nie równe
//
// ZADANIE:
// 1) Utwórz zmienną `mut zycie = 100`
// 2) Odejmij od niej obrażenia (np. zycie = zycie - 25)
// 3) Użyj if/else: jeśli zycie > 50 → "Jesteś zdrowy!", w przeciwnym razie → "Uważaj!"
// 4) Odejmij jeszcze raz i sprawdź czy zycie <= 0 → "Game Over!"
// 5) Wydrukuj życie po każdym kroku

fn main() {
    let damage = 67;
    let mut healthpoints = 100;
    
    println!("HP: {}", healthpoints);
    println!("Zadane obrażenia: {}", damage);

    healthpoints = healthpoints - damage;

    println!("HP: {}", healthpoints);

    if healthpoints >= 70 {
        println!("Jesteś Zdrowy, HP: {}", healthpoints);
    } else if healthpoints >= 45 {
        println!("Jesteś Zainfekowany, HP: {}", healthpoints);
    } else if healthpoints == 0 {
        println!("Game Over, HP: {}", healthpoints);
    } else {
        println!("Natychmiastowo uzyskaj pomocy, HP: {}", healthpoints);
    }
}