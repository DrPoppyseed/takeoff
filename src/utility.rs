use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub type CustomResult<T> = Result<T, Box<dyn Error>>;

pub fn copy_file_contents_to_another_file(in_path: &String, out_path: &String) -> CustomResult<()> {
  let content = fs::read_to_string(in_path).expect("Unable to read file");
  let mut file = File::create(out_path)?;
  file.write_all(content.as_ref())?;
  Ok(())
}

pub fn string_to_file(content: &str, out_path: &str) -> CustomResult<()> {
  fs::write(out_path, content).expect("Unable to write file");
  Ok(())
}

pub fn get_package_manager() -> &'static str {
  if Path::new("package-lock.json").exists() {
    "npm"
  } else {
    "yarn"
  }
}

pub fn install_deps(mut deps: Vec<&str>) -> CustomResult<()> {
  let mut argv;
  if get_package_manager().eq("npm") {
    argv = vec!["npm", "install", "--dev"];
  } else {
    argv = vec!["yarn", "add", "-D"];
  }
  argv.append(&mut deps);

  Command::new(&argv[0])
    .args(&argv[1..])
    .spawn()
    .expect("Unable to start subprocesses")
    .wait()
    .expect("Unable to wait for subprocesses");

  Ok(())
}
