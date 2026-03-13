<!-- ── Toast Component ──
  Single toast notification message.

  Props:
  - type: 'success' (green) | 'error' (magenta) | 'warning' (gold) | 'info' (cyan)
  - message: string
  - id: string (for removal)

  Features:
  - Colored left border or icon based on type
  - Auto-dismiss progress bar at bottom
  - Close button (X)
  - Svelte transition:fly from right side
  - Neon glow matching the type color

  Render a ToastContainer in +layout.svelte that maps
  over the toast store and renders each Toast.
-->
