module.exports = {
  root: true,
  env: {
    browser: true,
    node: true
  },
  parserOptions: {
    parser: "babel-eslint"
  },
  extends: [
    "eslint:recommended",
    "plugin:vue/strongly-recommended",
    "plugin:prettier/recommended"
  ],
  plugins: ["vue"],
  rules: {
    "no-console": "off",
    "prettier/prettier": "error"
  }
};
