const OFF = 0, WARN = 1, ERROR = 2;

module.exports =
{
    parser: '@typescript-eslint/parser',
    parserOptions: {
        project: './tsconfig.app.json',
        tsconfigRootDir: __dirname,
        sourceType: 'module',
    },
    plugins: ['@typescript-eslint/eslint-plugin', 'prettier'],
    extends: [
        'plugin:@typescript-eslint/recommended',
        'plugin:prettier/recommended',
    ],
    root: true,

    env: {
        node: true,
        jest: true,
    },
    ignorePatterns: ['vite.config.ts', 'tailwind.config.js', 'postcss.config.js'],
    rules: {
        'no-console': ERROR,
        'no-restricted-syntax': [
            ERROR,
            {
                'selector': 'CallExpression[callee.object.name=\'console\'][callee.property.name!=/^(log|warn|error|info|trace)$/]',
                'message': 'Unexpected property on console object was called'
            }
        ],
        '@typescript-eslint/explicit-function-return-type': ERROR,
        '@typescript-eslint/explicit-module-boundary-types': ERROR,
        '@typescript-eslint/no-explicit-any': ERROR,
        "no-unused-vars": OFF,
        "@typescript-eslint/no-unused-vars": [
            ERROR,
            {
                "argsIgnorePattern": "^_",
                "varsIgnorePattern": "^_",
                "caughtErrorsIgnorePattern": "^_"
            }
        ],
        "eqeqeq": ERROR
    },
}
