use std::process::{ Command, Stdio };
use std::io::{ BufRead, BufReader };
use std::env;
use anyhow::Result;

pub fn get_script(script: Option<String>) -> String {
  script.map_or(String::from("__default"), |x| x)
}

pub fn execute(command: &str, args: &Vec<String>) -> Result<bool> {
  let runner = Command::new(command)
    .args(args)
    .stdout(Stdio::piped())
    .spawn()?;

  let stdout = runner.stdout.expect("Can't get command's output!");
  let reader = BufReader::new(stdout);

  for line in reader.lines() {
    if let Ok(line) = line {
      println!("{}", line);
    } else {
      eprintln!("fail to read");
    }
  }
  Ok(true)
}

pub fn run_script(script: &str) -> Result<()> {
  let current_path = env::current_dir()?.into_os_string().into_string().expect("Fail to convert the path");
  println!("{}", current_path);
  // search script and current path
  // if find it, just execute it
  // if can't find it, just show this node project's script
  //  user can choose the script, execute it and save this relationship
  Ok(())
}
