// ── Chicken Road Game Engine ──
// Client-side state for Chicken Road (similar to Mines but linear progression).
//
// Types:
// Lane          → { safe: boolean | null, revealed: boolean }
// Row           → { lanes: Lane[3 or 4], multiplier: number }
// ChickenState  → { rows: Row[], currentRow: number, status, betAmount, chickenPosition }
// ChickenStatus → 'idle' | 'playing' | 'won' | 'lost'
//
// Game concept:
// - Grid of rows (e.g. 10 rows), each row has 3-4 lanes
// - In each row, one lane is a "car" (death), others are safe
// - Chicken starts at bottom, player picks a lane each row
// - Safe: chicken moves up, multiplier increases
// - Car: chicken dies, lose bet
// - Player can cash out at any row
//
// Functions:
// calculateMultiplier(row, totalRows, lanesPerRow) → number
// moveChicken(row, laneIndex, serverResponse)      → update state
// cashOut()                                         → lock winnings
