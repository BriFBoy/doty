use std::{
    env::current_dir,
    os::unix::fs,
    path::{Path, PathBuf},
    str::FromStr,
};

pub fn symlink_dir(path: &PathBuf) {
    let res = fs::symlink(dotfiles_dir(path).as_path(), location_path(path).as_path());
    if let Err(res) = res {
        println!("Failed to link file/dir");
        println!("{}", res.kind())
    } else {
        println!("Symlinked files")
    }
}

// TODO: recursively get the dir.
// get the first that doest exits
fn location_path(path: &PathBuf) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        PathBuf::from_str(&std::env::var("HOME").unwrap())
            .unwrap()
            .join(path)
    }
}
fn dotfiles_dir(path: &PathBuf) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        current_dir().unwrap().join(path)
    }
}

/// Checks recursively to find the first directory that doesnt exits
pub fn find_non_existing(_starting_path: &Path) {}
