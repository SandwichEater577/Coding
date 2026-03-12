// ── main.rs — Server entry point ──

mod config;
mod errors;
mod routes;
mod models;
mod middleware;
mod db;
mod ws;
mod utils;

#[tokio::main]
async fn main() {
    // TODO: Initialize tracing subscriber
    // TODO: Load configuration from .env
    // TODO: Create database connection pool
    // TODO: Build application router (routes, middleware, state)
    // TODO: Bind to address and start serving
}
