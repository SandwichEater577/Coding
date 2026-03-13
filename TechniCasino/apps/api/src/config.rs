// ── config.rs — Application Configuration ──
//
// Define a Config struct with these fields:
//   database_url: String
//   redis_url: String
//   port: u16 (default 3001)
//   jwt_secret: String
//   jwt_expiration_hours: u64 (default 24)
//   house_edge: f64 (default 0.02 = 2%)
//
// Implement Config::from_env() that reads from environment variables.
// Panic with clear error message if required vars are missing.
//
// Use dotenvy to load .env file.
// Parse PORT as u16, JWT_EXPIRATION_HOURS as u64.
