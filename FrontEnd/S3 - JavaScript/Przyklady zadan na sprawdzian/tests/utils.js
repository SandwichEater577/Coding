function isFunctionDeclaration(func) {
  if (typeof func !== 'function') return false;
  const funcString = func.toString();
  return /^function\s/.test(funcString);
}

function isArrowFunction(func) {
  if (typeof func !== 'function') return false;
  const funcString = func.toString();
  return /^(\([^\)]*\))?\s*([^\s]*?)\s*=>/.test(funcString);
}

function isFunctionExpression(func) {
  if (typeof func !== 'function') return false;
  const funcString = func.toString();
  return /^function\s*\(/.test(funcString);
}

function hasOnlyLetOrConst(func) {
  if (typeof func !== 'function') return false;
  const funcString = func.toString();
  const varRegex = /\bvar\b/;
  return !varRegex.test(funcString);
}

function hasDefaultParam(func) {
  if (typeof func !== 'function') return false;
  const funcString = func.toString();
  const pattern = /\w+\s*=\s*[^,)]+/;
  return pattern.test(funcString);
}

function hasVariableInCamelCase(func) {
  if (typeof func !== 'function') return false;
  const funcString = func.toString();
  const regExp =
    /function\s*[a-zA-Z0-9_]*\s*\(([a-zA-Z][a-zA-Z0-9]*)?(\s*[a-zA-Z][a-zA-Z0-9]*\s*(,\s*[a-zA-Z][a-zA-Z0-9]*)*)*\)\s*\{/;
  const regExpArrow =
    /\(\s*([a-zA-Z][a-zA-Z0-9]*)?(\s*[a-zA-Z][a-zA-Z0-9]*\s*(,\s*[a-zA-Z][a-zA-Z0-9]*)*)*\)\s*=>/;

  return regExp.test(funcString) || regExpArrow.test(funcString);
}

function hasConditionalOperator(func) {
  if (typeof func !== 'function') return false;
  const pattern = /\?[^:]*:/;
  return pattern.test(func.toString());
}

module.exports = {
  isArrowFunction,
  isFunctionExpression,
  isFunctionDeclaration,
  hasDefaultParam,
  hasOnlyLetOrConst,
  hasVariableInCamelCase,
  hasConditionalOperator,
};
