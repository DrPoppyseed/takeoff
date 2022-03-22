use inquire::{Select, Text};
use std::fs;

use crate::utility::CustomResult;
use crate::{constants::MIT_LICENSE, utility::read_lines};

pub fn run() {
    let license_options = vec![MIT_LICENSE];

    let ans = Select::new("Select the license type to add:", license_options).prompt();

    match ans {
        Err(err) => eprintln!("{:?}", err),
        Ok(option) => match option {
            MIT_LICENSE => setup_mit_license().unwrap(),
            _ => panic!("Invalid option selected!"),
        },
    }
}

fn setup_mit_license() -> CustomResult<()> {
    // 1. get name from user
    let name = Text::new("Enter name to use in license:").prompt();

    // 1a retrieve file contents of license
    let lines = read_lines(&"./templates/license-mit/LICENSE".to_string()).unwrap();

    let mut res = String::new();
    match name {
        Err(err) => eprintln!("Unable to retrieve your name. Error: {:?}", err),
        Ok(name) => {
            // 2. add retrieved name and add to new string with license info
            for (i, line) in lines.map(|x| x.unwrap()).enumerate() {
                if i == 2 {
                    // convert type
                    let copyright_line: &str = &*build_copyright_line(&name).to_owned();
                    res.push_str(copyright_line);
                } else {
                    let normal_line: &str = &*line.to_owned();
                    res.push_str(normal_line);
                }
                res.push_str("\n");
            }
        }
    }

    // 3. write file
    fs::write("./LICENSE", res).expect("Unable to write file");
    Ok(())
}

fn build_copyright_line(name: &str) -> String {
    let mut copyright_line = String::from("Copyright (c) ");
    copyright_line.push_str(name);
    copyright_line
}
