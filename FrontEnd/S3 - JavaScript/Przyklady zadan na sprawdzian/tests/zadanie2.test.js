const rewire = require(`rewire`);
const {
  isArrowFunction,
  hasOnlyLetOrConst,
  hasVariableInCamelCase,
} = require(`./utils`);
const zadanie1 = rewire(`../zadanie2.js`);

const fnName = `findIndex`;
let fn;
try {
  fn = zadanie1.__get__(fnName);
} catch (error) {
  fn = undefined;
}

describe(`Zadanie 2`, () => {
  it(`Użyto odpowiedniej nazwy`, () => {
    expect(fn.name).toEqual(fnName);
  });

  it(`${fnName} powinno być funkcją`, () => {
    expect(typeof fn).toEqual(`function`);
  });

  it(`${fnName} powinno zostać stworzone za pomocą funkcji strzałkowej`, () => {
    expect(isArrowFunction(fn)).toEqual(true);
  });

  it(`${fnName} powinno przyjmować dwa argumenty`, () => {
    expect(typeof fn === `function` && fn.length).toEqual(2);
  });

  it(`${fnName} powinno nie zawierać zmiennych zadeklarowanych za pomocą "var"`, () => {
    expect(hasOnlyLetOrConst(fn)).toEqual(true);
  });

  it(`${fnName} powinno zawierać nazwy zmiennych zgodne z konwencją camel case.`, () => {
    expect(hasVariableInCamelCase(fn)).toEqual(true);
  });

  it(`${fnName} powinno nie korzystać z Array.prototype.findIndex`, () => {
    expect(
      typeof fn === `function` && !fn.toString().includes(`.findIndex(`)
    ).toEqual(true);
  });

  it(`${fnName} powinno zwracać liczbę`, () => {
    expect(typeof fn === `function` && typeof fn([0, 1, 2], 1)).toEqual(
      `number`
    );
  });

  it(`${fnName}([], 1) powinno zwracać -1`, () => {
    expect(typeof fn === `function` && fn([], 1)).toEqual(-1);
  });

  it(`${fnName}([10, 20, 30], 10) powinno zwracać 0`, () => {
    expect(typeof fn === `function` && fn([10, 20, 30], 10)).toEqual(0);
  });

  it(`${fnName}([10, 20, 30], 20) powinno zwracać 1`, () => {
    expect(typeof fn === `function` && fn([10, 20, 30], 20)).toEqual(1);
  });

  it(`${fnName}([10, 20, 30], 30) powinno zwracać 2`, () => {
    expect(typeof fn === `function` && fn([10, 20, 30], 30)).toEqual(2);
  });

  it(`${fnName}([10, 20, 20, 20, 30, 20], 20) powinno zwracać 1`, () => {
    expect(
      typeof fn === `function` && fn([10, 20, 20, 20, 30, 20], 20)
    ).toEqual(1);
  });
});
