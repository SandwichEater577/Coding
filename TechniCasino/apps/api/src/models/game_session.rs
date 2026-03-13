// ── models/game_session.rs — Game Session Models ──
//
// GameSession     → id, user_id, game_type, bet_amount, multiplier (Option<f64>),
//                   payout, outcome (Option<GameOutcome>), state (serde_json::Value),
//                   server_seed, server_seed_hash, client_seed, nonce, created_at, completed_at
//
// GameOutcome     → enum { Win, Lose, Push }
// GameType        → enum { Blackjack, Roulette, Slots, Poker, Mines, ChickenRoad }
//
// Both enums: derive Serialize, Deserialize, sqlx::Type
// Use #[sqlx(type_name = "varchar")] and #[sqlx(rename_all = "UPPERCASE")]
//
// The `state` field is JSONB — stores game-specific state:
//   Mines: { grid, revealed_tiles, mine_positions }
//   Blackjack: { player_hand, dealer_hand, deck_position }
//   etc.
