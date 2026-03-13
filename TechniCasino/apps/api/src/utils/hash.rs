// ── utils/hash.rs — Password Hashing (Argon2) ──
//
// pub fn hash_password(password: &str) -> Result<String, AppError>
//   - Use argon2 crate with default params
//   - Generate random salt
//   - Return PHC-formatted hash string
//
// pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError>
//   - Parse PHC hash string
//   - Verify password against hash
//   - Return true if match, false if not
//
// NEVER store plaintext passwords. EVER.
// Argon2 is the winner of the Password Hashing Competition.
// It's memory-hard, making GPU/ASIC attacks extremely expensive.
