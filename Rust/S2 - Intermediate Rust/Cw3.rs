// CW3: Pętla for
// ================
// PRAKTYKA: `for` powtarza kod określoną liczbę razy.
//
// for i in 1..=5 {     ← od 1 do 5 WŁĄCZNIE (1, 2, 3, 4, 5)
//     println!("{}", i);
// }
//
// for i in 0..5 {      ← od 0 do 5 WYŁĄCZNIE (0, 1, 2, 3, 4)
//     println!("{}", i);
// }
//
// `1..=5` = range inclusive (z 5)
// `0..5`  = range exclusive (bez 5)

//fn praktyka_cw3() {;
    // Liczenie od 1 do 10;
    //println!("=== Liczenie ===");
    //for i in 1..=10 {;
     //   println!("Liczba: {}", i);
    //};

    // Tabliczka mnożenia dla 7;
    //println!("\n=== Tabliczka mnożenia: 7 ===");
    //for i in 1..=10 {;
      //  println!("7 * {} = {}", i, 7 * i);
    //};

    // Odliczanie;
   // println!("\n=== Odliczanie ===");
  //  for i in (1..=5).rev() {   // .rev() = odwróć kolejność;
  //      println!("{}...", i);
//    };
//    println!("START!");
//};

// WYJAŚNIENIE:
// `for zmienna in zakres { ... }` — powtarza kod dla każdej wartości w zakresie
// `1..=10`  = 1, 2, 3, ... 10 (z 10)
// `1..10`   = 1, 2, 3, ... 9  (bez 10)
// `.rev()`  = odwraca kolejność (10, 9, 8, ... 1)
//
// ZADANIE:
// 1) Wydrukuj liczby od 1 do 20
// 2) Wydrukuj tabliczkę mnożenia dla dowolnej liczby (np. 9)
// 3) Wydrukuj tylko liczby PARZYSTE od 2 do 20
//    Podpowiedź: użyj `if i % 2 == 0` wewnątrz pętli
// 4) Zrób odliczanie od 10 do 1 i na końcu wydrukuj "BOOM!"
// 5) (bonus) Wydrukuj trójkąt z gwiazdek:
//    *
//    **
//    ***
//    ****
//    *****
//    Podpowiedź: użyj pętli for i wewnątrz niej print!("*"), a potem println!()

fn main() {
    println!("<= Od 1 do 20 =>"); 
    
    for i in 1..=20 {
        println!("{}", i);
    }

    println!("<= Tabliczka Mnożenia =>");
    
    let tabliczka0 = " 1   2   3   4   5   6   7   8   9   10";
    let tabliczka1 = "/ -   -   -   -   -   -   -   -   -   -";
    let tabliczka2 = "1 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10";
    let tabliczka3 = "2 | 2 | 4 | 6 | 8 | 10| 12| 14| 16| 18| 20";
    let tabliczka4 = "3 | 3 | 6 | 9 | 12| 15| 18| 21| 24| 27| 30";
    let tabliczka5 = "4 | 4 | 8 | 12| 16| 20| 24| 28| 32| 36| 40";
    let tabliczka6 = "5 | 5 | 10| 15| 20| 25| 30| 35| 40| 45| 50";
    let tabliczka7 = "6 | 6 | 12| 18| 24| 30| 36| 42| 48| 54| 60";
    let tabliczka8 = "7 | 7 | 14| 21| 28| 35| 42| 49| 56| 63| 70";
    let tabliczka9 = "8 | 8 | 16| 24| 32| 40| 48| 56| 64| 72| 80";
    let tabliczka10 = "9 | 9 | 18| 27| 36| 45| 54| 63| 72| 81| 90";
    let tabliczka11 = "10| 10| 20| 30| 40| 50| 60| 70| 80| 90| 100";
    
    
    println!("{}", name0);
    println!("{}", name1);
    println!("{}", name2);
    println!("{}", name3);
    println!("{}", name4);
    println!("{}", name5);
    println!("{}", name6);
    println!("{}", name7);
    println!("{}", name8);
    println!("{}", name9);
    println!("{}", name10);
    println!("{}", name11);
    
}