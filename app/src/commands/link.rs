use std::{
    env::{current_dir, home_dir},
    fs::read_dir,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use datastruct::queue::Queue;
use glob::Pattern;

use crate::{
    config::dot::DotConfig,
    error::{DotyError, LinkError, SharedError},
};

pub fn link(package: &PathBuf) -> Result<(), DotyError> {
    let home_dir = home_dir().ok_or(SharedError::HomeDirError)?;
    let mut current_dir = current_dir().map_err(|_| SharedError::PathError)?;
    let mut queue: Queue<PathBuf> = Queue::new(None);
    let config = DotConfig::load_config(package)?;
    current_dir.push(package);

    let items =
        read_dir(&current_dir).map_err(|e| SharedError::ReadError(current_dir.clone(), e))?;
    for item in items {
        queue.append(
            item.map_err(|e| SharedError::ReadError(current_dir.clone(), e))?
                .path(),
        );
    }
    while let Some(item) = queue.take() {
        if item.is_dir() {
            dir_handle(&home_dir, &current_dir, &mut queue, item, &config)?;
        } else if item.is_file() {
            file_handle(&home_dir, &current_dir, item, &config)?;
        }
    }

    Ok(())
}

fn dir_handle(
    home_dir: &Path,
    current_dir: &Path,
    queue: &mut Queue<PathBuf>,
    item: PathBuf,
    config: &DotConfig,
) -> Result<(), DotyError> {
    let relative_path = item
        .strip_prefix(current_dir)
        .or(Err(SharedError::PathError))?;
    let home_path = Path::join(home_dir, relative_path);

    if !home_path.exists() {
        // Checks if the file should be excluded
        // TODO: Make it able to exclude files inside a sub directory. This can be done by creating
        // the dir insted of symlinking it and symlinking the files directory
        for exclude in &config.exclude {
            let pattern = Pattern::new(exclude.trim_end_matches('/'))
                .map_err(|_| DotyError::ConfigError("Incorrect pattern".to_string()))?;
            if pattern.matches_path(relative_path) {
                return Ok(());
            }
        }

        symlink(&item, home_path).map_err(|e| LinkError::SymlinkError(item, e))?;
    } else if home_path.is_symlink() {
    } else {
        for diritem in read_dir(&item).map_err(|e| SharedError::ReadError(item.clone(), e))? {
            queue.append(
                diritem
                    .map_err(|e| SharedError::ReadError(item.clone(), e))?
                    .path(),
            );
        }
    }
    Ok(())
}

fn file_handle(
    home_dir: &Path,
    current_dir: &PathBuf,
    item: PathBuf,
    config: &DotConfig,
) -> Result<(), DotyError> {
    let relative_path = item
        .strip_prefix(current_dir)
        .or(Err(SharedError::PathError))?;
    let home_path = Path::join(home_dir, relative_path);

    if !home_path.exists() {
        // Checks if the file should be excluded
        for exclude in &config.exclude {
            let pattern = Pattern::new(exclude)
                .map_err(|_| DotyError::ConfigError("Incorrect pattern".to_string()))?;
            if pattern.matches_path(relative_path) {
                return Ok(());
            }
        }

        symlink(&item, home_path).map_err(|e| LinkError::SymlinkError(item, e))?;
        return Ok(());
    } else if home_path.is_symlink() {
        return Ok(());
    } else {
        for parent in home_dir.ancestors() {
            if parent.is_symlink() {
                return Ok(());
            }
        }
    }
    Err(DotyError::LinkError(LinkError::FileExits(home_path)))
}
