extern crate dirs;
extern crate ini;

use anyhow::Result;
use colored::*;
use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use dirs::home_dir;
use ini::Ini;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub enum State {
    Success,
    Error,
    Info,
}

pub fn console<T: AsRef<str>>(state: State, content: T) {
    match state {
        State::Success => {
            println!(
                "{} {}",
                String::from(" SUCCESS ").black().on_green(),
                content.as_ref()
            );
        }
        State::Error => {
            println!(
                "{} {}",
                String::from(" ERROR ").white().on_red(),
                content.as_ref()
            );
        }
        State::Info => {
            println!(
                "{} {}",
                String::from(" INFO ").white().on_blue(),
                content.as_ref()
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub scripts: Option<HashMap<String, String>>,
}

pub fn get_script(script: Option<String>) -> String {
    script.map_or(String::from("__default"), |x| x)
}

pub fn execute(command: &str, args: &Vec<&str>) -> Result<bool> {
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

pub fn run_script(script: &str) -> Result<bool> {
    let mut config = get_alias_ini()?;
    let current_path = env::current_dir()?
        .into_os_string()
        .into_string()
        .expect("Fail to convert the path");

    let package_json = PathBuf::from(&current_path).join("package.json");

    if !package_json.exists() {
        console(State::Error, "Current directory is not a node project!");
        return Ok(false);
    }

    match config.with_section(Some(script)).get(&current_path) {
        Some(script) => Ok(execute("npm", &vec!["run", script])?),
        None => {
            let scripts_in_packagejson = get_scripts(&package_json)?;
            let selected_index = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick up the script that you want to run.")
                .default(0)
                .highlight_matches(false)
                .items(
                    &scripts_in_packagejson
                        .iter()
                        .map(|t| t.0.clone())
                        .collect::<Vec<String>>(),
                )
                .interact()?;

            let selected_script = &scripts_in_packagejson.get(selected_index).unwrap().1;
            config.with_section(Some(script)).set(
                &current_path,
                selected_script.clone().normal().clear().to_string(),
            );
            save_alias_ini(config)?;
            Ok(execute(
                "npm",
                &vec!["run", &selected_script.normal().clear()],
            )?)
        }
    }
}

fn get_scripts(packagejson_path: &PathBuf) -> Result<Vec<(String, String)>> {
    let file = File::open(packagejson_path)?;
    let rdr = BufReader::new(file);
    let json: PackageInfo = serde_json::from_reader(rdr)?;

    if let Some(scripts) = json.scripts {
        Ok(scripts
            .iter()
            .map(|(key, value)| (format!("{}: {}", key.green(), value), key.to_owned()))
            .collect())
    } else {
        Ok(vec![])
    }
}

fn get_config_path() -> PathBuf {
    home_dir()
        .unwrap_or(PathBuf::from("/"))
        .join(".config")
        .join("runner.ini")
}

fn get_alias_ini() -> Result<Ini> {
    let config_path = get_config_path();

    Ok(if !config_path.exists() {
        // create
        let mut config = Ini::new();
        config.with_section(None::<String>).set("encoding", "utf-8");
        config.write_to_file(&config_path)?;
        config
    } else {
        Ini::load_from_file(config_path)?
    })
}

fn save_alias_ini(config: Ini) -> Result<()> {
    Ok(config.write_to_file(get_config_path())?)
}
