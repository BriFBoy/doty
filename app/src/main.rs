use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};
use doty::{
    error::{DotyError, SharedError},
    link::symlink_dir,
    unlink::unlink,
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
            println!(
                "Unlinking for: {}",
                path.into_string().or(Err(SharedError::PathError))?
            );
        }
        Command::Install { path } => println!(
            "Installing: {}",
            path.into_string().or(Err(SharedError::PathError))?
        ),
        Command::Uninstall { path } => {
            println!(
                "Uninstalling: {}",
                path.into_string().or(Err(SharedError::PathError))?
            );
        }
    };
    Ok(())
}
