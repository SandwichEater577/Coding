// ── services/auth.rs — Auth Business Logic ──
//
// pub async fn register(pool: &PgPool, dto: CreateUserDTO) -> Result<UserProfile, AppError>
//   - Validate all fields
//   - Check username uniqueness → Conflict
//   - Check email uniqueness → Conflict
//   - Hash password with Argon2 (see utils/hash.rs)
//   - INSERT INTO users
//   - INSERT INTO wallets (user_id, balance=0)
//   - Return UserProfile
//
// pub async fn login(pool: &PgPool, config: &Config, dto: LoginDTO) -> Result<AuthResponse, AppError>
//   - SELECT user by email → Unauthorized if not found
//   - Verify password against hash → Unauthorized if wrong
//   - Generate JWT token (see utils/jwt.rs)
//   - Return AuthResponse { token, user: UserProfile }
