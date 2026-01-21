# Rclone Explorer

Cross-platform desktop GUI for rclone with dual panel file manager interface.

## Tech Stack

- **Frontend**: SvelteKit + TypeScript
- **Desktop**: Tauri 2 (Rust)
- **UI**: Tailwind CSS, Dark theme
- **Backend**: rclone integration

## Features

- **Nav Tabs UI** - 5 main sections: Explorer, Accounts, Tasks, Options, Help
- **Dual Panel File Manager** - Multi-tab support, resize panels
- **Account Management** - Google Drive, OneDrive, S3, SFTP, etc.
- **Task Monitoring** - Job progress tracking
- **Settings Page** - Rclone config, appearance, auto-refresh
- **Keyboard Shortcuts** - F5 copy, F6 move, F1 help, Delete
- **Dark Theme UI** - Modern, professional interface

## Prerequisites

### Windows
1. **Visual Studio C++ Build Tools**
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools --force --override "--passive --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
   ```
   Hoặc download: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

2. **Rust**: https://rustup.rs

3. **Node.js** (v18+): https://nodejs.org

### macOS
1. Xcode Command Line Tools: `xcode-select --install`
2. Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Node.js: https://nodejs.org

### Linux
1. Build essentials: `sudo apt install build-essential`
2. Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Node.js: https://nodejs.org

## Development

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

## Keyboard Shortcuts

- `Ctrl+T`: New tab
- `Ctrl+W`: Close tab
- `Tab`: Switch between left/right panel
- `F5`: Copy selected files to other panel
- `F6`: Move selected files to other panel
- `F2`: Rename (not yet implemented)
- `Delete`: Delete selected files
