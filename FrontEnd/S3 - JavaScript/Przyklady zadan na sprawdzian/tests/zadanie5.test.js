const rewire = require('rewire');
const {
  isArrowFunction,
  isFunctionDeclaration,
  hasOnlyLetOrConst,
  hasDefaultParam,
  hasVariableInCamelCase,
  hasConditionalOperator,
} = require('./utils');
const zadanie1 = rewire('../zadanie5.js');

const fnName = 'makeTranslate';
let fn;
try {
  fn = zadanie1.__get__(fnName);
} catch (error) {
  fn = undefined;
}

describe('Zadanie 5', () => {
  it('Użyto odpowiedniej nazwy', () => {
    expect(fn.name).toEqual(fnName);
  });

  it(`${fnName} powinno być funkcją`, () => {
    expect(typeof fn).toEqual('function');
  });

  it(`${fnName} powinno zostać stworzone za pomocą deklaracji funkcji`, () => {
    expect(isFunctionDeclaration(fn)).toEqual(true);
  });

  it(`${fnName} powinien przyjmować jeden argument, który ma domyślną wartość`, () => {
    expect(hasDefaultParam(fn)).toEqual(true);
  });

  it(`${fnName} nie zawiera zmiennych zadeklarowanych za pomocą "var"`, () => {
    expect(hasOnlyLetOrConst(fn)).toEqual(true);
  });

  it(`${fnName} powinno zawierać nazwy zmiennych zgodne z konwencją camel case.`, () => {
    expect(hasVariableInCamelCase(fn)).toEqual(true);
  });

  it(`${fnName} powinno zwracać funkcję`, () => {
    expect(typeof fn === 'function' && typeof fn()).toEqual('function');
  });

  it(`${fnName} powinno zwracać funkcję strzałkową`, () => {
    expect(isArrowFunction(fn())).toEqual(true);
  });

  it(`${fnName} powinno zwracać funkcję, która przyjmuje jeden argument`, () => {
    expect(typeof fn === 'function' && fn().length).toEqual(1);
  });

  it(`${fnName} powinno zwracać funkcję, która zawiera operator warunkowy`, () => {
    expect(hasConditionalOperator(fn())).toEqual(true);
  });

  it(`${fnName}()("header") powinno zwracać "Dzień dobry!"`, () => {
    expect(typeof fn === 'function' && fn()('header')).toEqual('Dzień dobry!');
  });

  it(`${fnName}("pl")("header") powinno zwracać "Dzień dobry!"`, () => {
    expect(typeof fn === 'function' && fn('pl')('header')).toEqual(
      'Dzień dobry!'
    );
  });

  it(`${fnName}("pl")("main") powinno zwracać "Nasze produkty"`, () => {
    expect(typeof fn === 'function' && fn('pl')('main')).toEqual(
      'Nasze produkty'
    );
  });

  it(`${fnName}("pl")("footer") powinno zwracać "Partnerzy"`, () => {
    expect(typeof fn === 'function' && fn('pl')('footer')).toEqual('Partnerzy');
  });

  it(`${fnName}("en")("header") powinno zwracać "Good morning!"`, () => {
    expect(typeof fn === 'function' && fn('en')('header')).toEqual(
      'Good morning!'
    );
  });

  it(`${fnName}("en")("main") powinno zwracać "Our products"`, () => {
    expect(typeof fn === 'function' && fn('en')('main')).toEqual(
      'Our products'
    );
  });

  it(`${fnName}("en")("footer") powinno zwracać "Our products"`, () => {
    expect(typeof fn === 'function' && fn('en')('footer')).toEqual('Partners');
  });

  it(`${fnName}()() powinno zwracać "Nie znaleziono"`, () => {
    expect(typeof fn === 'function' && fn()()).toEqual('Nie znaleziono');
  });

  it(`${fnName}("pl")() powinno zwracać "Nie znaleziono"`, () => {
    expect(typeof fn === 'function' && fn('pl')()).toEqual('Nie znaleziono');
  });

  it(`${fnName}("pl")("xyz") powinno zwracać "Nie znaleziono"`, () => {
    expect(typeof fn === 'function' && fn('pl')('xyz')).toEqual(
      'Nie znaleziono'
    );
  });

  it(`${fnName}("en")() powinno zwracać "Not found"`, () => {
    expect(typeof fn === 'function' && fn('en')()).toEqual('Not found');
  });

  it(`${fnName}("en")("xyz") powinno zwracać "Not found"`, () => {
    expect(typeof fn === 'function' && fn('en')('xyz')).toEqual('Not found');
  });
});
