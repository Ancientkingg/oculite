const OFF = 0, WARN = 1, ERROR = 2;

module.exports =
{
    parserOptions: {
        parser: '@typescript-eslint/parser',
        project: './tsconfig.app.json',
        tsconfigRootDir: __dirname,
        sourceType: 'module',
        extraFileExtensions: ['.vue'],
    },
    plugins: ['@typescript-eslint/eslint-plugin', 'prettier', 'vue', 'disable'],
    processor: 'disable/disable',
    extends: [
        'plugin:@typescript-eslint/recommended',
        'plugin:prettier/recommended',
        'plugin:vue/vue3-recommended'
    ],
    root: true,

    overrides: [{
        files: ['src/**/*.vue'],
        settings: {
            'disable/plugins': ['prettier']
        }
    }],

    env: {
        node: true,
        jest: true,
    },
    ignorePatterns: ['vite.config.ts', 'tailwind.config.js', 'postcss.config.js', '.eslintrc.cjs'],
    rules: {
        'vue/comment-directive': OFF,
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
