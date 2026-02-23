const rewire = require('rewire');
const {
  hasOnlyLetOrConst,
  hasVariableInCamelCase,
  isFunctionExpression,
} = require('./utils');
const zadanie1 = rewire('../zadanie6.js');

const fnName = 'findWinner';
let fn;
try {
  fn = zadanie1.__get__(fnName);
} catch (error) {
  fn = undefined;
}

describe('Zadanie 6', () => {
  it('Użyto odpowiedniej nazwy', () => {
    expect(fn.name).toEqual(fnName);
  });

  it(`${fnName} powinno być funkcją`, () => {
    expect(typeof fn).toEqual('function');
  });

  it(`${fnName} powinno zostać stworzone za pomocą wyrażenia funkcyjnego`, () => {
    expect(isFunctionExpression(fn)).toEqual(true);
  });

  it(`${fnName} powinien przyjmować jeden argument`, () => {
    expect(typeof fn === 'function' && fn.length).toEqual(1);
  });

  it(`${fnName} powinno nie zawierać zmiennych zadeklarowanych za pomocą "var"`, () => {
    expect(hasOnlyLetOrConst(fn)).toEqual(true);
  });

  it(`${fnName} powinno zawierać nazwy zmiennych zgodne z konwencją camel case.`, () => {
    expect(hasVariableInCamelCase(fn)).toEqual(true);
  });

  it(`
${fnName}([  
  ["O",    "O",    "X"],
  ["O",    "X",    "O"],
  ["X", undefined, "O"]
]) powinno zwrócić "X"`, () => {
    expect(
      fn([
        ['O', 'O', 'X'],
        ['O', 'X', 'O'],
        ['X', undefined, 'O'],
      ])
    ).toEqual('X');
  });

  it(`
${fnName}([  
  ["O", "X", "X"],
  ["O", "X", "O"],
  ["O", "O", "X"]
]) powinno zwrócić "O"`, () => {
    expect(
      fn([
        ['O', 'X', 'X'],
        ['O', 'X', 'O'],
        ['O', 'O', 'X'],
      ])
    ).toEqual('O');
  });

  it(`
${fnName}([  
  ["X", "O", "X"],
  ["O", "O", "X"],
  ["O", "X", "O"]
]) powinno zwrócić null`, () => {
    expect(
      fn([
        ['X', 'O', 'X'],
        ['O', 'O', 'X'],
        ['O', 'X', 'O'],
      ])
    ).toEqual(null);
  });

  it(`
${fnName}([  
  ["O",       undefined, "X"      ],
  [undefined, "X",       undefined],
  ["O",       undefined, "O"      ]
]) powinno zwrócić undefined`, () => {
    expect(
      fn([
        ['O', undefined, 'X'],
        [undefined, 'X', undefined],
        ['O', undefined, 'O'],
      ])
    ).toEqual(undefined);
  });
});
