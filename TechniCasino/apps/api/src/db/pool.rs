// ── db/pool.rs — Database Connection Pool ──
//
// pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error>
//
// Configure pool with:
//   max_connections: 20
//   min_connections: 5
//   acquire_timeout: 30 seconds
//
// Run a test query (SELECT 1) to verify connection on startup.
// Log: "Connected to PostgreSQL"
