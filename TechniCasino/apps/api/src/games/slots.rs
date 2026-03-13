// ── games/slots.rs — Slot Machine Game Logic ──
//
// Simple 3-reel, 1-payline slot machine.
//
// Symbols (weighted probability):
//   Cherry(🍒)=30%, Lemon(🍋)=25%, Orange(🍊)=20%,
//   Grapes(🍇)=12%, Star(⭐)=8%, Diamond(💎)=3%, Seven(7️⃣)=2%
//
// State (JSONB):
//   reels: [u8; 3]  (index into symbol array)
//   result_symbols: [Symbol; 3]
//
// pub fn spin(server_seed, client_seed, nonce) -> SlotsResult
//   - Use provably_fair::result_to_slot_reels() to determine 3 reel positions
//   - Map positions to symbols using weighted probability table
//   - Check win conditions
//
// Paytable (multipliers):
//   🍒🍒🍒 = 5x     🍋🍋🍋 = 8x     🍊🍊🍊 = 10x
//   🍇🍇🍇 = 15x    ⭐⭐⭐ = 25x    💎💎💎 = 50x
//   7️⃣7️⃣7️⃣ = 100x    Any 2x🍒 = 2x
//
// This is the second easiest game to build. Single REST call:
// POST /start → returns result immediately (no multi-step actions)
