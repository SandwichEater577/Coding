// CW7: match — eleganckie porównywanie
// ======================================
// PRAKTYKA: `match` to potężna wersja if/else if/else.
// Rust WYMAGA, żeby match obsłużył WSZYSTKIE możliwości!
//
// match wartość {
//     wzorzec1 => { kod },
//     wzorzec2 => { kod },
//     _        => { domyślny kod },   // _ = "cokolwiek innego"
// }

fn praktyka_cw7() {
    // match z liczbą
    let dzien = 3;
    let nazwa_dnia = match dzien {
        1 => "Poniedziałek",
        2 => "Wtorek",
        3 => "Środa",
        4 => "Czwartek",
        5 => "Piątek",
        6 => "Sobota",
        7 => "Niedziela",
        _ => "Nieznany dzień",
    };
    println!("Dzień {}: {}", dzien, nazwa_dnia);

    // match z zakresami
    let ocena = 85;
    let slownie = match ocena {
        90..=100 => "Celujący",
        75..=89  => "Bardzo dobry",
        60..=74  => "Dobry",
        45..=59  => "Dostateczny",
        30..=44  => "Dopuszczający",
        _        => "Niedostateczny",
    };
    println!("Ocena {}: {}", ocena, slownie);

    // match z tekstem
    let komenda = "start";
    match komenda {
        "start"  => println!("Rozpoczynam grę..."),
        "pause"  => println!("Pauza."),
        "quit"   => println!("Do widzenia!"),
        _        => println!("Nieznana komenda: {}", komenda),
    }

    // match który zwraca wartość
    let liczba = 7;
    let typ = match liczba % 2 {
        0 => "parzysta",
        _ => "nieparzysta",
    };
    println!("{} jest {}", liczba, typ);
}

// WYJAŚNIENIE:
// `match x { ... }` = porównaj x z wzorcami
// `1 => ...`    = jeśli x == 1
// `1..=5 => ..` = jeśli x jest od 1 do 5 (włącznie)
// `_ => ...`    = wszystko inne (wildcard / domyślny)
// match MUSI pokryć wszystkie możliwości! Dlatego `_` na końcu.
// match może ZWRACAĆ wartość (jak w przykładzie z `nazwa_dnia`)
//
// ZADANIE:
// 1) Napisz match dla miesięcy: 1-12 → nazwa miesiąca, _ → "Nieznany"
// 2) Napisz match dla ocen szkolnych (1-6) z komentarzami
//    np. 6 → "Wspaniale!", 1 → "Popracuj nad tym..."
// 3) Napisz "kalkulator": match na znaku operacji
//    let op = "+";
//    match op { "+" => ..., "-" => ..., "*" => ..., "/" => ..., _ => "Nieznana operacja" }
// 4) (bonus) Połącz match z funkcją: napisz fn ktory_sezon(miesiac: i32) -> &str
//    który zwraca "Wiosna"/"Lato"/"Jesień"/"Zima" na podstawie miesiąca

fn main() {
    // Twój kod tutaj!
}
