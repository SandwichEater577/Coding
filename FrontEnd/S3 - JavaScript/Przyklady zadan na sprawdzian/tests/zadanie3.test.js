const rewire = require(`rewire`);
const {
  isFunctionDeclaration,
  hasOnlyLetOrConst,
  hasVariableInCamelCase,
} = require(`./utils`);
const zadanie1 = rewire(`../zadanie3.js`);

const fnName = `removeByType`;
let fn;
try {
  fn = zadanie1.__get__(fnName);
} catch (error) {
  fn = undefined;
}

describe(`Zadanie 3`, () => {
  it(`Użyto odpowiedniej nazwy`, () => {
    expect(fn.name).toEqual(fnName);
  });

  it(`${fnName} powinno być funkcją`, () => {
    expect(typeof fn).toEqual(`function`);
  });

  it(`${fnName} powinno zostać stworzone za pomocą deklaracji funkcji`, () => {
    expect(isFunctionDeclaration(fn)).toEqual(true);
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

  it(`${fnName} powinno nie korzystać z Array.prototype.filter`, () => {
    expect(
      typeof fn === `function` && !fn.toString().includes(`.filter(`)
    ).toEqual(true);
  });

  it(`${fnName} powinno zwracać tablicę`, () => {
    expect(typeof fn === `function` && Array.isArray(fn([0, 1, 2], 1))).toEqual(
      true
    );
  });

  it(`${fnName}([10, "orange", true, "apple", {}], "string") powinno zwracać [10, true, {}]`, () => {
    expect(
      typeof fn === `function` &&
        fn([10, `orange`, true, `apple`, {}], `string`)
    ).toEqual([10, true, {}]);
  });

  it(`${fnName}([10, "orange", true, "apple", {}], "number") powinno zwracać ["orange", true, "apple", {}]`, () => {
    expect(
      typeof fn === `function` &&
        fn([10, `orange`, true, `apple`, {}], `number`)
    ).toEqual([`orange`, true, `apple`, {}]);
  });

  it(`${fnName}([10, "orange", true, "apple", {}], "boolean") powinno zwracać [10, "orange", "apple", {}]`, () => {
    expect(
      typeof fn === `function` &&
        fn([10, `orange`, true, `apple`, {}], `boolean`)
    ).toEqual([10, `orange`, `apple`, {}]);
  });
});
