// ── utils/jwt.rs — JWT Token Utilities ──
//
// pub fn encode_token(user_id: Uuid, secret: &str, expiration_hours: u64) -> Result<String, AppError>
//   - Create Claims { sub: user_id.to_string(), exp, iat }
//   - Encode with jsonwebtoken::encode() using HS256 algorithm
//
// pub fn decode_token(token: &str, secret: &str) -> Result<Claims, AppError>
//   - Decode with jsonwebtoken::decode() with validation
//   - Validate expiration
//   - Return Claims
//
// Claims struct: sub (String), exp (usize), iat (usize)
