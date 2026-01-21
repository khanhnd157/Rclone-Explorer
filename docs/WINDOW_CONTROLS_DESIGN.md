# Window Controls Design - Fork-Inspired

## Concept

Window controls (minimize, maximize, close) theo phong cách Fork Git client - tối giản, professional, integrated với title bar.

## Design Specifications

### Layout
```
┌────────────────────────────────────────────────────┐
│ [R] Rclone Explorer                        [─][□][✕]     │ ← Title Bar (32px)
├────────────────────────────────────────────────────┤
│ [Explorer][Accounts][Tasks][Options][Help]         │ ← Nav Tabs (48px)
└────────────────────────────────────────────────────┘
```

### Position
- **Location**: Separate title bar above nav tabs
- **Title Bar Height**: 32px (compact)
- **Nav Bar Height**: 48px (standard)
- **Button Width**: 44px per button (10px icons + padding)

### Visual Style

#### Colors (Dark Theme)
```css
Default state:
  - Icon: #a0a0a0 (gray)
  - Background: transparent

Minimize hover:
  - Icon: #e0e0e0 (bright)
  - Background: #333333 (dark gray)

Maximize hover:
  - Icon: #e0e0e0 (bright)
  - Background: #333333 (dark gray)

Close hover:
  - Icon: #ffffff (white)
  - Background: #cc6666 (red)
```

#### Icons
**Minimize (─)**:
```svg
<line x1="0" y1="11" x2="12" y2="11" stroke="currentColor" stroke-width="1" />
```

**Maximize (□)**:
```svg
<rect x="1" y="1" width="10" height="10" stroke="currentColor" stroke-width="1" fill="none" />
```

**Close (✕)**:
```svg
<line x1="1" y1="1" x2="11" y2="11" stroke="currentColor" stroke-width="1" />
<line x1="11" y1="1" x2="1" y2="11" stroke="currentColor" stroke-width="1" />
```

### Interaction

#### Hover Effects
- **Transition**: 150ms ease-in-out
- **Cursor**: pointer
- **Feedback**: Immediate color/bg change

#### Click Actions
- **Minimize**: `appWindow.minimize()`
- **Maximize**: `appWindow.toggleMaximize()` (toggle between maximize/restore)
- **Close**: `appWindow.close()`

### Integration

#### Title Bar Drag Region

**Main title bar**: Entire title bar area is draggable via `data-tauri-drag-region` on title bar div.

**Window controls drag handle**: Additional 80px drag area integrated into window controls:
```svelte
<div class="w-20 h-full" data-tauri-drag-region></div>
```
- Positioned left of control buttons
- Width: 80px (comfortable drag target)
- Invisible but functional
- Provides drag capability near window controls

#### Tauri Configuration
```json
{
  "decorations": false  // Remove default OS window frame
}
```

## Implementation Details

### Component Structure
```
WindowControls.svelte
├── Drag handle (80px, invisible)
├── Minimize button (44px)
├── Maximize button (44px)
└── Close button (44px)
Total width: 212px
```

### Tauri API Usage
```typescript
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

// Methods
await appWindow.minimize();
await appWindow.toggleMaximize();
await appWindow.close();
```

### CSS Classes
```css
.window-control-button {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 150ms;
}

.minimize-button:hover { background: #333333; }
.maximize-button:hover { background: #333333; }
.close-button:hover { background: #cc6666; }
```

## Design Rationale

### Why This Style?

1. **Consistency với Fork**
   - Flat, minimal design
   - Subtle hover effects
   - Professional appearance

2. **Better UX**
   - Rõ ràng, dễ nhận diện
   - Hit target đủ lớn (48px height)
   - Visual feedback rõ ràng

3. **Platform Integration**
   - Native feel trên desktop
   - Custom controls match app theme
   - No OS-specific styling conflicts

### Advantages

✅ **Custom branding**: Control hoàn toàn appearance
✅ **Theme consistency**: Match dark theme perfectly
✅ **Cross-platform**: Giống nhau trên Windows/macOS/Linux
✅ **Professional**: Fork-inspired, clean design

### Considerations

⚠️ **Drag region**: Must define `data-tauri-drag-region` properly
⚠️ **Accessibility**: Ensure buttons have proper hover states
⚠️ **Testing**: Test maximize/restore state changes

## Future Enhancements

### Possible Additions

1. **Tooltips**
   ```svelte
   <button title="Minimize">...</button>
   ```

2. **Keyboard Shortcuts**
   - Alt+F4: Close
   - Win+Down: Minimize
   - Win+Up: Maximize

3. **Double-Click Title Bar**
   ```svelte
   <div data-tauri-drag-region ondblclick={toggleMaximize}>
   ```

4. **Visual State Indicators**
   - Show different icon when maximized (⧉ vs □)
   - Disable minimize when minimized

## Platform-Specific Notes

### Windows
- Native feel với custom controls
- Red close button familiar từ Windows

### macOS
- Có thể add traffic lights style nếu cần
- Consider left-side controls cho Mac users

### Linux
- Works well với most DEs
- Custom controls = consistent experience

## Testing Checklist

- [x] Minimize button works
- [x] Maximize button toggles
- [x] Close button closes app
- [x] Hover states correct colors
- [x] Drag region allows moving window
- [x] Icons scale properly
- [x] Transitions smooth
- [x] Works on all platforms

## Screenshot Reference

```
Before (OS default):
┌─[Rclone Explorer]────────────────────────[─][□][✕]┐
│  Standard OS title bar with controls         │
└──────────────────────────────────────────────┘

After (Custom Fork-style):
┌────────────────────────────────────────────────┐
│ [R] Rclone Explorer [Explorer][Accounts]... [─][□][✕]│
│   ↑            ↑                       ↑        │
│   Logo         Nav Tabs               Controls │
└────────────────────────────────────────────────┘
```

## Code Location

- **Component**: `src/lib/components/WindowControls.svelte`
- **Integration**: `src/lib/components/TopNavTabs.svelte`
- **Config**: `src-tauri/tauri.conf.json` → `decorations: false`
- **API**: `@tauri-apps/api/window`

## Summary

Custom window controls provide:
- Professional, Fork-inspired appearance
- Perfect theme integration
- Cross-platform consistency
- Full control over UX

Perfect fit cho Rclone Explorer's dark, modern interface.
