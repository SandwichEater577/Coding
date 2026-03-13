<!-- ── Button Component ──
  Reusable button with neon casino styling.

  Props:
  - variant: 'primary' (cyan) | 'secondary' (magenta) | 'gold' | 'ghost' | 'danger'
  - size: 'sm' | 'md' | 'lg'
  - disabled: boolean
  - loading: boolean (show spinner, disable click)
  - fullWidth: boolean

  Features:
  - Neon glow on hover (box-shadow)
  - Scale animation on click (transform: scale(0.95))
  - Loading spinner replaces text when loading=true
  - Disabled state: dimmed, no glow, cursor-not-allowed
  - Use Svelte transition:scale for mount/unmount

  Slot: default slot for button content (text, icons, etc.)
-->
