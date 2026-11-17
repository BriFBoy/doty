# doty

A dotfile manager and OS-aware installer written in Rust 🦀

[![Rust](https://github.com/BriFBoy/doty/actions/workflows/rust.yml/badge.svg)](https://github.com/BriFBoy/doty/actions/workflows/rust.yml)
[![GitHub stars](https://img.shields.io/github/stars/BriFBoy/doty?style=social)](https://github.com/BriFBoy/doty/stargazers)

doty is in early development and might contain some bugs. Please feel free to
open an issue if you find one.

## Installation

The install script is for Linux x86_64 and aarch64. It installs to `~/.local/bin/doty`.
Use this command for a one-line install:

```bash
curl -fsSL https://raw.githubusercontent.com/BriFBoy/doty/master/install.sh | sh
```

If you get `command not found` afterwards, ~/.local/bin might be missing
from your PATH. Add this to your `~/.bashrc`:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

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

# List all bundles
doty bundle list   # alias: doty b l 

# Install bundle
doty bundle install dev   # alias: doty b i

# Uninstall bundle
doty bundle uninstall dev   # alias: doty b ui 
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
exclude = ["local/"]
depends_on = ["base"]

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
executable bit if it is missing. `exclude` is a list of glob patterns.
matching paths are left untouched by `link`, `depends_on` lists packages
that are installed first when running `install`.

Optionally, a `doty.toml` in the directory you run doty from configures
the project:

```toml
[packages]
fallback = "full"
conflict = "backup"

[bundles]
dev = ["vim", "zsh"]
```

`fallback` controls how far doty looks in the `ID_LIKE` field when there
is no entry for your OS, `once` (the default) tries only the first
candidate, `full` tries all of them in order. `conflict` decides what
`link` does when a real file is in the way. `error` (the default) stops,
`skip` leaves the file, `force` replaces it, and `backup` moves it aside
as `name.bak`. `bundles` defines named groups of packages for
`doty bundle install` and `doty bundle uninstall`.
