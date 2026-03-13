// ── models/user.rs — User Data Models ──
//
// Define structs with #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]:
//
// User            → id (Uuid), username, email, password_hash, avatar_url (Option), created_at
// CreateUserDTO   → username, email, password (from register request body)
// LoginDTO        → email, password
// AuthResponse    → token (String), user (UserProfile)
// UserProfile     → id, username, avatar_url, created_at (NO password_hash!)
// UserStats       → total_games, total_wins, total_losses, win_rate (f64), biggest_win (i64)
//
// Implement From<User> for UserProfile (strip sensitive fields).
//
// For request bodies: derive Deserialize
// For response bodies: derive Serialize
// For DB rows: derive sqlx::FromRow
