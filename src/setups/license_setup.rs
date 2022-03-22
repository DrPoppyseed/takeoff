use std::fs;

use inquire::{Select, Text};

use crate::constants::MIT_LICENSE;
use crate::utility::CustomResult;

const LICENSE: &str = r##"MIT License



Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"##;

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

  let mut res = String::new();
  match name {
    Err(err) => eprintln!("Unable to retrieve your name. Error: {:?}", err),
    Ok(name) => {
      // 2. add retrieved name and add to new string with license info
      for (i, line) in LICENSE.lines().enumerate() {
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
