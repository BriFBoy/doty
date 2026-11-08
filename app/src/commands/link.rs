use std::{
    env::{current_dir, home_dir},
    fs::read_dir,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use datastruct::queue::Queue;

use crate::error::{LinkError, SharedError};

pub fn symlink_dir(path: &PathBuf) -> Result<(), LinkError> {
    let home_dir = home_dir().ok_or(SharedError::HomeDirError)?;
    let mut current_dir = current_dir()?;
    let mut queue: Queue<PathBuf> = Queue::new(None);
    current_dir.push(path);

    let items =
        read_dir(&current_dir).map_err(|e| SharedError::ReadError(current_dir.clone(), e))?;
    for item in items {
        queue.append(item?.path());
    }
    while let Some(item) = queue.take() {
        if item.is_dir() {
            dir_handle(&home_dir, &current_dir, &mut queue, item)?;
        } else if item.is_file() {
            file_handle(&home_dir, &current_dir, item)?;
        }
    }

    Ok(())
}

fn dir_handle(
    home_dir: &Path,
    current_dir: &Path,
    queue: &mut Queue<PathBuf>,
    item: PathBuf,
) -> Result<(), LinkError> {
    let relative_path = item
        .strip_prefix(current_dir)
        .or(Err(SharedError::PathError))?;
    let home_path = Path::join(home_dir, relative_path);

    if !home_path.exists() {
        symlink(&item, home_path).map_err(|e| LinkError::SymlinkError(item, e))?;
    } else if home_path.is_symlink() {
    } else {
        for item in read_dir(&item).map_err(|e| SharedError::ReadError(item, e))? {
            queue.append(item?.path());
        }
    }
    Ok(())
}

fn file_handle(home_dir: &Path, current_dir: &PathBuf, item: PathBuf) -> Result<(), LinkError> {
    let relative_path = item
        .strip_prefix(current_dir)
        .or(Err(SharedError::PathError))?;
    let home_path = Path::join(home_dir, relative_path);

    if !home_path.exists() {
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
    Err(LinkError::FileExits(home_path))
}
