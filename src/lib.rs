use anyhow::Result;
use clap::Parser;

mod utils;

pub struct Cli {}

impl Cli {
    pub fn run() -> Result<bool, Box<dyn std::error::Error>> {
        let opts = Args::parse();

        match opts.subcmd {
            SubCmd::Run(args) => {
                let script = utils::get_alias(args.alias_name);
                Ok(utils::run_alias_script(&script)?)
            }
            SubCmd::Alias(args) => {
                let alias = utils::get_alias(args.alias_name);
                let script = args.script;

                Ok(utils::set_alias_script(&alias, script)?)
            }
            SubCmd::Delete(args) => {
                let delete_script = utils::get_alias(args.alias_name);
                Ok(utils::delete_alias_script(&delete_script)?)
            }
            SubCmd::Show(args) => {
                let is_show_all = args.all.unwrap_or_default();
                Ok(utils::show_alias(is_show_all)?)
            }
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcmd: SubCmd,
}

#[derive(Parser)]
pub enum SubCmd {
    /// Run any alias script or default
    Run(Run),
    /// Set alias to map script
    Alias(Alias),
    /// Delete alias
    Delete(Delete),
    /// Show alias
    Show(Show),
}

#[derive(Debug, Parser)]
pub struct Run {
    pub alias_name: Option<String>,
}

#[derive(Debug, Parser)]
pub struct Alias {
    pub alias_name: Option<String>,
    pub script: Option<String>,
}

#[derive(Debug, Parser)]
pub struct Delete {
    pub alias_name: Option<String>,
}

#[derive(Debug, Parser)]
pub struct Show {
    #[arg(
        short, 
        long, 
        num_args = 0..=1, 
        default_missing_value = "true", 
        value_parser = parser_show_params
    )]
    pub all: Option<bool>,
}

fn parser_show_params(is_show_all: &str) -> Result<bool> {
    Ok(!is_show_all.is_empty() && !is_show_all.eq("false"))
}
