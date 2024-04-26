use anyhow::Result;

use crate::clap::app::App;
use crate::config::Config;

mod clap;
mod config;
mod error;

/// This is the entry point for the application.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (kind, matches) = App::subcommand()?;
    // Get the command line matches for the subcommand that was used.
    // Clap will provide the arguments in priority order (highest to lowest):
    // command line argument, environment variable, or default value.
    let hc = Config::new(kind, matches)?;

    dbg!(hc);

    // let app = App::parse();
    // let cp = app.global_opts.config_path;

    Ok(())
}
