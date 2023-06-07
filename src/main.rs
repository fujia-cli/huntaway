use clap::{Parser, Subcommand};
use huntaway::{
  commands::{clear, doctor, init, publish},
  result::Result,
};
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
  Clear(clear::Cmd),
  Doctor(doctor::Cmd),
  Init(init::Cmd),
  Publish(publish::Cmd),
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
    Cmds::Clear(cmd) => cmd.run().await,
    Cmds::Doctor(cmd) => cmd.run().await,
    Cmds::Init(cmd) => cmd.run().await,
    Cmds::Publish(cmd) => cmd.run().await,
  }
}
