// Prettier configuration for JavaScript/TypeScript files
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
  importOrder: ['^react/(.*)$', '^@/(.*)$', '^[a-z]'],
  importOrderType: 'namespace',
  plugins: ['prettier-plugin-tailwindcss']
};