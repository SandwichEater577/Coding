// CW10: Podsumowanie - "Mój program"
// ====================================
// PRAKTYKA: Łaczenie wszystkiego co umiesz
fn praktyka_cw10() {
    println!("=== Mój profil ===");
    
    let imie = "Michał";
    let wiek = 25;
    let zarobek_na_h = 50;
    let godzin_dziennie = 8;
    
    let zarobek_dzienny = zarobek_na_h * godzin_dziennie;
    let ulubiona_liczba = 7;
    
    println!("Imię: {}", imie);
    println!("Wiek: {}", wiek);
    println!("Zarobek na godzinę: {} zł", zarobek_na_h);
    println!("Godzin dziennie: {}", godzin_dziennie);
    println!("Zarobek dzienny: {} zł", zarobek_dzienny);
    println!("Ulubiona liczba: {}", ulubiona_liczba);
    println!("Wszechświat: 42");
}

// ZADANIE:
// Napisz własny "profil" - zmienne: imię, wiek, zarobek, ulubiona liczba.
// Wydrukuj to sensownie. Możesz dodać ASCII art!
// Możesz też użyć if do warunków (np. "Jeśli wiek > 18: jestem dorosły").

fn main() {
    println!("<==Mój Profil==>");

    let imie = "Michał";
    let nazwisko = "Kostkowski";
    let wiek = 15;
    let status = "Szczęśliwy";
    let praca = "Full Stack BackShit Developer";
    let godzin_dziennie = 6;
    let godzin_miesiecznie = godzin_dziennie * 20; 
    let zarobek_na_godzine = 20;
    let pelna_pensja_miesieczna = godzin_miesiecznie * zarobek_na_godzine;

    println!("Imię: {}", imie);
    println!("Nazwisko: {}", nazwisko);
    println!("Wiek: {} lat", wiek);
    println!("Obecna Praca: {}", praca);
    println!("Pensja: {} zł", pelna_pensja_miesieczna);
    println!("Zarobek na godzine: {} zł", zarobek_na_godzine);
    println!("Emocje: {}", status);
}
