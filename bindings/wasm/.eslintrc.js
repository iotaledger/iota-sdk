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
        project: ["./tsconfig.eslint.json"],
    },
    rules: {
        '@typescript-eslint/no-var-requires': 0,
        '@typescript-eslint/ban-ts-comment': [
            'error',
            { 'ts-ignore': 'allow-with-description' },
        ],
        '@typescript-eslint/no-explicit-any': 'off',
        "@typescript-eslint/no-floating-promises": ["error"],
    },
};
