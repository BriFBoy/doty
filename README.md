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

## How it works

doty works on a stow directory, a folder that contains your dotfile
packages. Each package is a directory with the files you want in your
home directory. You run doty from the stow directory.

`link` mirrors a package into your home directory with symlinks. `install`
runs its OS-specific script. They are independent, so you can use either
alone or both.

Symlinks use absolute paths into the stow directory, so moving it breaks
the links.

## Usage

| Command | Alias | What it does |
| --- | --- | --- |
| `doty link <package>` | `doty l` | Symlinks the package into your home directory |
| `doty unlink <package>` | `doty ul` | Removes the symlinks doty created for the package |
| `doty install <package>` | `doty i` | Runs the install script for your OS |
| `doty uninstall <package>` | `doty ui` | Runs the uninstall script for your OS |
| `doty bundle list` | `doty b l` | Lists the bundles in your doty.toml |
| `doty bundle install <name>` | `doty b i` | Runs the install scripts of all packages in the bundle |
| `doty bundle uninstall <name>` | `doty b ui` | Runs the uninstall scripts of all packages in the bundle |

Package names are relative to the directory you run doty from. There is no
bulk link, so you run `doty link` once per package.

## Package layout

A stow directory typically looks like this:

```
dotfiles/
├── doty.toml
├── base/
│   ├── dot.toml
│   └── .bashrc
└── vim/
    ├── dot.toml
    ├── install.sh
    └── .vimrc
```

Everything in a package is mirrored into your home directory as-is,
including `dot.toml` and your install scripts.

A package without a `dot.toml` can still be linked. It cannot be installed,
because there is no script to run.

## Configuration

### dot.toml

Each package can contain a `dot.toml` that maps OS IDs to a script:

```toml
# vim/dot.toml
exclude = ["local/"]
depends_on = ["base"]

[fedora]
script = "install.sh"

[arch]
script = "install-arch.sh"
```

The table name is the OS ID from the `ID` field of `/etc/os-release`, so
any Linux distro works. Without an entry for your OS, doty refuses to run
a script rather than guessing.

Common IDs:

- `arch`
- `debian`
- `fedora`
- `gentoo`
- `nixos`
- `opensuse-leap`
- `ubuntu`
- `void`

`script` is a path relative to the package directory. doty runs it as
`<script> install` or `<script> uninstall` and sets the executable bit if
it is missing. The working directory is the stow directory, not the
package.

`depends_on` lists packages that are installed first. They are only
installed, never uninstalled, and circular dependencies are not supported.

`exclude` is a list of glob patterns that keep files out of the link. A
typical package excludes its own `dot.toml` and install scripts:

```toml
exclude = ["dot.toml", "install.sh"]
```

See [Glob patterns](#glob-patterns-exclude) for how the patterns work.

### doty.toml

Optionally, a `doty.toml` in the stow directory configures the project:

```toml
[packages]
fallback = "full"
conflict = "backup"

[bundles]
dev = ["vim", "zsh"]
```

The `[packages]` table must exist, even when it is empty. doty does not
parse a `doty.toml` without it.

`fallback` controls how far doty looks in `ID_LIKE` when there is no entry
for your OS. `once` (the default) tries only the first candidate, `full`
tries all of them.

`conflict` decides what `link` does when a real file or directory is in the
way. `error` (the default) stops, `skip` leaves it, `force` replaces it,
and `backup` moves it aside as `name.bak`, or `name.bak.0`, `name.bak.1`
and so on.

`Backup` will backup files from a different package if they overlap.
This should not be a problem in normal use.

### Bundles

Bundles are named groups of packages in the `[bundles]` table. `doty
bundle install dev` runs the install script of every package in the
bundle, `doty bundle uninstall dev` the uninstall script.

Bundles only run scripts. They never create or remove symlinks, so the
packages in a bundle still need their own `doty link`.

## Writing scripts

The script receives `install` or `uninstall` as its first argument. It
should only touch things belonging to its own package, and it should be
idempotent so it is safe to run several times.

```bash
#!/bin/sh
install() {
  mkdir -p "$HOME/.config/myapp"
}

uninstall() {
  rm -rf "$HOME/.config/myapp"
}

if [ "$1" = "install" ]; then
  install
elif [ "$1" = "uninstall" ]; then
  uninstall
fi
```

## Glob patterns (exclude)

`exclude` only applies to files and directories at the top level of the
package. A pattern is matched against the name of each top-level entry.
`*` matches any name, `?` matches a single character, and `**` matches
everything.

| Pattern | Matches |
| --- | --- |
| `local` or `local/` | the top-level `local` directory |
| `*.txt` | top-level `.txt` files |
| `dot.toml` | the top-level `dot.toml` |
