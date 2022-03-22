use std::fs::File;
use std::io::{BufReader, Lines};

use crate::utility::{
    copy_file_contents_to_another_file, get_line_contents, install_deps, read_lines, CustomResult,
};

pub fn run(setup_type: &str) {
    migrate_install_deps(setup_type).unwrap();

    copy_file_contents_to_another_file(&get_path(setup_type), &String::from("tsconfig.json"))
        .unwrap();
}

fn migrate_install_deps(setup_type: &str) -> CustomResult<()> {
    let deps = get_line_contents(get_deps(setup_type));

    install_deps(deps)
}

fn get_deps(setup_type: &str) -> CustomResult<Lines<BufReader<File>>> {
    let mut path = String::from("./templates/migrate-");
    path.push_str(setup_type);
    path.push_str("/dependencies");

    read_lines(&path)
}

fn get_path(setup_type: &str) -> String {
    let mut path = String::from("./templates/migrate-");
    path.push_str(setup_type);
    path.push_str("/tsconfig.json");

    path
}
