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
- The package argument is resolved relative to your current working
  directory; the `script` path in `dot.toml` is resolved relative to the
  package directory.

## Configuration

Each package can contain a `dot.toml` mapping OS IDs to a script:

```toml
# vim/dot.toml
[fedora]
script = "install.sh"

[arch]
script = "install-arch.sh"
```

The table name is the OS ID, read from the `ID` field of `/etc/os-release`,
so any Linux distro works as a target. If there is no entry for your OS,
doty refuses to run a script rather than guessing.

`script` is the path to a single script, relative to the package directory.
doty runs it as `<script> install` or `<script> uninstall` and sets the
executable bit if it is missing.
