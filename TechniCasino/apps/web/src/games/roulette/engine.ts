// ── Roulette Game Engine ──
//
// Types:
// BetType       → 'straight' | 'split' | 'street' | 'corner' | 'red' | 'black' | 'odd' | 'even' | 'high' | 'low' | 'dozen' | 'column'
// RouletteBet   → { type: BetType, numbers: number[], amount: number }
// RouletteState → { bets: RouletteBet[], spinning: boolean, result: number | null, totalBet, totalWin }
//
// Numbers: 0-36 (European roulette, single zero)
// Colors: 0=green, then alternating red/black (standard distribution)
//
// Functions:
// getNumberColor(n: number)    → 'red' | 'black' | 'green'
// calculatePayout(bet, result) → number (0 if lost)
// getBetMultiplier(betType)    → straight=35, split=17, red/black=1, dozen=2, etc.
// isWinningBet(bet, result)    → boolean
//
// This game is MULTIPLAYER (shared table via Socket.IO):
// - Betting phase (timer countdown)
// - Spin phase (all players watch same spin)
// - Result phase (each player's bets resolved individually)
