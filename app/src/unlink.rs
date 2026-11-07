use std::{
    env::{current_dir, home_dir},
    fs::{read_dir, remove_dir_all, remove_file},
    path::{Path, PathBuf},
};

use datastruct::queue::Queue;

use crate::error::SymlinkError;

pub fn unlink(path: &PathBuf) -> Result<(), SymlinkError> {
    let home_dir = home_dir().ok_or(SymlinkError::FileExits)?;
    let mut current_dir = current_dir()?;
    let mut queue: Queue<PathBuf> = Queue::new(None);
    current_dir.push(path);

    let items = read_dir(&current_dir)?;
    for item in items {
        queue.append(item?.path());
    }
    while let Some(item) = queue.take() {
        if item.is_dir() {
            dir_handle(&home_dir, &current_dir, &item, &mut queue)?;
        } else if item.is_file() {
            file_handle(&home_dir, &current_dir, item)?;
        }
    }

    Ok(())
}

fn dir_handle(
    home_dir: &Path,
    current_dir: &PathBuf,
    item: &Path,
    queue: &mut Queue<PathBuf>,
) -> Result<(), SymlinkError> {
    let relative_path = item
        .strip_prefix(current_dir)
        .or(Err(SymlinkError::PathError))?;
    let home_path = Path::join(home_dir, relative_path);

    if home_path.is_symlink() {
        remove_dir_all(home_path)?;
    } else if home_path.exists() {
        for item in read_dir(item)? {
            queue.append(item?.path());
        }
    }
    Ok(())
}

fn file_handle(home_dir: &Path, current_dir: &PathBuf, item: PathBuf) -> Result<(), SymlinkError> {
    let relative_path = item
        .strip_prefix(current_dir)
        .or(Err(SymlinkError::PathError))?;
    let home_path = Path::join(home_dir, relative_path);

    if !home_path.exists() {
    } else if home_path.is_symlink() {
        remove_file(home_path)?;
    } else {
        for parent in home_path.ancestors() {
            if parent.is_symlink() {
                if !parent.starts_with(home_dir) || parent == home_dir {
                    return Ok(());
                }
                remove_dir_all(parent)?;
                break;
            }
        }
    }
    Ok(())
}
