// CW9: Zarobki
// ==============
// PRAKTYKA: Obliczenie zarobków
fn praktyka_cw9() {
    let zarobek_na_godzine = 30;
    let godziny = 160;
    let bonus = 500;
    
    let zarobek_brutto = zarobek_na_godzine * godziny;
    let razem = zarobek_brutto + bonus;
    let podatek = razem / 5;  // 20% podatku (uproszczenie)
    let na_reke = razem - podatek;
    
    println!("Zarobek z pracy: {} zł", zarobek_brutto);
    println!("Bonus: {} zł", bonus);
    println!("Razem brutto: {} zł", razem);
    println!("Podatek (20%): {} zł", podatek);
    println!("Na rękę: {} zł", na_reke);
}

// ZADANIE:
// Zmień liczby: zarobek_na_godzine, godziny, bonus.
// Możliwie: zmień też sposób liczenia podatku.

fn main() {
    let zarobek_na_godzine = 15;
    let godziny = 125;
    let bonus = 120;

    let zarobek_brutto = zarobek_na_godzine * godziny;
    let razem = zarobek_brutto + bonus;
    let podatek = razem / 5;
    let na_reke = razem - podatek;

    println!("Zarobek na godzine: {} zł", zarobek_na_godzine);
    println!("Zarobek Brutto wynosi {} zł", zarobek_brutto);
    println!("Łączny Zarobek z bonusem: {} zł", razem);
    println!("Podatek: {} zł", podatek );
}
