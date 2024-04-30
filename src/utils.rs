extern crate dirs;
extern crate ini;

use anyhow::{anyhow, Result};
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

const DEFAULT_SCRIPT: &str = "-";

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

fn execute(command: &str, args: &Vec<&str>) -> Result<bool> {
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

fn get_config_ini() -> Result<Ini> {
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

fn pwd() -> Result<String> {
    Ok(env::current_dir()?.to_string_lossy().to_string())
}

fn get_project_info() -> Result<(String, PathBuf)> {
    let pwd = pwd()?;
    let pakcage_json = PathBuf::from(&pwd).join("package.json");

    if !pakcage_json.exists() {
        console(State::Error, "Current directory is not a node project!");
        return Err(anyhow!("error"));
    }

    Ok((pwd, pakcage_json))
}

fn get_selected_script(package_json: PathBuf) -> Result<String> {
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
    Ok(selected_script.to_owned())
}

pub fn get_alias(script: Option<String>) -> String {
    match script {
        None => DEFAULT_SCRIPT.to_owned(),
        Some(x) => x,
    }
}

pub fn run_alias_script(script: &str) -> Result<bool> {
    let (current_path, package_json) = get_project_info()?;
    let mut config = get_config_ini()?;

    match config.with_section(Some(script)).get(&current_path) {
        Some(script) => Ok(execute("npm", &vec!["run", script])?),
        None => {
            let selected_script = get_selected_script(package_json)?;
            config
                .with_section(Some(script))
                .set(&current_path, &selected_script);
            save_alias_ini(config)?;
            Ok(execute("npm", &vec!["run", &selected_script])?)
        }
    }
}

pub fn set_alias_script(alias: &str, script: Option<String>) -> Result<bool> {
    let (current_path, package_json) = get_project_info()?;
    let mut config = get_config_ini()?;

    let script = match script {
        Some(script) => script,
        None => get_selected_script(package_json)?,
    };

    config.with_section(Some(alias)).set(&current_path, &script);
    save_alias_ini(config)?;

    Ok(true)
}

pub fn delete_alias_script(alias: &str) -> Result<bool> {
    let mut config = get_config_ini()?;
    let current_path = pwd()?;

    if let None = config.with_section(Some(alias)).get(&current_path) {
        console(
            State::Error,
            format!(
                "Can't found alias \"{}\" in this project.",
                alias.bold().green()
            ),
        );
        Ok(false)
    } else {
        config.with_section(Some(alias)).delete(&current_path);
        save_alias_ini(config)?;
        console(
            State::Success,
            format!("Delete alias {} success.", alias.bold().green()),
        );
        Ok(true)
    }
}

pub fn show_alias(is_show_all: bool) -> Result<bool> {
    let current_path = pwd()?;
    let config = get_config_ini()?;
    let mut result: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for (sec, prop) in config {
        if let Some(sec) = sec {
            for (path, script) in prop.into_iter() {
                if is_show_all || path == current_path {
                    result
                        .entry(path)
                        .or_insert(vec![])
                        .push((sec.clone(), script));
                }
            }
        }
    }
    if result.is_empty() {
        if is_show_all {
            console(State::Info, "Not found any config.")
        } else {
            console(State::Info, "Not found any config in this project.")
        }
        return Ok(false);
    }

    for (sec, scripts) in result.iter() {
        println!("{}", format!(" {} ", sec).black().on_white());

        for (alias, script) in scripts {
            println!(" {} => {}", alias.green(), script);
        }
        println!();
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_alias_name() {
        assert_eq!(get_alias(None::<String>), DEFAULT_SCRIPT);
        assert_eq!(get_alias(Some("-".to_string())), DEFAULT_SCRIPT);
        assert_eq!(get_alias(Some("start".to_string())), "start".to_string());
    }

    #[test]
    fn find_user_script_ini() {}
}
