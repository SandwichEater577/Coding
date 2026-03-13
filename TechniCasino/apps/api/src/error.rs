// ── error.rs — Application Error Types ──
//
// Define AppError enum using thiserror:
//
// #[derive(Debug, thiserror::Error)]
// pub enum AppError {
//     NotFound(String),
//     Unauthorized(String),
//     BadRequest(String),
//     Conflict(String),          // e.g. username already taken
//     InsufficientFunds,
//     InternalError(String),
//     DatabaseError(sqlx::Error),
//     ValidationError(String),
// }
//
// Implement IntoResponse for AppError:
// - Each variant maps to an HTTP status code
// - Returns JSON: { "success": false, "error": { "code": "...", "message": "..." } }
//
// NotFound → 404, Unauthorized → 401, BadRequest → 400,
// Conflict → 409, InsufficientFunds → 400, InternalError → 500
//
// In production: don't leak internal error details in 500 responses.
