use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};
use doty::{
    commands::{install::install, link::symlink_dir, uninstall::uninstall, unlink::unlink},
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
    Link {
        path: PathBuf,
    },
    /// Removes symlink a folder
    Unlink {
        path: PathBuf,
    },
    Install {
        path: PathBuf,
    },
    Uninstall {
        path: PathBuf,
    },
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
            symlink_dir(&path)?;
            println!("Finished symlinking");
        }
        Command::Unlink { path } => {
            unlink(&path)?;
            println!("Finished Unlinking");
        }
        Command::Install { path } => install(&path)?,
        Command::Uninstall { path } => uninstall(&path)?,
    };
    Ok(())
}
