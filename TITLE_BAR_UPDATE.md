# Title Bar Update - Rclone Explorer

## Change Summary

Đã tách **Title Bar** và **Nav Tabs** thành 2 hàng riêng biệt để tạo layout rõ ràng hơn, theo phong cách VS Code / Fork.

## Before vs After

### Before (Single Row)
```
┌──────────────────────────────────────────────┐
│ [R] Rclone Explorer [Exp][Acc][Tasks]... [─][□][✕]│ ← 48px
└──────────────────────────────────────────────┘
```

### After (Two Rows)
```
┌──────────────────────────────────────────────┐
│ [R] Rclone Explorer                    [─][□][✕]   │ ← 32px Title Bar
├──────────────────────────────────────────────┤
│ [Explorer][Accounts][Tasks][Options][Help]   │ ← 48px Nav Tabs
└──────────────────────────────────────────────┘
```

## Design Rationale

### Why Separate?

1. **Clearer Hierarchy**
   - Title bar = window management
   - Nav tabs = content navigation
   - No mixing of concerns

2. **Better UX**
   - Larger drag area (full title bar)
   - Window controls always visible
   - Tabs không bị chen lấn

3. **Industry Standard**
   - VS Code: Title bar riêng
   - Fork: Title bar riêng
   - Most modern apps: Separate title + nav

## Layout Specifications

### Title Bar (32px)

**Background:** `#323233` (darker than nav)
**Height:** `32px` (compact)
**Padding:** `px-3`

**Content:**
- Left: Logo (20px) + App name (text-xs)
- Right: Window controls (3 buttons × 44px)
- Full draggable region

**Logo:**
- Size: 20×20px (was 28px)
- Font: 9px (was xs)
- Style: Same blue #0e639c

### Nav Tabs (48px)

**Background:** `#2b2b2b`
**Height:** `48px` (standard)
**Padding:** `px-3`

**Content:**
- Tabs only (no logo, no controls)
- Same tab design as before
- More horizontal space

## Window Controls Updates

### Size Changes
- Width: `44px` (was 48px) - slightly compact
- Icon: `10×10px` (was 12px)
- Height: `32px` (match title bar)

### Visual
- Same hover colors
- Slightly smaller to fit compact title bar
- Better proportions

## Colors

```css
Title Bar:     #323233  (darker)
Nav Bar:       #2b2b2b  (standard)
Border:        #3e3e42

Controls Hover:
  Min/Max:     #404040
  Close:       #cc6666
```

## Implementation

### TopNavTabs.svelte Structure
```svelte
<div>
  <!-- Title Bar -->
  <div class="h-8 bg-[#323233]" data-tauri-drag-region>
    <Logo /> + <WindowControls />
  </div>
  
  <!-- Nav Tabs -->
  <nav class="h-12 bg-[#2b2b2b]">
    <Tabs />
  </nav>
</div>
```

### WindowControls.svelte
- Adjusted for 32px height
- Smaller icons (10px)
- Width: 44px per button

## Advantages

✅ **Cleaner separation**: Window vs content navigation
✅ **More space**: Tabs have full row
✅ **Better drag area**: Entire title bar draggable
✅ **Professional**: Matches VS Code / Fork style
✅ **Scalable**: Easy to add more tabs

## Total Height

```
Title Bar:   32px
Nav Tabs:    48px
───────────────────
Total:       80px  (vs 48px before)
```

**Trade-off:** +32px vertical space
**Benefit:** Much better UX & clarity

## Future Enhancements

### Possible Additions

1. **Breadcrumb in Title Bar**
   ```
   [R] Rclone Explorer › Workspace › Project
   ```

2. **Search in Title Bar**
   ```
   [Logo] [Search...] [Controls]
   ```

3. **Status Indicators**
   ```
   [Logo] [Name] 🔴 Syncing... [Controls]
   ```

## Visual Comparison

### VS Code Style
```
┌─────────────────────────────────────┐
│ VS Code - file.ts            [─][□][✕]│ ← Title
├─────────────────────────────────────┤
│ File Edit View... Explorer Ext...   │ ← Menu/Tabs
```

### Rclone Explorer (New)
```
┌─────────────────────────────────────┐
│ [R] Rclone Explorer              [─][□][✕]│ ← Title
├─────────────────────────────────────┤
│ [Explorer][Accounts][Tasks]...      │ ← Tabs
```

Perfect alignment với professional desktop apps!

## Testing

✅ Build: Success
✅ Layout: Title + Nav separate
✅ Drag: Title bar draggable
✅ Controls: Working correctly
✅ Responsive: Scales properly

## Summary

Tách title bar và nav tabs tạo:
- Professional appearance
- Clear visual hierarchy
- Better usability
- Industry-standard layout

Perfect cho Rclone Explorer's Fork-inspired design! 🎯
