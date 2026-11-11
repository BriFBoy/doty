# doty

A dotfile manager and OS-aware installer written in Rust 🦀

[![Rust](https://github.com/BriFBoy/doty/actions/workflows/rust.yml/badge.svg)](https://github.com/BriFBoy/doty/actions/workflows/rust.yml)
[![GitHub stars](https://img.shields.io/github/stars/BriFBoy/doty?style=social)](https://github.com/BriFBoy/doty/stargazers)

doty is in early development; installation instructions will be added once a
release is available. Packages use the same file structure as GNU Stow.

## Usage

Run from your stow directory (the parent of your packages):

```bash
# Symlink a package into your home directory
doty link vim        # alias: doty l

# Remove the symlinks again
doty unlink vim      # alias: doty ul

# Run the install script for the current OS 
doty install vim     # alias: doty i

# Run the uninstall script for the current OS
doty uninstall vim   # alias: doty ui
```

- `link` mirrors the package into `$HOME`: existing symlinks are skipped,
existing directories are descended into, and a conflicting real file is an error.
- The package argument and the script paths in `dot.toml` are resolved
relative to your current working directory.

## Configuration

Each package can contain a `dot.toml` mapping OS IDs to install/uninstall scripts:

```toml
# vim/dot.toml
[target.fedora]
install = "./vim/install.sh"
uninstall = "./vim/uninstall.sh"

[target.arch]
install = "./vim/install-arch.sh"
uninstall = "./vim/uninstall-arch.sh"
```

The OS ID is read from the `ID` field of `/etc/os-release`,
so any Linux distro works as a target. If there is no entry for your OS,
doty refuses to run a script rather than guessing.
