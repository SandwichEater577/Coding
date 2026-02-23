/*
  Zadanie 5 (10p.)
  Napisz deklarację funkcji o nazwie "makeTranslate". 
  Funkcja "makeTranslate" w argumencie otrzymuje klucz reprezentujący język ("pl"/"en"), domyślna wartość argumentu to "pl".
  Zadaniem funkcji "makeTranslate" jest zwrócić funkcję strzałkową.

  Funkcja strzałkowa w argumencie przyjmuje klucz reprezentujący sekcję na stronie ("header"/"main"/"footer"). 
  Zadaniem funkcji strzałkowej jest zwrócić tekst w odpowiednim języku dla danej sekcji na stronie.
  W przypadku, gdy podany w argumencie klucz reprezentujący sekcję nie zostanie odnaleziony w pod-obiekcie obiektu "content", należy wyświetlić wartość, która znajduje się pod kluczem "default" dla wybranego języka. Użyj do tego operatora warunkowego.

  📥 Przykładowe wejście dla funkcji "makeTranslate": "en"
  📥 Przykładowe wyjście dla funkcji "makeTranslate": funkcja strzałkową, która po wywołaniu z argumentem "header" zwróci tekst "Good morning!"

  📥 Przykładowe wejście dla funkcji "makeTranslate": "pl"
  📥 Przykładowe wyjście dla funkcji "makeTranslate": funkcja strzałkową, która po wywołaniu z argumentem "xyz" zwróci tekst "Nie znaleziono"
*/
