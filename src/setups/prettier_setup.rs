use crate::utility::{install_deps, string_to_file};

const PRETTIER: &str = r##"{
  "singleQuote": true,
  "arrowParens": "avoid",
  "semi": false,
  "tabWidth": 2,
  "jsxSingleQuote": true
}
"##;

pub fn run() {
  install_deps(vec!["prettier"]).unwrap();
  string_to_file(PRETTIER, ".prettierrc").unwrap();
}
