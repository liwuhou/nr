use clap::Parser;
use anyhow::Result;

mod utils;

pub struct Cli {}

impl Cli {
  pub fn run() -> Result<()> {
    let opts = Args::parse();

    match opts.subcmd {
      SubCmd::Run(args) => {
        let script = utils::get_script(args.script);
        Ok(utils::run_script(&script)?)
      }
      SubCmd::Alias(args) => {
        Ok(())

      }
      SubCmd::Delete(args) => {
        Ok(())

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
  // Set alias to map script
  Alias(Alias),
  // Delete alias
  Delete(Delete),
}

#[derive(Debug, Parser)]
pub struct Run {
  pub script: Option<String>
}

#[derive(Debug, Parser)]
pub struct Alias {
  pub alias_name: String,
  pub scripts: Vec<String>
}

#[derive(Debug, Parser)]
pub struct Delete {
  pub alias_name: String
}