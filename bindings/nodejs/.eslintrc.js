const typescriptEslintRules = {
    '@typescript-eslint/ban-ts-comment': [
        'error',
        { 'ts-ignore': 'allow-with-description' },
    ],
    '@typescript-eslint/no-empty-interface': 'off',
    '@typescript-eslint/no-var-requires': 'off', // cleanest way to set dotenv path
    '@typescript-eslint/no-explicit-any': 'off',
    "@typescript-eslint/no-floating-promises": ["error"],
};

module.exports = {
    env: {
        node: true,
        commonjs: true,
        es2019: true,
    },
    plugins: ['@typescript-eslint'],
    extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended',
        'prettier',
    ],
    parser: '@typescript-eslint/parser',
    parserOptions: {
        ecmaVersion: 12,
        sourceType: 'module',
        project: ["./tsconfig.json"],
    },
    rules: typescriptEslintRules,
};
