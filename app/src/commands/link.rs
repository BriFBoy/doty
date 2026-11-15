use std::{
    env::{current_dir, home_dir},
    fs::{self, read_dir, remove_dir_all, remove_file, rename},
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use datastruct::queue::Queue;
use glob::Pattern;

use crate::{
    config::{dot::DotConfig, doty::DotyConfig},
    error::{DotyError, LinkError, SharedError},
};

pub fn link(package: &PathBuf) -> Result<(), DotyError> {
    let home_dir = home_dir().ok_or(SharedError::HomeDirError)?;
    let mut current_dir = current_dir().map_err(|_| SharedError::PathError)?;
    let mut queue: Queue<PathBuf> = Queue::new(None);
    let package_config = DotConfig::load_config(package)?;
    let doty_config = DotyConfig::load_from_cwd()?;
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
        let relative_path = item
            .strip_prefix(&current_dir)
            .or(Err(SharedError::PathError))?;
        if is_excluded(&package_config, relative_path)? {
            continue;
        }

        if item.is_dir() {
            dir_handle(&home_dir, &current_dir, &mut queue, item, &doty_config)?;
        } else if item.is_file() {
            file_handle(&home_dir, &current_dir, item, &doty_config)?;
        }
    }

    Ok(())
}

fn dir_handle(
    home_dir: &Path,
    current_dir: &Path,
    queue: &mut Queue<PathBuf>,
    item: PathBuf,
    doty_config: &DotyConfig,
) -> Result<(), DotyError> {
    let relative_path = item
        .strip_prefix(current_dir)
        .or(Err(SharedError::PathError))?;
    let home_path = Path::join(home_dir, relative_path);
    if home_path.is_symlink() {
        if fs::read_link(&home_path).map_err(LinkError::IoError)? != item {
            handle_conflict(item, doty_config, home_path)
        } else {
            Ok(())
        }
    } else if !home_path.exists() {
        symlink(&item, home_path).map_err(|e| LinkError::SymlinkError(item, e))?;
        Ok(())
    } else {
        if !home_path.is_dir() {
            return handle_conflict(item, doty_config, home_path);
        }

        for diritem in read_dir(&item).map_err(|e| SharedError::ReadError(item.clone(), e))? {
            queue.append(
                diritem
                    .map_err(|e| SharedError::ReadError(item.clone(), e))?
                    .path(),
            );
        }
        Ok(())
    }
}

fn file_handle(
    home_dir: &Path,
    current_dir: &PathBuf,
    item: PathBuf,
    doty_config: &DotyConfig,
) -> Result<(), DotyError> {
    let relative_path = item
        .strip_prefix(current_dir)
        .or(Err(SharedError::PathError))?;
    let home_path = Path::join(home_dir, relative_path);

    if home_path.is_symlink() {
        if fs::read_link(&home_path).map_err(LinkError::IoError)? != item {
            handle_conflict(item, doty_config, home_path)
        } else {
            Ok(())
        }
    } else if !home_path.exists() {
        symlink(&item, home_path).map_err(|e| LinkError::SymlinkError(item, e))?;
        Ok(())
    } else {
        handle_conflict(item, doty_config, home_path)
    }
}

fn handle_conflict(
    item: PathBuf,
    doty_config: &DotyConfig,
    home_path: PathBuf,
) -> Result<(), DotyError> {
    match doty_config.get_conflict() {
        "skip" => {
            println!(
                "Skiping file {}",
                home_path.file_name().ok_or(DotyError::ParsError)?.display()
            );
            Ok(())
        }
        "force" => {
            println!(
                "forcing {}",
                home_path.file_name().ok_or(DotyError::ParsError)?.display()
            );
            if home_path.is_file() {
                remove_file(&home_path).map_err(LinkError::IoError)?;
            } else {
                remove_dir_all(&home_path).map_err(LinkError::IoError)?;
            }
            symlink(&item, &home_path).map_err(|e| LinkError::SymlinkError(item, e))?;
            Ok(())
        }
        "backup" => {
            println!(
                "backing up {}",
                home_path.file_name().ok_or(DotyError::ParsError)?.display()
            );
            let mut backup_name = home_path.clone();
            let mut base = backup_name
                .file_name()
                .ok_or(SharedError::PathError)?
                .to_owned();
            base.push(".bak");
            backup_name.set_file_name(&base);
            let mut n = 0;
            while backup_name.exists() {
                let mut name = base.clone();
                name.push(format!(".{}", n));
                backup_name.set_file_name(&name);
                n += 1;
            }

            rename(&home_path, backup_name).map_err(LinkError::IoError)?;
            symlink(&item, &home_path).map_err(|e| LinkError::SymlinkError(item, e))?;
            Ok(())
        }
        _ => Err(DotyError::LinkError(LinkError::FileExits(home_path))),
    }
}

// TODO: Make it able to exclude files inside a sub directory. This can be done by creating
// the dir insted of symlinking it and symlinking the files directory
/// Checks if the file should be excluded
fn is_excluded(config: &DotConfig, relative_path: &Path) -> Result<bool, DotyError> {
    for exclude in &config.exclude {
        let pattern = Pattern::new(exclude.trim_end_matches('/'))
            .map_err(|_| DotyError::ConfigError("Incorrect pattern".to_string()))?;
        if pattern.matches_path(relative_path) {
            return Ok(true);
        }
    }
    Ok(false)
}
