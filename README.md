# takeoff

Setup ESLint, Prettier, EditorConfig, etc... for a js/ts project through an interactive CLI. Takeoff installs the needed
dependencies through yarn or npm (inferred from weather the project uses package-lock.json or yarn.lock) and modifies
package.json scripts or Husky's pre-commit files accordingly.

Takeoff was written entirely with rust, with the help of the [inquire](https://github.com/mikaelmello/inquire) crate.

## Features

Takeoff supports

1. For Typescript and React-Typescript projects
    - Husky
    - ESLint
    - Prettier
    - EditorConfig
2. For Javascript and React-Javascript projects
    - Migrate project to Typescript
3. Add License
    - MIT

## License

MIT
