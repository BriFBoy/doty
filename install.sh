#!/bin/sh
# Installs the latest doty release into ~/.local/bin.
# Usage: curl -fsSL https://raw.githubusercontent.com/BriFBoy/doty/master/install.sh | sh
set -eu

case "$(uname -m)" in
  x86_64 | amd64) arch=x86_64 ;;
  aarch64 | arm64) arch=aarch64 ;;
  *)
    echo "Unsupported architecture: $(uname -m)" >&2
    exit 1
    ;;
esac

url="https://github.com/BriFBoy/doty/releases/latest/download/doty-${arch}-linux-musl"
dest="$HOME/.local/bin/doty"

mkdir -p "$HOME/.local/bin"
if command -v curl >/dev/null 2>&1; then
  curl -fL --progress-bar -o "$dest" "$url"
else
  wget -O "$dest" "$url"
fi
chmod +x "$dest"
echo "Installed doty to $dest"

case ":$PATH:" in
  *":$HOME/.local/bin:"*) ;;
  *)
    echo "Note: \$HOME/.local/bin is not on your PATH. Add it, e.g.:"
    echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
    ;;
esac
