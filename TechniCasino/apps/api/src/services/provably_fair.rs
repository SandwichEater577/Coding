// ── services/provably_fair.rs — Provably Fair Algorithm ──
//
// Uses: sha2, hmac, hex, rand crates. NO external randomness deps.
//
// pub fn generate_server_seed() -> String
//   → Generate 32 random bytes with rand::thread_rng()
//   → Encode as 64-char hex string
//   → NEVER use rand for game outcomes — only for seed generation
//
// pub fn hash_seed(seed: &str) -> String
//   → SHA-256 hash of seed → hex string
//   → This hash is shown to player BEFORE the game as a commitment
//
// pub fn generate_result(server_seed: &str, client_seed: &str, nonce: u32) -> Vec<u8>
//   → HMAC-SHA256(key=server_seed, message="{client_seed}:{nonce}")
//   → Return raw hash bytes (32 bytes)
//
// pub fn result_to_float(hash: &[u8]) -> f64
//   → Take first 4 bytes of hash
//   → Convert to u32
//   → Divide by u32::MAX → gives 0.0 to 1.0
//
// pub fn result_to_mines_grid(hash: &[u8], grid_size: usize, mine_count: usize) -> Vec<(usize, usize)>
//   → Use hash bytes to deterministically place mines via Fisher-Yates
//   → Returns list of (x, y) mine positions
//
// pub fn result_to_card_deck(hash: &[u8]) -> Vec<u8>
//   → Deterministic Fisher-Yates shuffle of 0..51 using hash as seed
//   → Each index maps to a card (0-12 = A-K of hearts, 13-25 = diamonds, etc.)
//
// pub fn result_to_roulette_number(hash: &[u8]) -> u8
//   → Map to 0..=36 (European roulette)
//
// pub fn result_to_slot_reels(hash: &[u8]) -> [u8; 3]
//   → Map to 3 reel positions using weighted probability table
//
// CRITICAL: NEVER use rand::thread_rng() for game outcomes.
// Game outcomes MUST be deterministic from (server_seed, client_seed, nonce)
// so players can verify fairness after the game.
