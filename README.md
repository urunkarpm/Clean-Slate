# CleanSlate QA v2.0

Cross-platform desktop utility that resets QA workstations to a clean baseline in one click.

## Features

- **Browser Cleanup**: Clears cache, cookies, and local storage for Chrome, Firefox, and Edge
- **System Cleanup**: Removes temp files, clears clipboard, and wipes shell history
- **Network Reset**: Flushes DNS cache and restores hosts file to default
- **Validation**: Verifies cleanup was successful with detailed reports
- **Secure Logging**: JSON logs stored with user-only permissions

## Tech Stack

- **Framework**: Tauri v2
- **Backend**: Rust 1.75+ (tokio, serde, walkdir, tracing)
- **Frontend**: React 18 + TypeScript + Vite + TailwindCSS
- **Plugins**: tauri-plugin-shell, tauri-plugin-dialog, tauri-plugin-log

## Project Structure

```
cleanslate-qa/
├── src-tauri/              # Rust backend
│   ├── capabilities/       # IPC permissions
│   ├── src/
│   │   ├── engine/         # Cleanup logic modules
│   │   ├── logger/         # Secure logging
│   │   ├── lib.rs          # IPC commands
│   │   └── main.rs         # Entry point
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── hooks/              # Custom React hooks
│   ├── types/              # TypeScript interfaces
│   └── App.tsx
├── package.json
└── README.md
```

## Getting Started

### Prerequisites

- Node.js 18+
- Rust 1.75+
- Platform-specific dependencies:
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio C++ Build Tools
  - **Linux**: `libwebkit2gtk`, `libgtk-3`, `libayatana-appindicator`

### Installation

```bash
# Install Node dependencies
npm install

# Install Rust dependencies
cd src-tauri
cargo build

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Usage

1. Launch CleanSlate QA
2. Click "Start Cleanup" to begin the reset process
3. Review the results and validation report
4. Check the log file for detailed operation records

## Configuration

Access settings via the gear icon to:
- Add excluded paths that should never be cleaned
- Toggle individual cleanup operations
- Configure clipboard and DNS handling

## Security

- Logs are stored with user-only permissions (0600 on Unix)
- No data is transmitted externally
- All operations are logged for audit purposes

## License

MIT