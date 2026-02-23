const rewire = require('rewire');
const {
  isFunctionDeclaration,
  hasOnlyLetOrConst,
  hasVariableInCamelCase,
} = require('./utils');
const zadanie1 = rewire('../zadanie4.js');

const fnName = 'analyzePersons';
let fn;
try {
  fn = zadanie1.__get__(fnName);
} catch (error) {
  fn = undefined;
}

describe('Zadanie 4', () => {
  it('Użyto odpowiedniej nazwy', () => {
    expect(fn.name).toEqual(fnName);
  });

  it(`${fnName} powinno być funkcją`, () => {
    expect(typeof fn).toEqual('function');
  });

  it(`${fnName} powinno zostać stworzone za pomocą deklaracji funkcji`, () => {
    expect(isFunctionDeclaration(fn)).toEqual(true);
  });

  it(`${fnName} powinno przyjmować jeden argument`, () => {
    expect(typeof fn === 'function' && fn.length).toEqual(1);
  });

  it(`${fnName} powinno nie zawierać zmiennych zadeklarowanych za pomocą "var"`, () => {
    expect(hasOnlyLetOrConst(fn)).toEqual(true);
  });

  it(`${fnName} powinno zawierać nazwy zmiennych zgodne z konwencją camel case.`, () => {
    expect(hasVariableInCamelCase(fn)).toEqual(true);
  });

  it(`${fnName} powinno zwracać obiekt`, () => {
    expect(typeof fn === 'function' && typeof fn([0, 1, 2], 1)).toEqual(
      'object'
    );
  });

  it(`${fnName}([{ name: "Bill", age: 10 }, { name: "John", age: 20 }, { name: "Kate", age: 30 }]) powinno zwracać { minAge: 10, avgAge: 20, maxAge: 30 }`, () => {
    expect(
      typeof fn === 'function' &&
        fn([
          { name: 'Bill', age: 10 },
          { name: 'John', age: 20 },
          { name: 'Kate', age: 30 },
        ])
    ).toEqual({ minAge: 10, avgAge: 20, maxAge: 30 });
  });

  it(`${fnName}([{ name: "Bill", age: 90 }, { name: "Anna", age: 20 }, { name: "Roger", age: 20 }, { name: "Kate", age: 20 }]) powinno zwracać { minAge: 20, avgAge: 20, maxAge: 90 }`, () => {
    expect(
      typeof fn === 'function' &&
        fn([
          { name: 'Bill', age: 40 },
          { name: 'Anna', age: 20 },
          { name: 'Roger', age: 20 },
          { name: 'Kate', age: 20 },
        ])
    ).toEqual({ minAge: 20, avgAge: 25, maxAge: 40 });
  });
});
