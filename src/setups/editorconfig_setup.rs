use crate::utility::string_to_file;

const EDITOR_CONFIG: &str = "\
root = true
[*]
charset = utf-8
end_of_line = lf
indent_style = tab
trim_trailing_whitespace = true
indent_size = 2
";

pub fn run() {
  string_to_file(&EDITOR_CONFIG.to_string(), ".editorconfig").unwrap();
}
