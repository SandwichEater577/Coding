// ── state.rs — Shared Application State ──
//
// Define AppState struct:
//   pool: sqlx::PgPool
//   redis: redis::Client
//   config: Config
//
// Implement Clone for AppState (required by Axum).
// This gets passed to all route handlers via Axum's State extractor.
//
// Usage in handlers:
//   async fn handler(State(state): State<AppState>) -> Result<Json<...>, AppError>
