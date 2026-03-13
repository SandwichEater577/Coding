// ── Auth Validators ──
// Export validation functions (pure functions, no framework dependency):
//
// validateEmail(email: string)    → { valid: boolean, error?: string }
//   - Must be valid email format
//
// validatePassword(password: string) → { valid: boolean, error?: string }
//   - Min 8 chars, at least 1 uppercase, 1 lowercase, 1 number
//
// validateUsername(username: string) → { valid: boolean, error?: string }
//   - 3-20 chars, alphanumeric + underscores only, no spaces
//
// These run on BOTH frontend and backend (that's why they're in shared/)
