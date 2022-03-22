use inquire::{MultiSelect, Select};

use setups::{eslint_setup, license_setup, migrate_setup};

use crate::constants::{
    ADD_LICENSE, EDITORCONFIG, ESLINT, HUSKY, MIGRATE_JS_REACT_PROJECT_TO_OPTION,
    MIGRATE_JS_TO_TS_OPTION, PRETTIER, SETUP_REACT_PROJECT_OPTION, SETUP_TS_PROJECT_OPTION,
};
use crate::setups::{editorconfig_setup, husky_setup, prettier_setup};

mod constants;
mod setups;
mod utility;

fn main() {
    let top_level_options = vec![
        SETUP_TS_PROJECT_OPTION,
        SETUP_REACT_PROJECT_OPTION,
        MIGRATE_JS_TO_TS_OPTION,
        MIGRATE_JS_REACT_PROJECT_TO_OPTION,
        ADD_LICENSE,
    ];

    let ans = Select::new("Select your config action:", top_level_options).prompt();

    match ans {
        Err(err) => eprintln!("{:?}", err),
        Ok(option) => {
            match option {
                SETUP_TS_PROJECT_OPTION => setup_options("ts"),
                SETUP_REACT_PROJECT_OPTION => setup_options("tsx"),
                MIGRATE_JS_TO_TS_OPTION => migrate_setup::run("ts"),
                MIGRATE_JS_REACT_PROJECT_TO_OPTION => migrate_setup::run("tsx"),
                ADD_LICENSE => license_setup::run(),
                _ => panic!("Invalid option selected!"),
            };
        }
    }
}

fn setup_options(setup_type: &str) {
    let mid_level_options = vec![ESLINT, PRETTIER, EDITORCONFIG, HUSKY];

    let ans = MultiSelect::new("Select services to add:", mid_level_options).prompt();

    match ans {
        Err(err) => eprintln!("{:?}", err),
        Ok(options) => {
            for option in options {
                match option {
                    ESLINT => eslint_setup::run(&setup_type),
                    PRETTIER => prettier_setup::run(),
                    EDITORCONFIG => editorconfig_setup::run(),
                    HUSKY => husky_setup::run(&setup_type),
                    _ => panic!("something bad happened..."),
                }
            }
        }
    }
}
