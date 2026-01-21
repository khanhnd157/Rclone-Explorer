# Rclone Explorer - Quick Start Guide

## Prerequisites

1. **Node.js** (v18+): https://nodejs.org

2. **Visual Studio C++ Build Tools** (Windows only):
   - Download: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
   - Chọn "Desktop development with C++"
   - Hoặc dùng winget:
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools --force --override "--passive --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
   ```
   - Restart terminal sau khi cài xong

3. **Rust** (latest stable): https://rustup.rs
   - Windows: Download rustup-init.exe và chạy
   - Restart terminal sau khi cài xong

4. **Tauri Prerequisites**: https://tauri.app/v2/guides/prerequisites

## Installation

```bash
# Clone hoặc vào thư mục project
cd Rclone Explorer

# Install dependencies
npm install
```

## Development

```bash
# Chạy app trong dev mode
npm run tauri dev
```

App sẽ mở với:
- Hot reload enabled
- DevTools available
- Mock rclone data

## Project Tour

### Main Window

```
┌─────────────────────────────────────────────┐
│ [R] Rclone Explorer  File Accounts Tasks Options  │ ← TopBar
├─────────────────────────────────────────────┤
│ [Tab 1] [Tab 2] [+]                         │ ← TabBar
├──────────────────────┬──────────────────────┤
│ Left Panel           │ Right Panel          │
│ ┌──────────────────┐ │ ┌──────────────────┐ │
│ │Remote: gdrive ▼  │ │ │Remote: local  ▼  │ │
│ ├──────────────────┤ │ ├──────────────────┤ │
│ │← → ↑ ⟳  /path    │ │ │← → ↑ ⟳  /path    │ │
│ ├──────────────────┤ │ ├──────────────────┤ │
│ │📁 Documents      │ │ │📁 Downloads      │ │ ← DualPanel
│ │📄 file.txt       │ │ │📄 image.png      │ │
│ │                  │ │ │                  │ │
│ └──────────────────┘ │ └──────────────────┘ │
├──────────────────────┴──────────────────────┤
│ local:/path  2 items  Selected: 1  Ready   │ ← StatusBar
└─────────────────────────────────────────────┘
```

### Keyboard Shortcuts

| Key         | Action                        |
|-------------|-------------------------------|
| `Ctrl+T`    | New Tab                       |
| `Ctrl+W`    | Close Tab                     |
| `Tab`       | Switch between left/right     |
| `F5`        | Copy selected → other panel   |
| `F6`        | Move selected → other panel   |
| `Delete`    | Delete selected files         |
| `F1`        | Show keyboard shortcuts       |

### Using the File Manager

#### Navigation
1. **Select Remote**: Click dropdown ở top của panel
2. **Browse Folders**: Double-click vào folder
3. **Go Back/Forward**: Click ← → buttons
4. **Go Up**: Click ↑ button
5. **Refresh**: Click ⟳ button

#### File Operations
1. **Select Files**: Click để select, Ctrl+Click cho multiple
2. **Copy Files**:
   - Select files trong left panel
   - Press `F5` để copy sang right panel
3. **Move Files**:
   - Select files
   - Press `F6` để move
4. **Delete Files**:
   - Select files
   - Press `Delete`

#### Multi-Tab Workflow
1. Press `Ctrl+T` để tạo tab mới
2. Mỗi tab có riêng left/right panel state
3. Switch tabs bằng click
4. Close tab với `Ctrl+W` hoặc click X

### Managing Accounts

1. Click **Accounts** menu
2. Click **Add Account**
3. Chọn provider (Google Drive, OneDrive, S3, etc.)
4. Nhập account name
5. Click **Continue** (sẽ trigger OAuth nếu cần)

### Monitoring Tasks

1. Click **Tasks** menu hoặc click vào running jobs indicator
2. Xem progress của copy/move operations
3. Refresh để update status

## Current Limitations (Mock Mode)

- File operations chỉ log ra console
- Không thực sự connect tới rclone
- Mock data: 3 remotes (gdrive, onedrive, local)
- No real OAuth flow

## Next: Real Rclone Integration

Để integrate rclone thật, xem `PROJECT_STRUCTURE.md` section "Next Steps".

## Troubleshooting

### "linker link.exe not found" (Windows)
```powershell
# Cài Visual Studio C++ Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools --force --override "--passive --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"

# Hoặc download manual:
# https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

# Sau đó restart terminal và verify:
cargo --version
```

### Build errors
```bash
# Clean và rebuild
rm -rf node_modules package-lock.json
npm install
npm run build
```

### Tauri errors
```bash
# Clear Rust cache
cd src-tauri
cargo clean
cd ..
npm run tauri dev
```

### Icons
Icons đã bị disable trong config để tránh lỗi build.

Để thêm icons:
1. Tạo icon files trong `src-tauri/icons/`
2. Update `tauri.conf.json`:
```json
"bundle": {
  "icon": [
    "icons/32x32.png",
    "icons/128x128.png", 
    "icons/icon.ico"
  ]
}
```

Tools tạo icons: https://tauri.app/v1/guides/features/icons/

## Building for Production

```bash
# Build app
npm run tauri build

# Output:
# Windows: src-tauri/target/release/bundle/
# macOS: src-tauri/target/release/bundle/
# Linux: src-tauri/target/release/bundle/
```

## Development Tips

### Hot Reload
- Frontend changes auto-reload
- Rust changes require restart

### Debug Mode
- Press `F12` để mở DevTools (dev mode)
- Check console cho errors/logs

### Testing Components
Sửa mock data trong `src-tauri/src/rclone.rs`:
```rust
pub async fn list_dir(...) -> Result<Vec<FileItem>, String> {
    Ok(vec![
        FileItem {
            name: "Your Folder".to_string(),
            // ...
        },
    ])
}
```

## File Structure Quick Reference

```
src/
├── lib/
│   ├── api/rcClient.ts       ← API calls
│   ├── components/           ← UI components
│   ├── stores/               ← State management
│   └── types/                ← TypeScript types
└── routes/
    └── +page.svelte          ← Main page

src-tauri/src/
├── main.rs                   ← Entry point
└── rclone.rs                 ← Backend logic (mock)
```

## Support

- Check `PROJECT_STRUCTURE.md` cho detailed docs
- Xem `README.md` cho overview
- Tauri docs: https://tauri.app
- SvelteKit docs: https://kit.svelte.dev
