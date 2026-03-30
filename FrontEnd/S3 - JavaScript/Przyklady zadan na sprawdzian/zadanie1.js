/*
  Zadanie 1 (10p.)
  Napisz wyrażenie funkcyjne i przypisz do stałej o nazwie "reverse". 
  Wyrażenie funkcyjne w argumencie otrzymuje tablicę.
  Zadaniem funkcji jest zwrócić odwróconą tablicę za pomocą dowolnej pętli.

  📥 Przykładowe wejście: [10, 20, 30]
  📥 Przykładowe wyjście: [30, 20, 10]
  
  ⛔️ Zabronionym jest użycie gotowej funkcji Array.prototype.reverse.
*/

const reverse = function (arr) {
  const result = [];
  for (let index = arr.length - 1; index >= 0; index--) {
    result.push(arr[index]);
  }
  return result;
};
