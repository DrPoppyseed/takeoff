use std::fs::File;
use std::io::{BufReader, Lines};

use crate::utility::{
    copy_file_contents_to_another_file, get_line_contents, install_deps, read_lines, CustomResult,
};

pub fn run() {
    migrate_install_deps().unwrap();

    copy_file_contents_to_another_file(
        &String::from("./templates/migrate-tsx/tsconfig.json"),
        &String::from(".tsconfig.json"),
    )
    .unwrap();
}

fn migrate_install_deps() -> CustomResult<()> {
    let deps = get_line_contents(get_deps());

    install_deps(deps)
}

fn get_deps() -> CustomResult<Lines<BufReader<File>>> {
    let path = String::from("./templates/migrate-tsx/dependencies");

    read_lines(&path)
}
