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
        semi: ["error", "never"]
    },
    settings: {
        'import/resolver': 'webpack',
    },
    env: {
        mocha: true,
        browser: true,
    },
};
