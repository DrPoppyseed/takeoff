use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;
use std::process::Command;

use futures::executor::block_on;
use npm_package_json::Package;
use serde_json::{from_str, Value};

use crate::utility::{
    copy_file_contents_to_another_file, get_line_contents, get_package_manager, install_deps,
    read_lines, CustomResult,
};

pub fn run(setup_type: &str) {
    block_on(husky_install_deps(setup_type));
}

async fn husky_install_deps(setup_type: &str) {
    // get dependencies
    let deps = get_line_contents(get_deps(setup_type));
    install_deps(deps).unwrap();

    let mut command = vec!["npx", "husky-init", "&&"];
    if get_package_manager().eq("npm") {
        command.push("npm");
    } else {
        command.push("yarn");
    }

    run_husky_init(command).await;

    build_lintstagedrc(setup_type, get_existing_configs())
        .await
        .unwrap();

    add_packagejson_scripts(setup_type, get_existing_configs())
        .await
        .unwrap();

    replace_last_line_in_file().unwrap();
}

async fn run_husky_init(command: Vec<&str>) {
    Command::new(&command[0])
        .args(&command[1..])
        .spawn()
        .expect("Unable to start subprocesses")
        .wait()
        .expect("Unable to wait for subprocesses");
}

// in .husky/pre-commit
fn replace_last_line_in_file() -> CustomResult<()> {
    let path = ".husky/pre-commit";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let lines = BufReader::new(file).lines();

    let mut res = String::new();
    let target = String::from("npm test");
    for line in lines.map(|x| x.unwrap()) {
        if line.ne(&target) {
            res.push_str(&line.to_string());
            res.push_str("\n");
        }
    }

    if get_package_manager().eq("npm") {
        res.push_str("npm run lint-staged");
    } else {
        res.push_str("yarn run lint-staged");
    }

    let mut file = File::create(".husky/pre-commit")?;
    file.write_all(res.as_ref())?;
    Ok(())
}

async fn add_packagejson_scripts(setup_type: &str, configs: Vec<&'static str>) -> CustomResult<()> {
    let path = "package.json";
    let mut package = Package::from_path(path)?;

    let prettier_scope = if setup_type.eq("ts") {
        "'./**/*.{js,ts,md,json}'"
    } else {
        "'./**/*.{js,ts,jsx,tsx,md,json,css,sass,scss}'"
    };
    let eslint_scope = if setup_type.eq("ts") {
        "'src/**/*.{js,ts}'"
    } else {
        "'src/**/*.{js,ts,jsx,tsx}'"
    };

    let scripts = HashMap::from([
        (
            "eslint",
            [
                build_script("lint", "eslint", eslint_scope),
                build_script("lint:fix", "eslint --fix", eslint_scope),
            ],
        ),
        (
            "prettier",
            [
                build_script("format", "prettier --write", prettier_scope),
                build_script("format:check", "prettier --check", prettier_scope),
            ],
        ),
    ]);

    for config in configs {
        let script_group = scripts.get(config).unwrap();
        for script_el in script_group {
            let mut script = script_el.to_owned();
            package.scripts.append(&mut script);
        }
    }

    // clean the end package.json product
    let serialized = serde_json::to_string(&package)?;

    let raw: HashMap<String, Value> = from_str(&serialized).unwrap();
    let mut clean: HashMap<String, Value> = HashMap::new();
    for (k, v) in raw.into_iter() {
        if v.to_string().ne("{}") && v.to_string().ne("[]") && v.to_string().ne("null") {
            clean.insert(k, v);
        }
    }

    // keep backup of current file
    copy_file_contents_to_another_file(
        &"./package.json".to_string(),
        &"./package-backup.json".to_string(),
    )
    .unwrap();

    let serialized = serde_json::to_string(&clean).unwrap();
    fs::write("./package.json", serialized).expect("Unable to write file");
    Ok(())
}

fn build_script(script_name: &str, script_pre: &str, scope: &str) -> BTreeMap<String, String> {
    BTreeMap::from([(
        script_name.to_string(),
        add_scope_to_script(script_pre, scope),
    )])
}

fn add_scope_to_script(script: &str, scope: &str) -> String {
    let mut res = String::new();
    res.push_str(&script.to_string());
    res.push_str(" ");
    res.push_str(&scope.to_string());
    res
}

// .lintstagedrc
async fn build_lintstagedrc(setup_type: &str, configs: Vec<&'static str>) -> CustomResult<()> {
    // Adjust script based on package manager
    let script_names = HashMap::from(if get_package_manager().eq("yarn") {
        [
            ("eslint", "yarn lint:fix"),
            ("prettier", "yarn format"),
            ("jest", "yarn test"),
        ]
    } else {
        [
            ("eslint", "npm run lint:fix"),
            ("prettier", "npm run format"),
            ("jest", "npm run test"),
        ]
    });

    let mut script_name_lines = String::new();
    for config in configs {
        let script_name = script_names.get(config).unwrap();
        script_name_lines.push_str("\n\t\t");
        script_name_lines.push_str(script_name);
    }
    script_name_lines.push_str("\n");

    let mut res = String::new();
    let lines = read_lines(&path_to_lintstagedrc(setup_type)).unwrap();
    for (i, line) in lines.enumerate() {
        if let Ok(ip) = line {
            res.push_str(&ip.to_string());
            if i == 1 {
                res.push_str(&script_name_lines);
            } else {
                res.push_str("\n");
            }
        }
    }

    let mut file = File::create(".lintstagedrc")?;
    file.write_all(res.as_ref())?;
    Ok(())
}

fn get_existing_configs() -> Vec<&'static str> {
    let mut existing_configs = vec![];
    let configs = HashMap::from([
        (".eslintrc.json", "eslint"),
        (".prettierrc", "prettier"),
        ("jest.config.json", "jest"),
    ]);
    for config in configs {
        if Path::new(config.0).exists() {
            existing_configs.push(config.1);
        }
    }
    existing_configs
}

fn path_to_lintstagedrc(setup_type: &str) -> String {
    let mut path = String::from("./templates/husky-");
    path.push_str(setup_type);
    path.push_str("/.lintstagedrc");
    return path;
}

fn get_deps(setup_type: &str) -> CustomResult<Lines<BufReader<File>>> {
    let mut path = String::from("./templates/husky-");
    path.push_str(setup_type);
    path.push_str("/dependencies");

    read_lines(&path)
}
