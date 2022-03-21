use inquire::{MultiSelect, Select};

use setups::{eslint_setup, migrate_setup};

use crate::constants::{
    EDITORCONFIG, ESLINT, MIGRATE_JS_TO_TS_OPTION, PRETTIER, SETUP_REACT_PROJECT_OPTION,
    SETUP_TS_PROJECT_OPTION,
};
use crate::setups::{editorconfig_setup, prettier_setup};

mod constants;
mod setups;
mod utility;

fn main() {
    let top_level_options = vec![
        SETUP_TS_PROJECT_OPTION,
        SETUP_REACT_PROJECT_OPTION,
        MIGRATE_JS_TO_TS_OPTION,
    ];

    let ans = Select::new("Select your config action:", top_level_options).prompt();

    match ans {
        Ok(option) => {
            match option {
                SETUP_TS_PROJECT_OPTION => setup_options("eslint-ts"),
                SETUP_REACT_PROJECT_OPTION => setup_options("eslint-tsx"),
                MIGRATE_JS_TO_TS_OPTION => migrate_setup::run(),
                _ => panic!("Invalid option selected!"),
            };
        }
        Err(_) => println!("The config action could not be acted upon"),
    }
}

fn setup_options(setup_type: &str) {
    let mid_level_options = vec![ESLINT, PRETTIER, EDITORCONFIG];

    let ans = MultiSelect::new("Select services to add:", mid_level_options).prompt();

    match ans {
        Ok(options) => {
            for option in options {
                match option {
                    ESLINT => eslint_setup::run(&setup_type),
                    PRETTIER => prettier_setup::run(),
                    EDITORCONFIG => editorconfig_setup::run(),
                    _ => eprintln!("something bad happened..."),
                }
            }
        }
        Err(_) => {
            println!("Something wrong happened!")
        }
    }
}
