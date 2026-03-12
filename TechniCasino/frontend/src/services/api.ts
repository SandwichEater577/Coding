// ── services/api.ts — Base HTTP client ──
// Wraps fetch() with base URL, auth headers, error handling.
// Methods: get(), post(), put(), delete().
// Automatically attaches JWT from localStorage.

const API_BASE = "/api";
