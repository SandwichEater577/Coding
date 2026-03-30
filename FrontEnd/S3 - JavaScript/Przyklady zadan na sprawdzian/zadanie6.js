/*
  Zadanie 6 (10p.)
  Napisz wyrażenie funkcyjne o nazwie "findWinner". 
  Funkcja "findWinner" w argumencie otrzymuje macierz (3x3) reprezentującą stan gry w "Kółko i Krzyżyk". Pojedynczy element tej tablicy to "O", "X" lub wartość undefined, gdzie undefined oznacza, że żaden z graczy nie zaznaczył tego pola.

  Zadaniem funkcji "findWinner" jest zwrócić informację o tym czy znaleziono wygraną na podstawie otrzymanej macierzy.
  Jeśli wygrał gracz "O" należy zwrócić "O".
  Jeśli wygrał gracz "X" należy zwrócić "X".
  Jeśli jest remis należy zwrócić null.
  Jeśli gra jest w trakcie rozgrywki należy zwrócić undefined.

  📥 Przykładowe wejście dla funkcji: 
      [
        ["O",    "O",    "X"],
        ["O",    "X",    "O"],
        ["X", undefined, "O"]
      ]
  📥 Przykładowe wyjście dla funkcji: "X"
*/
