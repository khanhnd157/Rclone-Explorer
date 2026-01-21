# Icons Design - Rclone Explorer

## Overview

Rclone Explorer sử dụng **monochrome line icons** theo phong cách Fork/VS Code để đảm bảo consistency và professional appearance.

## Icon System

### Style Guide

**Type:** Line-based SVG
**Stroke Width:** 1.5px
**Size:** 20×20px
**Color:** `currentColor` (inherits from parent)
**Style:** Rounded corners (`stroke-linecap: round`)

### Design Principles

1. **Simplicity**: Minimal strokes, clear shapes
2. **Consistency**: Same stroke width across all icons
3. **Clarity**: Recognizable at small sizes
4. **Scalability**: Vector-based, scales perfectly

## Nav Tab Icons

### 1. Explorer (Grid)
```svg
<svg width="20" height="20" viewBox="0 0 20 20">
  <path d="M3 5h14M3 10h14M3 15h14"/>
  <path d="M7 3v14M13 3v14"/>
</svg>
```
**Meaning:** Grid layout representing file panels
**States:** 
- Inactive: #a0a0a0
- Active: #0e639c
- Hover: #e0e0e0

### 2. Accounts (User)
```svg
<svg width="20" height="20" viewBox="0 0 20 20">
  <circle cx="10" cy="7" r="3"/>
  <path d="M5 17c0-2.5 2.5-4 5-4s5 1.5 5 4"/>
</svg>
```
**Meaning:** User profile for cloud accounts
**Alternative:** Could use cloud icon if needed

### 3. Tasks (Clock)
```svg
<svg width="20" height="20" viewBox="0 0 20 20">
  <circle cx="10" cy="10" r="7"/>
  <path d="M10 6v4l3 2"/>
</svg>
```
**Meaning:** Clock showing progress/time-based tasks
**Alternative:** List icon for task list view

### 4. Options (Settings)
```svg
<svg width="20" height="20" viewBox="0 0 20 20">
  <circle cx="10" cy="10" r="2"/>
  <path d="M10 3v1.5M10 15.5V17M6.34 6.34l-1.06-1.06
           M14.72 14.72l1.06 1.06M3 10h1.5M15.5 10H17
           M6.34 13.66l-1.06 1.06M14.72 5.28l1.06-1.06"/>
</svg>
```
**Meaning:** Settings/configuration gear
**Note:** 8 spokes for clear recognition

### 5. Help (Question)
```svg
<svg width="20" height="20" viewBox="0 0 20 20">
  <circle cx="10" cy="10" r="7"/>
  <path d="M8 8c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2"/>
  <circle cx="10" cy="14" r="0.5" fill="currentColor"/>
</svg>
```
**Meaning:** Help/information
**Note:** Question mark with dot

## Implementation

### Svelte Component
```svelte
<script>
  function getIcon(tabId: NavTab) {
    const icons: Record<NavTab, string> = {
      explorer: '<svg>...</svg>',
      // ...
    };
    return icons[tabId];
  }
</script>

<div class="mb-0.5">
  {@html getIcon(tab.id)}
</div>
```

### Color States
```css
/* Inactive */
color: #a0a0a0;

/* Hover */
color: #e0e0e0;

/* Active */
color: #0e639c;
```

## Icon Library

### Potential Additional Icons

**File Operations:**
```svg
<!-- Copy -->
<svg><rect/><rect transform="translate"/></svg>

<!-- Move -->
<svg><path d="M...arrow"/>...</svg>

<!-- Delete -->
<svg><path d="M...trash"/></svg>

<!-- Refresh -->
<svg><path d="M...circular arrow"/></svg>
```

**Cloud Providers:**
```svg
<!-- Cloud Generic -->
<svg><path d="M...cloud shape"/></svg>

<!-- Drive -->
<svg><path d="M...triangle"/></svg>

<!-- OneDrive -->
<svg><path d="M...cloud"/></svg>
```

**Status:**
```svg
<!-- Success -->
<svg><circle/><path d="M...checkmark"/></svg>

<!-- Error -->
<svg><circle/><path d="M...X"/></svg>

<!-- Warning -->
<svg><path d="M...triangle + !"/></svg>
```

## Advantages of Monochrome Icons

✅ **Consistency**: Same style throughout app
✅ **Themeable**: Works with any color scheme
✅ **Scalable**: SVG scales perfectly
✅ **Lightweight**: Small file size
✅ **Accessible**: Clear at any size
✅ **Professional**: Modern, clean look

## Comparison: Emoji vs Monochrome

### Before (Emoji)
```
📁 Explorer
👤 Accounts
⚙️ Tasks
🔧 Options
❓ Help
```

**Issues:**
- ❌ Inconsistent rendering across OS
- ❌ Colors don't match theme
- ❌ Hard to style
- ❌ Can look unprofessional

### After (Monochrome)
```
⊞ Explorer  (grid icon)
⚭ Accounts  (user icon)
⏱ Tasks     (clock icon)
⚙ Options   (gear icon)
? Help      (question icon)
```

**Benefits:**
- ✅ Consistent across platforms
- ✅ Matches dark theme
- ✅ Fully customizable
- ✅ Professional appearance

## Icon Sources

**Inspiration:**
- Lucide Icons (lucide.dev)
- Heroicons (heroicons.com)
- Feather Icons (feathericons.com)

**Customization:**
- Modified for Rclone Explorer
- Optimized for 20×20px
- Consistent 1.5px stroke

## Future Enhancements

### Icon Animation
```css
@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.loading-icon {
  animation: rotate 1s linear infinite;
}
```

### Icon Badges
```svg
<!-- Notification badge -->
<circle cx="16" cy="4" r="3" fill="#cc6666"/>
<text x="16" y="6" fill="white">3</text>
```

### Hover Effects
```css
.icon {
  transition: transform 150ms;
}

.icon:hover {
  transform: scale(1.1);
}
```

## Best Practices

1. **Keep It Simple**: Max 3-4 strokes per icon
2. **Optical Alignment**: Center visually, not mathematically
3. **Consistent Weight**: Always 1.5px stroke
4. **Round Caps**: Use `stroke-linecap: round`
5. **Test Sizes**: Ensure clarity at 16px, 20px, 24px

## Accessibility

### Screen Readers
```svelte
<button aria-label="Explorer">
  {@html getIcon('explorer')}
  <span class="sr-only">Explorer</span>
</button>
```

### Focus States
```css
button:focus {
  outline: 2px solid #0e639c;
  outline-offset: 2px;
}
```

## Summary

Monochrome line icons provide:
- Professional appearance
- Theme consistency
- Cross-platform reliability
- Easy maintenance
- Scalable design

Perfect cho Rclone Explorer's Fork-inspired interface.
