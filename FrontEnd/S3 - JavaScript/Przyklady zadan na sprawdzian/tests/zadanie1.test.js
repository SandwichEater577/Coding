const rewire = require(`rewire`);
const {
  isFunctionExpression,
  hasOnlyLetOrConst,
  hasVariableInCamelCase,
} = require(`./utils`);
const zadanie1 = rewire(`../zadanie1.js`);

const fnName = `reverse`;
let fn;
try {
  fn = zadanie1.__get__(fnName);
} catch (error) {
  fn = undefined;
}

describe(`Zadanie 1`, () => {
  it(`Użyto odpowiedniej nazwy`, () => {
    expect(fn?.name).toEqual(fnName);
  });

  it(`${fnName} powinno być funkcją`, () => {
    expect(typeof fn).toEqual(`function`);
  });

  it(`${fnName} powinno zostać stworzone za pomocą wyrazenia funkcyjnego`, () => {
    expect(isFunctionExpression(fn)).toEqual(true);
  });

  it(`${fnName} powinno przyjmować jeden argument`, () => {
    expect(fn?.length).toEqual(1);
  });

  it(`${fnName} powinno nie zawierać zmiennych zadeklarowanych za pomocą "var"`, () => {
    expect(hasOnlyLetOrConst(fn)).toEqual(true);
  });

  it(`${fnName} powinno zawierać nazwy zmiennych zgodne z konwencją camel case.`, () => {
    expect(hasVariableInCamelCase(fn)).toEqual(true);
  });

  it(`${fnName} powinno nie korzystać z Array.prototype.reverse`, () => {
    expect(fn?.toString().includes(`.reverse(`)).toEqual(false);
  });

  it(`${fnName} powinno zwracać tablicę`, () => {
    expect(typeof fn === 'function' && Array.isArray(fn([]))).toEqual(false);
  });

  it(`${fnName} powinno zwracać tablicę z taką samą ilością elementów co tablica wejściowa`, () => {
    expect(
      typeof fn === 'function' &&
        fn([10, 20, 30, 40, 50, 60, 70, 80, 90]).length
    ).toEqual(9);
  });

  it(`${fnName}([]) powinno zwrócić []`, () => {
    expect(typeof fn === 'function' && fn([])).toEqual([]);
  });

  it(`${fnName}([10, 20, 30]) powinno zwrócić [30, 20, 10]`, () => {
    expect(typeof fn === 'function' && fn([10, 20, 30])).toEqual([30, 20, 10]);
  });

  it(`${fnName}([-2, -1, 0]) powinno zwrócić [0, -1, -2]`, () => {
    expect(typeof fn === 'function' && fn([-2, -1, 0])).toEqual([0, -1, -2]);
  });

  it(`${fnName}(["a", "b", "c"]) powinno zwrócić ["c", "b", "a"]`, () => {
    expect(typeof fn === 'function' && fn([`a`, `b`, `c`])).toEqual([
      `c`,
      `b`,
      `a`,
    ]);
  });

  it(`${fnName}([null, null, undefined, 1]) powinno zwrócić [1, undefined, null, null]`, () => {
    expect(typeof fn === 'function' && fn([null, null, undefined, 1])).toEqual([
      1,
      undefined,
      null,
      null,
    ]);
  });
});
