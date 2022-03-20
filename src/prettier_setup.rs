use std::fs::{self, File};
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;
use std::process::Command;
use std::result::Result;

use inquire::Select;

use crate::utility::{copy_file_contents_to_another_file, install_deps, read_lines, CustomResult};

pub fn run() {
    prettier_install_deps().unwrap();

    copy_file_contents_to_another_file(
        &String::from("./templates/migrates/tsconfig.json"),
        &String::from(".tsconfig.json"),
    )
    .unwrap();
}

fn prettier_install_deps() -> CustomResult<()> {
    let mut deps: Vec<String> = vec![];
    match get_deps() {
        Err(err) => eprintln!("{}", err),
        Ok(lines) => {
            for line in lines {
                if let Ok(ip) = line {
                    deps.push(ip);
                }
            }
        }
    }

    install_deps(deps)
}

fn get_deps() -> CustomResult<Lines<BufReader<File>>> {
    let path = String::from("./templates/migrate/dependencies");

    read_lines(&path)
}
