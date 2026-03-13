// ── Toast Notification Store ──
// Svelte writable store for toast messages.
//
// Toast shape: { id, type (success/error/warning/info), message, duration }
//
// Export functions:
// addToast(type, message, duration?)  → add toast, auto-remove after duration
// removeToast(id)                     → manually remove
//
// Toasts stack in bottom-right corner.
// Use Svelte transitions (fly, fade) for enter/exit animations.
