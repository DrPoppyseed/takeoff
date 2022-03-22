use crate::utility::{install_deps, string_to_file, CustomResult};

const TS_ESLINTRC: &str = r##"{
  "root": true,
  "env": {
    "node": true,
    "commonjs": true,
    "browser": true,
    "es6": true
  },
  "parser": "@typescript-eslint/parser",
  "plugins": ["@typescript-eslint"],
  "extends": [
    "eslint:recommended",
    "plugin:@typescript-eslint/eslint-recommended",
    "plugin:@typescript-eslint/recommended"
  ],
  "rules": {
    "@typescript-eslint/no-unused-vars": "warn",
    "@typescript-eslint/no-explicit-any": "warn",
    "@typescript-eslint/no-non-null-assertion": "warn",
    "@typescript-eslint/no-var-requires": "warn",
    "@typescript-eslint/ban-ts-comment": "off"
  }
}
"##;
const TSX_ESLINTRC: &str = r##"{
  "env": {
    "browser": true,
    "es2021": true
  },
  "extends": [
    "airbnb",
    "prettier",
    "eslint:recommended",
    "airbnb-typescript",
    "plugin:react/recommended",
    "plugin:react-hooks/recommended",
    "plugin:prettier/recommended"
  ],
  "parser": "@typescript-eslint/parser",
  "parserOptions": {
    "ecmaFeatures": {
      "jsx": true
    },
    "ecmaVersion": 12,
    "sourceType": "module",
    "project": "./tsconfig.json"
  },
  "plugins": [
    "react",
    "react-hooks",
    "prettier",
    "@typescript-eslint"
  ],
  "rules": {
    "import/order": "warn",
    "import/prefer-default-export": "off",
    "import/no-extraneous-dependencies": "warn",
    "consistent-return": "warn",
    "no-console": "warn",
    "no-plusplus": "off",
    "no-return-assign": "warn",
    "no-nested-ternary": "off",
    "no-param-reassign": "off",
    "no-unused-expressions": "warn",
    "no-unused-vars": "warn",
    "no-restricted-exports": "off",
    "react/jsx-no-constructed-context-values": "warn",
    "react/jsx-props-no-spreading": "off",
    "react/no-this-in-sfc": "warn",
    "react/require-default-props": "off",
    "react/jsx-curly-brace-presence": "warn",
    "react/function-component-definition": [
      "off",
      {
        "namedComponents": "function-declaration"
      }
    ],
    "react-hooks/exhaustive-deps": "warn",
    "@typescript-eslint/no-unused-vars": "warn",
    "@typescript-eslint/no-unused-expressions": "warn",
    "@typescript-eslint/no-use-before-define": "off",
    "jsx-a11y/no-noninteractive-element-interactions": "warn",
    "jsx-a11y/click-events-have-key-events": "warn",
    "jsx-a11y/label-has-associated-control": "warn",
    "jsx-a11y/no-static-element-interactions": "warn"
  }
}
"##;

const TS_ESLINTIGNORE: &str = r##"\
/node_modules
/coverage
.yarn
**/*.js
"##;
const TSX_ESLINTIGNORE: &str = r##"\
/node_modules
/public
/coverage
.yarn
**/*.js
"##;

pub fn run(setup_type: &str) {
  eslint_install_deps(setup_type).unwrap();

  let eslintrc = if setup_type.eq("ts") {
    TS_ESLINTRC
  } else {
    TSX_ESLINTRC
  };

  let eslintignore = if setup_type.eq("ts") {
    TS_ESLINTIGNORE
  } else {
    TSX_ESLINTIGNORE
  };

  string_to_file(eslintrc, ".eslintrc.json").unwrap();

  string_to_file(eslintignore, ".eslintignore").unwrap();
}

fn eslint_install_deps(setup_type: &str) -> CustomResult<()> {
  let deps = if setup_type.eq("ts") {
    vec![
      "@typescript-eslint/eslint-plugin",
      "@typescript-eslint/parser",
      "eslint",
    ]
  } else {
    vec![
      "@typescript-eslint/eslint-plugin",
      "@typescript-eslint/parser",
      "eslint-config-airbnb-typescript",
      "eslint-config-airbnb",
      "eslint-config-prettier",
      "eslint-plugin-react-hooks",
      "eslint-plugin-jsx-a11y",
      "eslint-plugin-prettier",
      "eslint-plugin-import",
      "eslint-plugin-react",
      "eslint",
    ]
  };

  install_deps(deps)
}
