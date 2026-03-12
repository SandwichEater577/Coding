// ── middleware/auth.rs — Authentication middleware ──
// Extracts and validates JWT from Authorization header.
// Injects authenticated user ID into request extensions.
//
// Usage: apply to protected routes as a layer.
