// ── games/roulette.rs — Roulette Game Logic ──
// This game is MULTIPLAYER — uses the Node.js Socket.IO gateway.
//
// European roulette: 0-36 (single zero).
//
// The Rust API handles:
// - Accepting bets during betting phase
// - Generating the spin result (provably fair)
// - Resolving all bets and calculating payouts
//
// The Node.js realtime gateway handles:
// - Timer countdown for betting phase
// - Broadcasting spin result to all players at the table
// - Real-time bet display
//
// Bet types and payouts:
//   Straight (single number): 35:1
//   Split (2 numbers): 17:1
//   Street (3 numbers): 11:1
//   Corner (4 numbers): 8:1
//   Red/Black: 1:1
//   Odd/Even: 1:1
//   High(19-36)/Low(1-18): 1:1
//   Dozen (1-12, 13-24, 25-36): 2:1
//   Column: 2:1
//
// pub fn spin(server_seed, client_seed, nonce) -> u8
//   - provably_fair::result_to_roulette_number()
//
// pub fn resolve_bets(bets: Vec<RouletteBet>, result: u8) -> Vec<BetResult>
//   - For each bet, check if it covers the result number
//   - Calculate payout based on bet type multiplier
//
// pub fn number_color(n: u8) -> Color { Red | Black | Green }
//   - 0 = Green
//   - Standard European distribution for red/black
