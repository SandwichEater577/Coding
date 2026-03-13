// ── main.rs — Server Entry Point ──
//
// 1. Initialize tracing subscriber (tracing_subscriber::fmt::init)
// 2. Load .env file (dotenvy::dotenv().ok())
// 3. Parse Config from environment
// 4. Create PostgreSQL connection pool (sqlx::PgPool)
// 5. Create Redis connection
// 6. Build AppState (pool, redis, config)
// 7. Build Axum router:
//    - Merge all route modules (auth, user, wallet, game)
//    - Add CORS layer (tower_http::cors)
//    - Add tracing layer
//    - Pass AppState via .with_state()
// 8. Bind to 0.0.0.0:{PORT} and serve
// 9. Log: "TechniCasino API running on port {PORT}"
//
// Graceful shutdown: tokio::signal::ctrl_c()
