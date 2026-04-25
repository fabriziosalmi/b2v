// prettier configuration
module.exports = {
  semi: false,
  singleQuote: true,
  trailingComma: 'es5',
  printWidth: 80,
  tabWidth: 2,
  endOfLine: 'auto',
  arrowParens: 'always',
  proseWrap: 'never',
  bracketSpacing: true,
  importOrder: ['^@\w+', '^[.\/](.*)', '^@/(.*\.ts)$'],
  importOrderMergeWhenImportsAreAdjacent: true
};