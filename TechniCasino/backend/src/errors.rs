// ── errors.rs — Application error types ──
// Central AppError enum implementing IntoResponse.
//
// Variants:
//   NotFound
//   Unauthorized
//   BadRequest(String)
//   InternalError(String)
//   DatabaseError(sqlx::Error)
//   ValidationError(String)
