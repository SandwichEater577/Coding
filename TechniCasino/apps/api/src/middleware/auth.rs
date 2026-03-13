// ── middleware/auth.rs — JWT Authentication Middleware ──
//
// Create an Axum extractor: AuthUser(pub Uuid)
//
// Implement FromRequestParts for AuthUser:
// 1. Get "Authorization" header from request
// 2. Strip "Bearer " prefix to get token string
// 3. If no token → return 401 Unauthorized
// 4. Decode token using jsonwebtoken::decode(token, secret, validation)
// 5. If invalid/expired → return 401
// 6. Extract user_id (Uuid) from claims.sub
// 7. Return AuthUser(user_id)
//
// Usage in route handlers:
//   async fn get_balance(AuthUser(user_id): AuthUser, State(state): State<AppState>) -> ...
//
// Claims struct:
//   sub: String (user UUID),
//   exp: usize (expiration timestamp),
//   iat: usize (issued at)
