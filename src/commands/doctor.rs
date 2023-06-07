use crate::{commands::*, result::Result};
use bollard::Docker;
use clap::Args;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

// execute the `hsi doctor` command to
// check the prerequisites of cli
#[derive(Debug, Args)]
pub struct Cmd {}

impl Cmd {
  pub async fn run(&self) -> Result {
    let spinner_style = ProgressStyle::with_template("{spinner} {wide_msg}")
      .unwrap()
      .tick_strings(&["🔸 ", "🔶 ", "🟠 ", "🟠 ", "🔶 "]);

    println!(
      "{} Checking the prerequisites of cli...",
      ui::emoji::LOOKING_GLASS
    );

    let pb = ProgressBar::new(0);
    pb.set_style(spinner_style.clone());

    for _ in 0..10 {
      pb.set_message("Checking the version of Docker...");
      pb.inc(1);
      thread::sleep(Duration::from_millis(200));
    }

    let new_spinner_style = ProgressStyle::with_template("{wide_msg}").unwrap();
    pb.set_style(new_spinner_style);

    let _docker = Docker::connect_with_local_defaults().unwrap();

    match _docker.version().await {
      Ok(version) => pb.finish_with_message(format!(
        "{} {}: {}\n{} {}",
        ui::emoji::SUCCESS,
        String::from("Docker version"),
        version.version.unwrap(),
        ui::emoji::SPARKLE,
        style("Success! The docker requirement for deploying service has been satisfied.").green(),
      )),
      Err(_) => pb.finish_with_message(format!(
        "{} {}\n{} {}",
        ui::emoji::FAIL,
        String::from("No running docker found."),
        ui::emoji::WARN,
        style("Please check the status of docket.").red(),
      )),
    }

    println!();
    Ok(())
  }
}
