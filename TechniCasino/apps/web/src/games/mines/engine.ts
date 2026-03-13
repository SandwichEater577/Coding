// ── Mines Game Engine ──
// Client-side game state management for Mines.
// ALL randomness comes from server. This just manages UI state.
//
// Types:
// MinesTile    → { x, y, revealed: boolean, isMine: boolean | null, isGem: boolean | null }
// MinesConfig  → { gridSize: 5, mineCount: number (1-24), betAmount: number }
// MinesState   → { grid: MinesTile[][], mineCount, revealedCount, multiplier, status, cashoutAvailable }
// MinesStatus  → 'idle' | 'betting' | 'playing' | 'won' | 'lost'
//
// Functions:
// createGrid(size)                → initialize empty 5x5 grid
// calculateMultiplier(mines, revealed) → math formula for current multiplier
//    Formula: (25! / (25-revealed)!) / ((25-mines)! / (25-mines-revealed)!) * (1 - houseEdge)
// revealTile(x, y, serverResponse) → update grid with server result
// cashOut()                        → lock multiplier, end game
// getMultiplierTable(mines)       → precalculate multipliers for 1-24 reveals
//
// The multiplier increases exponentially as you reveal more safe tiles
// with more mines on the board.
