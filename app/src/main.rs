use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};
use doty::{
    commands::{
        bundle::{install_or_remove_bundles, list_bundles},
        install_or_remove,
        link::link,
        unlink::unlink,
    },
    config::fs_interact::RunMode::{Install, Uninstall},
    error::DotyError,
};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Symlinks a folder
    #[command(visible_alias = "l")]
    Link { path: PathBuf },
    /// Removes symlink a folder
    #[command(visible_alias = "ul")]
    Unlink { path: PathBuf },
    #[command(visible_alias = "i")]
    Install { path: PathBuf },
    #[command(visible_alias = "ui")]
    Uninstall { path: PathBuf },

    #[command(subcommand)]
    Bundle(Bundle),
}

#[derive(Subcommand)]
enum Bundle {
    List,
    Install { name: String },
    Uninstall { name: String },
}

fn main() -> ExitCode {
    let cli = Args::parse();

    if let Err(e) = run(cli) {
        eprintln!("{}", e);
        ExitCode::from(1)
    } else {
        ExitCode::from(0)
    }
}

fn run(cli: Args) -> Result<(), DotyError> {
    match cli.command {
        Command::Link { path } => {
            link(&path)?;
            println!("Finished symlinking");
        }
        Command::Unlink { path } => {
            unlink(&path)?;
            println!("Finished Unlinking");
        }
        Command::Install { path } => install_or_remove(&path, &Install)?,
        Command::Uninstall { path } => install_or_remove(&path, &Uninstall)?,
        Command::Bundle(bundle) => match bundle {
            Bundle::List => list_bundles()?,
            Bundle::Install { name } => install_or_remove_bundles(&name, &Install)?,
            Bundle::Uninstall { name } => install_or_remove_bundles(&name, &Uninstall)?,
        },
    };
    Ok(())
}
