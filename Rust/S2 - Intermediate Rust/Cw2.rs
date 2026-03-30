// CW2: Łańcuchy if / else if / else
// ====================================
// PRAKTYKA: Możesz mieć wiele warunków po sobie.
// if sprawdza pierwszy warunek, else if sprawdza kolejny, else = reszta.
//
// if ocena >= 90 {
//     "Bardzo dobrze"
// } else if ocena >= 70 {
//     "Dobrze"
// } else if ocena >= 50 {
//     "Dostatecznie"
// } else {
//     "Niedostatecznie"
// }

fn praktyka_cw2() {
    let temperatura = 25;

    if temperatura >= 30 {
        println!("Gorąco! 🔥");
    } else if temperatura >= 20 {
        println!("Przyjemnie ciepło 😊");
    } else if temperatura >= 10 {
        println!("Chłodno 🧥");
    } else if temperatura >= 0 {
        println!("Zimno! ❄️");
    } else {
        println!("Mróz! Zostań w domu! 🥶");
    }

    let wiek = 17;

    if wiek >= 18 {
        println!("Możesz głosować i kupić piwo.");
    } else if wiek >= 16 {
        println!("Możesz jeździć motorowerem.");
    } else if wiek >= 13 {
        println!("Możesz mieć konto w social media.");
    } else {
        println!("Jesteś jeszcze za młody na wszystko 😄");
    }
}

// ZADANIE:
// 1) Utwórz zmienną `let ocena = ...;` (liczba 0-100)
// 2) Napisz łańcuch if/else if/else:
//    - >= 90 → "Celujący (6)"
//    - >= 75 → "Bardzo dobry (5)"
//    - >= 60 → "Dobry (4)"
//    - >= 45 → "Dostateczny (3)"
//    - >= 30 → "Dopuszczający (2)"
//    - poniżej → "Niedostateczny (1)"
// 3) Dodaj drugą zmienną `let frekwencja = ...;` (procent 0-100)
//    - >= 90 → "Wzorowa frekwencja"
//    - >= 75 → "Dobra frekwencja"
//    - poniżej → "Niska frekwencja — uwaga!"

fn main() {
    let ocena = 52;

    if ocena >= 90 {
        println!("Celujący: 6");
    } else if ocena >= 75 {
        println!("Bardzo Dobry: 5");
    } else if ocena >= 60 {
        println!("Dobry: 4");
    } else if ocena >= 45 {
        println!("Dostateczny: 3");
    } else if ocena >= 30 {
        println!("Dopuszczający: 2");
    } else {
        println!("Niedostateczny: 1");
    }

    let frekwencja = 77;

    if frekwencja >= 90 {
        println!("Wzorowa frekwencja, : {}%", frekwencja);
    } else if frekwencja >= 75{
        println!("Dobra frekwencja, :{}%", frekwencja);
    } else {
        println!("Niska frekwencja - Uwaga, :{}%", frekwencja);
    }
}