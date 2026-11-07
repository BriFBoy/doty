use std::{
    env::{current_dir, home_dir},
    fs::read_dir,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use datastruct::queue::Queue;

use crate::error::SymlinkError;
pub mod error;

pub fn symlink_dir(path: &PathBuf) -> Result<(), SymlinkError> {
    let _home_dir = home_dir().ok_or(SymlinkError::FileExits)?;
    let mut current_dir = current_dir()?;
    let mut queue: Queue<PathBuf> = Queue::new(None);
    current_dir.push(path);

    let items = read_dir(&current_dir)?;
    for item in items {
        queue.append(item?.path());
    }
    while let Some(item) = queue.take() {
        if item.is_dir() {
            dir_handle(&_home_dir, &current_dir, &mut queue, &item)?;
        } else if item.is_file() {
            file_handle(&_home_dir, &current_dir, item)?;
        }
    }

    Ok(())
}

fn dir_handle(
    _home_dir: &PathBuf,
    current_dir: &PathBuf,
    queue: &mut Queue<PathBuf>,
    item: &PathBuf,
) -> Result<(), SymlinkError> {
    let relative_path = item
        .strip_prefix(current_dir)
        .or(Err(SymlinkError::PathError))?;
    let home_path = Path::join(_home_dir, relative_path);

    if !home_path.exists() {
        symlink(item, home_path)?;
    } else if home_path.is_symlink() {
    } else {
        for item in read_dir(item)? {
            queue.append(item?.path());
        }
    }
    Ok(())
}

fn file_handle(
    _home_dir: &PathBuf,
    current_dir: &PathBuf,
    item: PathBuf,
) -> Result<(), SymlinkError> {
    let relative_path = item
        .strip_prefix(current_dir)
        .or(Err(SymlinkError::PathError))?;
    let home_path = Path::join(_home_dir, relative_path);

    if !home_path.exists() {
        symlink(item, home_path)?;
        return Ok(());
    } else if home_path.is_symlink() {
        return Ok(());
    } else {
        for parent in home_path.ancestors() {
            if parent.is_symlink() {
                return Ok(());
            }
        }
    }
    Err(SymlinkError::FileExits)
}

/// Checks recursively to find the first directory that doesnt exits
pub fn find_non_existing(_starting_path: &Path) {}
