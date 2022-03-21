use std::fs::File;
use std::io::{BufReader, Lines};

use crate::utility::{
    copy_file_contents_to_another_file, get_line_contents, install_deps, read_lines, CustomResult,
};

pub fn run(setup_type: &str) {
    eslint_install_deps(setup_type).unwrap();

    let eslintrc_out_path = String::from(".eslintrc.json");
    let eslintrc_in_path = path_to_eslintrc(setup_type);
    copy_file_contents_to_another_file(&eslintrc_in_path, &eslintrc_out_path).unwrap();

    let eslintignore_out_path = String::from(".eslintignore");
    let eslintignore_in_path = path_to_eslintignore(setup_type);
    copy_file_contents_to_another_file(&eslintignore_in_path, &eslintignore_out_path).unwrap();
}

fn eslint_install_deps(setup_type: &str) -> CustomResult<()> {
    // get dependencies
    let deps = get_line_contents(get_deps(setup_type));

    install_deps(deps)
}

fn get_deps(setup_type: &str) -> CustomResult<Lines<BufReader<File>>> {
    let mut path = String::from("./templates/");
    path.push_str(setup_type);
    path.push_str("/dependencies");

    read_lines(&path)
}

fn path_to_eslintrc(setup_type: &str) -> String {
    let mut path = String::from("./templates/");
    path.push_str(setup_type);
    path.push_str("/.eslintrc.json");
    return path;
}

fn path_to_eslintignore(setup_type: &str) -> String {
    let mut path = String::from("./templates/");
    path.push_str(setup_type);
    path.push_str("/.eslintignore");
    return path;
}
