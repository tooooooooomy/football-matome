module.exports = {
    parser: 'babel-eslint',
    extends: "airbnb",
    installedESLint: true,
    plugins: [
        'react',
        'jsx-a11y',
        'import',
    ],
    rules: {
        'import/first': 'off',
        'jsx-a11y/label-has-for': 'off',
        semi: ['error', 'never'],
        'import/no-extraneous-dependencies': ['error', { devDependencies: ['**/*.spec.js*'] }],
        'no-alert': 'off',
        'no-console': 'off',
        'no-nested-ternary': 'off',
        'no-underscore-dangle': 'off',
        'react/forbid-prop-types': 'off',
        'react/no-multi-comp': 'off',
        'react/require-default-props': 'off',
    },
    settings: {
        'import/resolver': 'webpack',
    },
    env: {
        mocha: true,
        browser: true,
    },
};
