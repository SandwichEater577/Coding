// ── routes/auth.rs — Authentication Routes ──
//
// POST /api/auth/register
//   - Extract: Json<CreateUserDTO>
//   - Validate username (3-20 chars, alphanumeric + underscore)
//   - Validate email format
//   - Validate password (min 8, 1 upper, 1 lower, 1 number)
//   - Check if username/email already exists → 409 Conflict
//   - Hash password with Argon2
//   - Insert user into DB
//   - Auto-create wallet with 0 balance
//   - Return 201 Created
//
// POST /api/auth/login
//   - Extract: Json<LoginDTO>
//   - Find user by email → 401 if not found
//   - Verify password with Argon2 → 401 if wrong
//   - Generate JWT with user.id as subject
//   - Return Json<AuthResponse> { token, user }
//
// GET /api/auth/me (protected — requires auth middleware)
//   - Extract user_id from auth middleware
//   - Query user profile from DB
//   - Return Json<UserProfile>
