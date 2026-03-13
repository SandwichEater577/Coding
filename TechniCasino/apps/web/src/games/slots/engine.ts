// ── Slots Game Engine ──
// Client-side state for Slot Machine.
//
// Types:
// SlotSymbol     → '🍒' | '🍋' | '🍊' | '🍇' | '⭐' | '💎' | '7️⃣' | '🃏'
// Reel           → { symbols: SlotSymbol[], currentIndex: number, spinning: boolean }
// PayLine        → { positions: [number, number, number], multiplier: number }
// SlotsState     → { reels: Reel[3], spinning: boolean, result: SlotSymbol[3], winAmount, payline }
//
// Functions:
// getReelSymbols()         → weighted array of symbols (💎 rarer than 🍒)
// checkWin(symbols[3])     → { isWin, multiplier, payline }
// getPayTable()            → map of symbol combinations → multipliers
//
// 3 reels, 1 payline (center row).
// Payouts:
//   🍒🍒🍒 = 5x    🍋🍋🍋 = 8x    🍊🍊🍊 = 10x
//   🍇🍇🍇 = 15x   ⭐⭐⭐ = 25x    💎💎💎 = 50x
//   7️⃣7️⃣7️⃣ = 100x   Any 2 🍒 = 2x
//
// Server determines result. Client just animates reels
// to land on the result symbols.
