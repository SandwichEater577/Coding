// ── routes/mod.rs — Route Aggregator ──
//
// pub mod auth;
// pub mod user;
// pub mod wallet;
// pub mod game;
//
// pub fn router(state: AppState) -> Router {
//     Router::new()
//         .nest("/api/auth", auth::router())
//         .nest("/api/user", user::router())
//         .nest("/api/wallet", wallet::router())
//         .nest("/api/games", game::router())
//         .with_state(state)
// }
//
// Each sub-module exports a router() function that returns Router<AppState>.
