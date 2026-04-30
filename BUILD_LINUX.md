# Linux Installer Build Guide

This document explains how to build direct installer files for Linux.

## Prerequisites

Before building, ensure you have the following installed:

### Ubuntu/Debian
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libgtk-3-dev \
    libjavascriptcoregtk-4.1-dev \
    libsoup-3.0-dev \
    libdbus-1-dev \
    libappindicator3-dev \
    rpm  # For RPM builds
    fakeroot  # For DEB builds
```

### Fedora/RHEL
```bash
sudo dnf install -y webkit2gtk4.1-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libxdo-devel \
    gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    libdbus-1-devel \
    rpm-build
```

### Rust and Node.js
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Node.js (v18+)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# Or use nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
```

## Building Linux Installers

### Quick Build (Recommended)
```bash
npm install
npm run install:linux
```

This will create both `.deb` (Debian/Ubuntu) and `.AppImage` (universal Linux) packages.

### Build All Linux Formats
```bash
npm run tauri:build:linux
```

### Build Specific Format
```bash
# Only DEB package (for Debian/Ubuntu)
cd src-tauri
cargo tauri build --target x86_64-unknown-linux-gnu --bundles deb

# Only AppImage (universal)
cargo tauri build --target x86_64-unknown-linux-gnu --bundles appimage

# Only RPM package (for Fedora/RHEL/openSUSE)
cargo tauri build --target x86_64-unknown-linux-gnu --bundles rpm
```

## Output Files

After a successful build, installer files will be located in:
```
src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/
├── deb/
│   └── cleanslate-qa_2.0.0_amd64.deb
├── appimage/
│   └── CleanSlate_QA_2.0.0_x86_64.AppImage
└── rpm/
    └── cleanslate-qa-2.0.0-1.x86_64.rpm
```

## Installation

### Debian/Ubuntu (.deb)
```bash
sudo dpkg -i src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/deb/cleanslate-qa_2.0.0_amd64.deb
# OR
sudo apt install ./src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/deb/cleanslate-qa_2.0.0_amd64.deb
```

### Universal Linux (.AppImage)
```bash
chmod +x src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/appimage/CleanSlate_QA_2.0.0_x86_64.AppImage
./src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/appimage/CleanSlate_QA_2.0.0_x86_64.AppImage
```

### Fedora/RHEL/openSUSE (.rpm)
```bash
sudo rpm -ivh src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/rpm/cleanslate-qa-2.0.0-1.x86_64.rpm
# OR
sudo dnf install src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/rpm/cleanslate-qa-2.0.0-1.x86_64.rpm
```

## Troubleshooting

### Missing Dependencies
If you encounter errors about missing libraries, install the development packages listed in the Prerequisites section.

### Build Fails with Rust Errors
```bash
rustup update
cargo clean
npm run tauri:build:linux
```

### Icon Issues
Ensure all icon files exist in `src-tauri/icons/`:
- 32x32.png
- 128x128.png
- 128x128@2x.png
- icon.icns
- icon.ico

## Cross-Compilation

To build for different architectures:

### ARM64 (aarch64)
```bash
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu
cargo tauri build --target aarch64-unknown-linux-gnu
```

## Distribution-Specific Notes

### Ubuntu
- Tested on Ubuntu 20.04, 22.04, 24.04
- Use `.deb` package for best integration

### Debian
- Tested on Debian 11, 12
- Use `.deb` package

### Fedora
- Tested on Fedora 38, 39, 40
- Use `.rpm` package or `.AppImage`

### Arch Linux
- Use `.AppImage` or build from AUR
- May need to install `webkit2gtk-5.0` instead of `webkit2gtk-4.1`

### openSUSE
- Use `.rpm` package or `.AppImage`

## Support

For issues, please check:
- [Tauri Documentation](https://tauri.app/v1/guides/getting-started/prerequisites)
- [Tauri GitHub Issues](https://github.com/tauri-apps/tauri/issues)
