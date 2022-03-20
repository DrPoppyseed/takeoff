use std::fs::File;
use std::io::{BufReader, Lines};

use inquire::Select;

use crate::utility::{copy_file_contents_to_another_file, install_deps, read_lines, CustomResult};

pub fn run(setup_type: &str) {
    let eslintrc_file_formats = vec!["yml", "json"];

    let eslintrc_file_format = Select::new(
        "select file format for the eslintrc file:",
        eslintrc_file_formats,
    )
    .prompt();

    match eslintrc_file_format {
        Ok(format) => {
            configure(&format, &setup_type);
        }
        Err(_) => println!("file format invalid?"),
    }
}

fn eslint_install_deps(setup_type: &str) -> CustomResult<()> {
    // get dependencies
    let mut deps: Vec<String> = vec![];
    match get_deps(setup_type) {
        Err(err) => eprintln!("{}: {}", setup_type, err),
        Ok(lines) => {
            for line in lines {
                if let Ok(ip) = line {
                    deps.push(ip);
                }
            }
        }
    };

    install_deps(deps)
}

fn get_deps(setup_type: &str) -> CustomResult<Lines<BufReader<File>>> {
    let mut path = String::from("./templates/");
    path.push_str(setup_type);
    path.push_str("/dependencies");

    read_lines(&path)
}

fn configure(format: &str, setup_type: &str) {
    eslint_install_deps(setup_type).unwrap();

    let mut eslintrc_out_path = String::from(".eslintrc.");
    eslintrc_out_path.push_str(format);
    let eslintrc_in_path = path_to_eslintrc(format, setup_type);
    copy_file_contents_to_another_file(&eslintrc_in_path, &eslintrc_out_path).unwrap();

    let eslintignore_out_path = String::from(".eslintignore");
    let eslintignore_in_path = path_to_eslintignore(setup_type);
    copy_file_contents_to_another_file(&eslintignore_in_path, &eslintignore_out_path).unwrap();
}

fn path_to_eslintrc(format: &str, setup_type: &str) -> String {
    let mut path = String::from("./templates/");
    path.push_str(setup_type);
    path.push_str("/.eslintrc.");
    path.push_str(format);
    return path;
}

fn path_to_eslintignore(setup_type: &str) -> String {
    let mut path = String::from("./templates/");
    path.push_str(setup_type);
    path.push_str("/.eslintignore");
    return path;
}
