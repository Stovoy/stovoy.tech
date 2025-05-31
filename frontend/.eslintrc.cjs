module.exports = {
  root: true,
  parser: '@typescript-eslint/parser',
  plugins: ['svelte', '@typescript-eslint'],
  extends: [
    'eslint:recommended',
    'plugin:svelte/recommended',
    'plugin:@typescript-eslint/recommended',
    'prettier',
  ],
  overrides: [
    {
      files: ['*.svelte'],
      processor: 'svelte/svelte',
    },
  ],
  env: {
    browser: true,
    es2020: true,
    node: true,
  },
  settings: {
    'svelte3/typescript': () => require('typescript'),
  },
};