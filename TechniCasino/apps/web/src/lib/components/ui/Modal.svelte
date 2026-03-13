<!-- ── Modal Component ──
  Overlay modal dialog with neon styling.

  Props:
  - isOpen: boolean
  - title: string
  - size: 'sm' | 'md' | 'lg'
  - closable: boolean (show X button, close on backdrop click)

  Features:
  - Backdrop: dark semi-transparent overlay with blur
  - Content card: --color-surface background, neon border glow
  - Enter animation: fade backdrop + scale/fly content
  - Exit animation: reverse
  - Close on Escape key
  - Close on backdrop click (if closable)
  - Trap focus inside modal for accessibility

  Slots: 'header', 'default' (body), 'footer'
-->
