use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;
use std::process::Command;

pub type CustomResult<T> = Result<T, Box<dyn Error>>;

pub fn copy_file_contents_to_another_file(
    in_path: &String,
    out_path: &String,
) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(in_path).expect("Unable to read file");
    let mut file = File::create(out_path)?;
    file.write_all(content.as_ref())?;
    Ok(())
}

pub fn read_lines(in_path: &String) -> CustomResult<Lines<BufReader<File>>> {
    let file = File::open(in_path)?;
    Ok(BufReader::new(file).lines())
}

pub fn install_deps(deps: Vec<String>) -> CustomResult<()> {
    // 1. infer yarn or npm. defaults to yarn
    let argv;
    if Path::new("package-lock.json").exists() {
        argv = vec!["npm", "install", "--dev"];
    } else {
        argv = vec!["yarn", "add", "-D"];
    }

    // 2. combine deps with package manager commands
    let mut command = argv
        .iter()
        .map(|c| String::from(*c))
        .collect::<Vec<String>>();
    command.append(&mut &deps);

    Command::new(&command[0])
        .args(&command[1..])
        .spawn()
        .expect("Unable to start subprocesses")
        .wait()
        .expect("Unable to wait for subprocesses");

    Ok(())
}
