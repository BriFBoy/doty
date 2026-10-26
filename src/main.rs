use std::{
    io::{Write, stderr},
    path::PathBuf,
};

use clap::{
    Parser, Subcommand,
    error::ErrorKind::{self, MissingRequiredArgument},
};
use doty::symlink_dir;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
    #[arg()]
    path: Option<PathBuf>,
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

fn main() -> Result<(), ErrorKind> {
    let cli = Args::parse();
    match cli.command {
        Some(Command::Link { path }) => {
            symlink_dir(&path);
        }
        Some(Command::Unlink { path }) => {
            println!("Unlinking for: {}", path.into_string().unwrap());
        }
        Some(Command::Install { path }) => println!("Installing: {}", path.into_string().unwrap()),
        Some(Command::Uninstall { path }) => {
            println!("Uninstalling: {}", path.into_string().unwrap())
        }
        None => {
            if let Some(path_buf) = cli.path {
                println!(
                    "linking and installing: {}",
                    path_buf.into_string().unwrap()
                )
            } else {
                let error = stderr();
                {
                    let mut lock = error.lock();
                    lock.write_all("Needs a path to preform operation!\n".as_bytes())
                        .unwrap();
                }
                return Err(MissingRequiredArgument);
            }
        }
    }
    Ok(())
}
