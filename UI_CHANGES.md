# UI/UX Changes - Nav Tabs Implementation

## Summary

Đã thay thế top menu bar cũ bằng **nav tabs** hiện đại với icon + text layout.

## Changes Made

### 1. New Navigation System

**Created:**
- `src/lib/stores/navigation.ts` - Store quản lý active tab
- `src/lib/components/TopNavTabs.svelte` - Nav tabs component

**Nav Tabs Layout:**
- 5 tabs: Explorer, Accounts, Tasks, Options, Help
- Mỗi tab: **Monochrome SVG icon** (20px) phía trên, text (10px) phía dưới
- Icons: Line-based, 1.5px stroke, currentColor
- Tab active: bg #1e1e1e + top border #0e639c
- Tab hover: bg #333333
- Logo "R" + "Rclone Explorer" text ở bên trái

### 2. Page Components

**Created (src/lib/components/pages/):**
- `ExplorerPage.svelte` - Dual panel file manager (moved from +page)
- `AccountsPage.svelte` - Full page accounts manager
- `TasksPage.svelte` - Full page tasks list
- `OptionsPage.svelte` - Settings page với rclone config, appearance, refresh interval
- `HelpPage.svelte` - Help page với shortcuts, about, resources

### 3. Removed Components

**Deleted:**
- `TopBar.svelte` - Old menu bar
- `AccountsModal.svelte` - Replaced by AccountsPage
- `TasksModal.svelte` - Replaced by TasksPage

### 4. Updated Files

**`src/routes/+layout.svelte`:**
- Import TopNavTabs thay TopBar
- Import tất cả page components
- Conditional rendering dựa trên $activeNavTab
- Removed modals (AccountsModal, TasksModal, RegisterAccountModal)
- Giữ KeyboardHelper và StatusBar

**`src/routes/+page.svelte`:**
- Cleared content (routing handled by layout)

**`src/lib/stores/index.ts`:**
- Export navigation store

## UI Features

### TopNavTabs Component
```svelte
<nav> 
  Logo + Name → [Explorer][Accounts][Tasks][Options][Help]
</nav>
```

**Styling:**
- Height: 64px (h-16)
- Tab padding: px-6 py-2
- Icon: text-2xl (emoji)
- Text: text-xs font-medium
- Active: border-b-2 border-dark-accent
- Hover: bg-dark-hover

### Page-Specific Features

**ExplorerPage:**
- TabBar + DualPanel (original functionality)
- Multi-tab file manager

**AccountsPage:**
- Search bar + Add/Refresh buttons
- Table view: Name, Provider, Status, Actions
- RegisterAccountModal integration

**TasksPage:**
- Empty state với icon
- Table: Operation, Source, Dest, Status, Progress, Speed
- Status badges với colors

**OptionsPage:**
- 3 sections: Rclone, Appearance, File List
- Form inputs: text, checkbox, number
- Save button

**HelpPage:**
- Keyboard shortcuts grid (2 cols)
- About section với logo + version
- Resources links

## Benefits

✅ **Better UX:**
- Clear visual hierarchy
- Icon + text dễ nhận biết
- Active state rõ ràng
- No dropdowns/hidden menus

✅ **Full Page Views:**
- More space cho content
- No modal overlays
- Better for large datasets

✅ **Clean Architecture:**
- Separate page components
- Store-based routing
- Easy to extend

✅ **Responsive:**
- Flex layout adapts
- Icon + text scales well

## Dark Theme Colors

```css
bg-dark-bg: #1e1e1e       /* Main background */
bg-dark-panel: #252526    /* Panel/card bg */
border-dark-border: #3e3e42 /* Borders */
bg-dark-hover: #2a2d2e    /* Hover state */
text-dark-text: #cccccc   /* Normal text */
text-dark-text-dim: #858585 /* Dimmed text */
bg-dark-accent: #0e639c   /* Accent color */
```

## Testing

✅ Frontend build: Success
✅ All pages render
✅ Tab switching works
✅ Dark theme consistent
✅ StatusBar still visible

## Window Controls & Title Bar (NEW)

**Separate Title Bar** (32px):
- Logo + app name (compact: 20px, text-xs)
- Window controls on right
- Background: #323233 (darker)
- Full draggable region

**WindowControls.svelte**:
- Custom minimize/maximize/close buttons
- Replaces OS default title bar
- Fork-inspired style:
  - Width: 44px per button
  - Icons: SVG line-based (─ □ ✕), 10px
  - Hover: minimize/max → #404040, close → #cc6666
- Integrated in separate title bar
- Drag region: `data-tauri-drag-region`
- Tauri config: `decorations: false`

**Nav Tabs** (48px):
- Now has full row, no logo/controls
- More horizontal space for tabs
- Background: #2b2b2b

## Technical Limitations

**Rounded Window Corners:**
- Không thể implement với `decorations: false` trong Tauri
- Transparent windows trên Windows gây rendering issues
- Native OS rounded corners chỉ available khi dùng default decorations
- Workaround: Focus vào internal UI polish thay vì window shape

## Next Steps

Optional enhancements:
- [ ] Add transition animations between pages
- [ ] Persist active tab to localStorage
- [ ] Add keyboard shortcuts (Ctrl+1-5) to switch tabs
- [ ] Add notification badges on tabs (e.g., running jobs count)
- [ ] Double-click title bar to maximize
- [ ] Different maximize icon when maximized (⧉)