// results.js
export function getPassedNames(students) {
  return students.filter(({ points }) => points >= 10).map(({ name }) => name);
}

// index.js
// W osobnym pliku index.js użyj:
// import { getPassedNames } from "./results.js";
