// ── models/user.rs — User data models ──
//
// Structs:
//   User           → full user row from DB (id, username, email, password_hash, created_at)
//   CreateUser     → registration request payload
//   LoginRequest   → login request payload
//   AuthResponse   → JWT token response
//   UserProfile    → public user info (no password_hash)
