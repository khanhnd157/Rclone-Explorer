<div class="flex flex-col h-full p-6">
  <h1 class="text-2xl font-semibold mb-6">Help</h1>

  <div class="max-w-3xl space-y-6">
    <section class="bg-dark-panel rounded border border-dark-border p-6">
      <h2 class="text-lg font-medium mb-4">Keyboard Shortcuts</h2>
      <div class="grid grid-cols-2 gap-4 text-sm">
        <div class="flex justify-between items-center py-2 border-b border-dark-border">
          <span class="text-dark-text-dim">New Tab</span>
          <kbd class="bg-dark-bg px-3 py-1 rounded text-xs">Ctrl+T</kbd>
        </div>
        <div class="flex justify-between items-center py-2 border-b border-dark-border">
          <span class="text-dark-text-dim">Close Tab</span>
          <kbd class="bg-dark-bg px-3 py-1 rounded text-xs">Ctrl+W</kbd>
        </div>
        <div class="flex justify-between items-center py-2 border-b border-dark-border">
          <span class="text-dark-text-dim">Switch Panel</span>
          <kbd class="bg-dark-bg px-3 py-1 rounded text-xs">Tab</kbd>
        </div>
        <div class="flex justify-between items-center py-2 border-b border-dark-border">
          <span class="text-dark-text-dim">Copy Files</span>
          <kbd class="bg-dark-bg px-3 py-1 rounded text-xs">F5</kbd>
        </div>
        <div class="flex justify-between items-center py-2 border-b border-dark-border">
          <span class="text-dark-text-dim">Move Files</span>
          <kbd class="bg-dark-bg px-3 py-1 rounded text-xs">F6</kbd>
        </div>
        <div class="flex justify-between items-center py-2 border-b border-dark-border">
          <span class="text-dark-text-dim">Delete Files</span>
          <kbd class="bg-dark-bg px-3 py-1 rounded text-xs">Delete</kbd>
        </div>
        <div class="flex justify-between items-center py-2 border-b border-dark-border">
          <span class="text-dark-text-dim">Show Help</span>
          <kbd class="bg-dark-bg px-3 py-1 rounded text-xs">F1</kbd>
        </div>
      </div>
    </section>

    <section class="bg-dark-panel rounded border border-dark-border p-6">
      <h2 class="text-lg font-medium mb-4">About Rclone Explorer</h2>
      <div class="space-y-4 text-sm">
        <div class="flex items-center gap-4">
          <div class="w-16 h-16 bg-dark-accent rounded flex items-center justify-center font-bold text-3xl">
            R
          </div>
          <div>
            <p class="font-semibold text-lg">Rclone Explorer</p>
            <p class="text-dark-text-dim">Version 0.1.0</p>
          </div>
        </div>
        <p class="text-dark-text-dim">
          Cross-platform desktop GUI for rclone with dual panel file manager interface.
        </p>
        <div class="pt-4 border-t border-dark-border">
          <h3 class="font-medium mb-2">Technology Stack</h3>
          <ul class="space-y-1 text-dark-text-dim">
            <li>• SvelteKit + TypeScript</li>
            <li>• Tauri 2 (Rust)</li>
            <li>• Tailwind CSS</li>
          </ul>
        </div>
      </div>
    </section>

    <section class="bg-dark-panel rounded border border-dark-border p-6">
      <h2 class="text-lg font-medium mb-4">Resources</h2>
      <div class="space-y-2 text-sm">
        <a href="https://rclone.org" target="_blank" class="block text-dark-accent hover:underline">
          Rclone Documentation
        </a>
        <a href="https://tauri.app" target="_blank" class="block text-dark-accent hover:underline">
          Tauri Documentation
        </a>
        <a href="https://kit.svelte.dev" target="_blank" class="block text-dark-accent hover:underline">
          SvelteKit Documentation
        </a>
      </div>
    </section>
  </div>
</div>
