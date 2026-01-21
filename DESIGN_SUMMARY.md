# Rclone Explorer - Design Summary

## Overview

Rclone Explorer là desktop file manager GUI cho rclone, được thiết kế theo phong cách **Fork Git client** với dark theme professional.

## Design Philosophy

### Fork-Inspired Principles

1. **3-Layer Architecture**
   - Top: Nav tabs (global navigation)
   - Middle: Sub-toolbar (context actions)
   - Bottom: Content area (master-detail)

2. **Minimalist & Professional**
   - Flat design, no gradients
   - Subtle borders (#3e3e42)
   - Consistent spacing
   - Line-based icons

3. **Dark Theme Optimized**
   - Background hierarchy: #1e1e1e → #252526 → #2b2b2b
   - Text hierarchy: #e0e0e0 → #cccccc → #858585
   - Accent: #0e639c (blue)
   - Alerts: #7cc77c (green), #c77c7c (red)

## Component Breakdown

### 1. Window Frame

**Custom Title Bar** (32px height) + **Nav Bar** (48px height)
```
┌─────────────────────────────────────────────────┐
│ [R] Rclone Explorer                    [─][□][✕]     │ ← Title Bar
├─────────────────────────────────────────────────┤
│ [Explorer][Accounts][Tasks][Options][Help]      │ ← Nav Tabs
│  ↑                                               │
│  Navigation Tabs                                 │
└─────────────────────────────────────────────────┘
```

**Features:**
- No OS decorations (`decorations: false`)
- Separate title bar với logo + window controls
- Draggable title bar (`data-tauri-drag-region`)
- Nav tabs ở hàng riêng bên dưới

**Title Bar:**
- Height: 32px (compact)
- Background: #323233 (darker)
- Logo: 20px, compact version

**Window Controls:**
- Width: 44px each
- Icons: SVG line art (1px stroke, 10px size)
- Hover states:
  - Min/Max: bg #404040
  - Close: bg #cc6666 (red)

### 2. Navigation Tabs

**Layout:** Icon (20px) above, text (10px) below
**Active state:** 
- Top border: 2px #0e639c
- Background: #1e1e1e
- Text: #0e639c
**Inactive:**
- Text: #a0a0a0
- Hover: bg #333333

**Tabs:**
1. Explorer ⊞ (grid icon)
2. Accounts ⚭ (user icon)
3. Tasks ⏱ (clock icon)
4. Options ⚙ (gear icon)
5. Help ? (question icon)

**Icon Style:** Monochrome SVG, 20×20px, 1.5px stroke

### 3. Sub-Toolbar (40px height)

**Purpose:** Context-specific actions per tab

**Explorer:**
- New Tab, Sync, Mount, Filter

**Accounts:**
- Add Account, Refresh, Search

**Tasks:**
- Running/Completed filters, Clear Finished

**Options:**
- General/Advanced sections

### 4. Content Area

#### Explorer Page
```
┌─────────────────────────────────────────┐
│ [Tab 1] [Tab 2] [+]                     │
├──────────────────┬──────────────────────┤
│ Left Panel       │ Right Panel          │
│ [Remote: ▼]      │ [Remote: ▼]          │
│ ← → ↑ ⟳ /path    │ ← → ↑ ⟳ /path        │
│ 📁 Folder         │ 📁 Folder             │
│ 📄 File           │ 📄 File               │
└──────────────────┴──────────────────────┘
```

#### Accounts Page (Master-Detail)
```
┌────────────────────────────────────────┐
│ ┌──────────┬─────────────────────────┐ │
│ │ Master   │ Detail Form             │ │
│ │ List     │ ┌───────────────────┐   │ │
│ │          │ │ Header            │   │ │
│ │ • acc1   │ ├───────────────────┤   │ │
│ │ • acc2   │ │ General Section   │   │ │
│ │ • acc3   │ │ Auth Section      │   │ │
│ │          │ │ Advanced Section  │   │ │
│ │          │ └───────────────────┘   │ │
│ └──────────┴─────────────────────────┘ │
└────────────────────────────────────────┘
```

**Master (320px):**
- Filter input
- List items: icon + name + badge
- Selected: bg #1e1e1e + left border #0e639c

**Detail (flex-1):**
- Header: Icon + name + actions
- Sections: Collapsible/expandable
- Form fields: Label (w-32) + Input (flex-1)
- Footer: Cancel + Save buttons

### 5. Modals

**Register Account Dialog:**
```
┌───────────────────────────────┐
│ ➕ Register Account        ×  │
├──────────┬────────────────────┤
│ Provider │ Configuration      │
│ List     │ Form               │
│          │                    │
│ • Drive  │ [Account Name]     │
│ • S3     │ [Provider]         │
│ • SFTP   │ ℹ️ Info box        │
└──────────┴────────────────────┘
│         [Back]  [Continue]     │
└────────────────────────────────┘
```

### 6. Status Bar (32px height)

**Layout:** Left info + Right status
```
┌─────────────────────────────────────────┐
│ remote:/path | 15 items | Selected: 3  │
│                          ● 2 jobs running│
└─────────────────────────────────────────┘
```

## Color Palette

### Backgrounds
```css
--main-bg:       #1e1e1e
--panel-bg:      #252526
--nav-bg:        #2b2b2b
--hover-bg:      #2a2d2e
--control-hover: #333333
```

### Borders
```css
--border:        #3e3e42
--border-active: #0e639c
```

### Text
```css
--text-primary:   #e0e0e0
--text-secondary: #cccccc
--text-dim:       #858585
--text-inactive:  #a0a0a0
```

### Accents
```css
--accent:        #0e639c
--success:       #7cc77c
--error:         #c77c7c
--warning:       #c7c77c
--close-hover:   #cc6666
```

## Typography

### Font Stack
```css
font-family: -apple-system, BlinkMacSystemFont, 
             'Segoe UI', Roboto, 'Helvetica Neue', 
             Arial, sans-serif;
```

### Sizes
- Nav tabs text: 10px uppercase
- Sub-toolbar: 12px
- Body text: 14px
- Headers: 16-20px
- Icons: 20-24px

## Spacing System

**Base unit:** 4px (0.25rem)

- xs: 4px (gap-1)
- sm: 8px (gap-2)
- md: 12px (gap-3)
- lg: 16px (gap-4)
- xl: 24px (gap-6)

## Interactive States

### Buttons
```css
Default:  bg transparent, text #cccccc
Hover:    bg #2a2d2e, text #e0e0e0
Active:   bg #0e639c, text #ffffff
Disabled: opacity 50%
```

### List Items
```css
Default:  bg transparent
Hover:    bg #2a2d2e
Selected: bg #1e1e1e, border-l #0e639c
```

### Input Fields
```css
Default:  bg #252526, border #3e3e42
Focus:    border #0e639c
Error:    border #c77c7c
```

## Transitions

**Standard:** 150ms ease-in-out
**Fast:** 100ms
**Slow:** 300ms

```css
transition: all 150ms ease-in-out;
```

## Icons

**Style:** Monochrome line-based SVG
**Stroke:** 1.5px (nav tabs), 1px (window controls)
**Format:** Inline SVG with `currentColor`
**Size:** 20px (nav tabs), 12px (window controls)

**Nav Tab Icons:**
- Explorer: Grid layout (⊞)
- Accounts: User profile (⚭)
- Tasks: Clock (⏱)
- Options: Settings gear (⚙)
- Help: Question mark (?)

**Window Controls:**
- Minimize: ─
- Maximize: □
- Close: ✕

**File Icons:**
- Folder: 📁 (emoji fallback)
- File: 📄 (emoji fallback)

## Responsive Behavior

### Window Sizes
- Minimum: 1000x600
- Default: 1400x900
- Panels: Resizable with divider

### Breakpoints
- Compact: < 1200px (reduce padding)
- Standard: >= 1200px (normal spacing)
- Wide: >= 1600px (max-width content)

## Accessibility

### Keyboard Navigation
- Tab: Focus next element
- Shift+Tab: Focus previous
- Enter: Activate button
- Arrow keys: Navigate lists
- Esc: Close modals

### Screen Readers
- ARIA labels on all buttons
- Semantic HTML structure
- Focus indicators visible

## Performance

### Optimization
- Virtual scrolling cho long lists
- Lazy loading cho heavy components
- Debounced search inputs
- Memoized derived values

## Browser Support

**Target:** Chromium (Tauri uses WebView2/WebKit)
- Modern CSS (Grid, Flexbox, Custom Properties)
- ES2020+ JavaScript
- No polyfills needed

## Design Tokens

```typescript
const theme = {
  colors: {
    bg: {
      main: '#1e1e1e',
      panel: '#252526',
      nav: '#2b2b2b',
      hover: '#2a2d2e'
    },
    text: {
      primary: '#e0e0e0',
      secondary: '#cccccc',
      dim: '#858585'
    },
    accent: '#0e639c',
    border: '#3e3e42'
  },
  spacing: {
    xs: '4px',
    sm: '8px',
    md: '12px',
    lg: '16px',
    xl: '24px'
  },
  typography: {
    xs: '10px',
    sm: '12px',
    base: '14px',
    lg: '16px',
    xl: '20px'
  }
};
```

## Inspiration Sources

1. **Fork Git Client**
   - 3-layer architecture
   - Master-detail layouts
   - Minimal window controls

2. **VS Code**
   - Dark theme colors
   - Status bar design
   - Panel management

3. **Figma**
   - Properties panel
   - Layer list
   - Toolbar organization

## Future Considerations

### Themes
- [ ] Light theme variant
- [ ] Custom theme support
- [ ] Accent color picker

### Advanced Features
- [ ] Drag & drop files
- [ ] Context menus
- [ ] File preview
- [ ] Multiple windows
- [ ] Workspace presets

## Documentation

- **WINDOW_CONTROLS_DESIGN.md** - Window controls specs
- **UI_CHANGES.md** - Implementation changes log
- **IMPLEMENTATION_SUMMARY.md** - Overall project summary

## Summary

Rclone Explorer implements Fork-inspired professional dark theme với:
- ✅ Custom window controls
- ✅ 3-layer navigation architecture
- ✅ Master-detail layouts
- ✅ Consistent spacing & colors
- ✅ Smooth interactions
- ✅ Accessible & keyboard-friendly

Perfect balance giữa aesthetics và functionality cho professional file manager.
