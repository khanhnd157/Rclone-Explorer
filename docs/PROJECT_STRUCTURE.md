# Rclone Explorer - Project Structure

## Overview
Rclone Explorer là ứng dụng desktop GUI cho rclone với dual panel file manager interface.

## Technology Stack
- **Frontend**: SvelteKit 2 + TypeScript
- **Desktop Shell**: Tauri 2 (Rust)
- **Styling**: Tailwind CSS
- **Build Tool**: Vite

## Directory Structure

```
Rclone Explorer/
├── src/                          # Frontend source code
│   ├── lib/
│   │   ├── api/
│   │   │   └── rcClient.ts      # Tauri API wrapper
│   │   ├── components/          # Svelte components
│   │   │   ├── TopBar.svelte
│   │   │   ├── TabBar.svelte
│   │   │   ├── DualPanel.svelte
│   │   │   ├── FilePanel.svelte
│   │   │   ├── FileList.svelte
│   │   │   ├── StatusBar.svelte
│   │   │   ├── AccountsModal.svelte
│   │   │   ├── RegisterAccountModal.svelte
│   │   │   ├── TasksModal.svelte
│   │   │   └── KeyboardHelper.svelte
│   │   ├── stores/              # Svelte stores
│   │   │   ├── tabs.ts          # Tab management
│   │   │   ├── accounts.ts      # Account/remote management
│   │   │   ├── jobs.ts          # Task/job management
│   │   │   ├── keyboard.ts      # Keyboard shortcuts
│   │   │   └── index.ts         # Store exports
│   │   └── types/               # TypeScript types
│   │       └── index.ts         # Type definitions
│   ├── routes/                  # SvelteKit routes
│   │   ├── +layout.svelte       # Root layout
│   │   ├── +layout.ts           # Layout config
│   │   └── +page.svelte         # Main page
│   ├── app.css                  # Global styles
│   └── app.html                 # HTML template
├── src-tauri/                   # Tauri backend (Rust)
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   ├── lib.rs               # Library exports
│   │   └── rclone.rs            # Rclone integration
│   ├── icons/                   # App icons
│   ├── Cargo.toml               # Rust dependencies
│   ├── build.rs                 # Build script
│   └── tauri.conf.json          # Tauri configuration
├── static/                      # Static assets
├── build/                       # Build output (generated)
├── package.json                 # Node dependencies
├── vite.config.ts               # Vite configuration
├── svelte.config.js             # Svelte configuration
├── tailwind.config.js           # Tailwind configuration
└── tsconfig.json                # TypeScript configuration
```

## Key Components

### Frontend Components

#### Layout Components
- **TopBar**: Menu bar với File, Accounts, Tasks, Options, Help
- **TabBar**: Multi-tab management với add/close/switch
- **DualPanel**: Container cho left/right panels với resizable divider
- **StatusBar**: Hiển thị thông tin panel hiện tại và running jobs

#### File Manager Components
- **FilePanel**: Single panel với remote selector, toolbar, breadcrumb
- **FileList**: Table view của files/folders với sort, filter, selection

#### Modal Components
- **AccountsModal**: Quản lý accounts/remotes
- **RegisterAccountModal**: Thêm mới account với provider selection
- **TasksModal**: Hiển thị running/completed jobs
- **KeyboardHelper**: Help modal cho keyboard shortcuts

### Stores (State Management)

#### tabs.ts
- Quản lý multi-tab state
- Mỗi tab có left/right panel state
- History navigation (back/forward)

#### accounts.ts
- Danh sách remotes/accounts
- Modal visibility state

#### jobs.ts
- Running/completed jobs
- Task modal visibility

#### keyboard.ts
- Global keyboard shortcuts
- F5 (copy), F6 (move), Delete, Tab switch

### API Layer (rcClient.ts)
TypeScript wrapper cho Tauri commands:
- `listRemotes()`: Lấy danh sách remotes
- `listDir()`: Lấy files trong thư mục
- `copyItems()`, `moveItems()`, `deleteItems()`: File operations
- `getJobs()`, `getJob()`: Job monitoring
- `configCreate()`, `configDelete()`, `configReconnect()`: Account management

### Backend (Rust - src-tauri/)

#### main.rs
Entry point, register Tauri commands

#### rclone.rs
Rclone integration với mock data:
- Commands: list_remotes, list_dir, copy_items, move_items, delete_items
- Commands: get_jobs, get_job
- Commands: config_list, config_create, config_delete, config_reconnect

**Note**: Hiện tại sử dụng mock data. Để tích hợp rclone thật:
1. Thêm process spawning cho `rclone rcd`
2. HTTP client để gọi rclone RC API
3. Job tracking và progress monitoring

## Features Implemented

### ✅ Core Features
- [x] Dual panel file manager layout
- [x] Multi-tab support
- [x] Resizable panel divider
- [x] Remote/account selector
- [x] File list với sort/filter
- [x] Breadcrumb navigation
- [x] History (back/forward)

### ✅ Account Management
- [x] View accounts list
- [x] Add account với provider grid
- [x] Delete account
- [x] Reconnect account

### ✅ Task Management
- [x] Tasks modal với job list
- [x] Job status display
- [x] Running jobs indicator trong status bar

### ✅ Keyboard Shortcuts
- [x] Ctrl+T: New tab
- [x] Ctrl+W: Close tab
- [x] Tab: Switch panel
- [x] F5: Copy
- [x] F6: Move
- [x] Delete: Delete files
- [x] F1: Help

### ✅ UI/UX
- [x] Dark theme
- [x] Hover states
- [x] Selection highlighting
- [x] Modal dialogs
- [x] Context-appropriate icons

## Next Steps (Real Rclone Integration)

### 1. Rclone Binary Management
```rust
// src-tauri/src/rclone/binary.rs
- Detect rclone binary path
- Allow user configuration
- Validate rclone version
```

### 2. RCD Daemon Management
```rust
// src-tauri/src/rclone/daemon.rs
- Start rclone rcd with auth
- Keep daemon alive
- Restart on failure
```

### 3. RC API Client
```rust
// src-tauri/src/rclone/client.rs
- HTTP client cho RC API
- Implement all RC endpoints:
  - operations/list
  - operations/stat
  - operations/copy
  - operations/move
  - operations/delete
  - job/status
  - job/list
  - config/listremotes
  - config/create
  - config/delete
```

### 4. OAuth Flow
```rust
// src-tauri/src/rclone/oauth.rs
- Handle OAuth providers (Drive, OneDrive)
- Open system browser
- Capture callback
- Store tokens securely
```

### 5. Advanced Features
- [ ] Drag & drop files between panels
- [ ] Context menu (right-click)
- [ ] File preview
- [ ] Rename operation (F2)
- [ ] Multiple selection modes
- [ ] Progress bars trong file list
- [ ] Cancel/pause jobs
- [ ] Settings page
- [ ] Theme customization

## Development Commands

```bash
# Install dependencies
npm install

# Run dev mode (requires Rust)
npm run tauri dev

# Build production
npm run tauri build

# Type check
npm run check

# Frontend only
npm run dev
```

## Configuration

### Tailwind Dark Theme Colors
```js
dark-bg: #1e1e1e      // Main background
dark-panel: #252526    // Panel background
dark-border: #3e3e42   // Border color
dark-hover: #2a2d2e    // Hover state
dark-text: #cccccc     // Text color
dark-text-dim: #858585 // Dimmed text
dark-accent: #0e639c   // Accent color
```

### Tauri Window Settings
- Default size: 1400x900
- Min size: 1000x600
- Theme: dark

## Notes

- Mock data hiện tại trong `src-tauri/src/rclone.rs`
- Icons là placeholder (cần thay bằng icons thật)
- Chưa implement thực sự rclone integration
- File operations chỉ log ra console
- Keyboard shortcuts global trong app
