use std::path::PathBuf;

use clap::{Parser, Subcommand};
use doty::{error::DotyError, symlink_dir};

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

fn main() -> Result<(), DotyError> {
    let cli = Args::parse();
    match cli.command {
        Command::Link { path } => {
            symlink_dir(&path)?;
            println!("Finished symlinking");
        }
        Command::Unlink { path } => {
            println!("Unlinking for: {}", path.into_string().unwrap());
        }
        Command::Install { path } => println!("Installing: {}", path.into_string().unwrap()),
        Command::Uninstall { path } => {
            println!("Uninstalling: {}", path.into_string().unwrap());
        }
    }
    Ok(())
}
