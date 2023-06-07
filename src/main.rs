use clap::{Parser, Subcommand};
use huntaway::{commands::doctor, result::Result};
use std::process;

#[derive(Debug, Parser)]
#[clap(name = "huntaway")]
#[clap(version)]
struct Cli {
  #[clap(subcommand)]
  cmd: Cmds,
}

#[derive(Debug, Subcommand)]
enum Cmds {
  Doctor(doctor::Cmd),
}

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  if let Err(e) = run(cli).await {
    eprint!("error: {e:?}");
    process::exit(1)
  }
}

async fn run(cli: Cli) -> Result {
  match cli.cmd {
    Cmds::Doctor(cmd) => cmd.run().await,
  }
}
