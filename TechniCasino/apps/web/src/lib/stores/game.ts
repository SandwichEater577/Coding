// ── Game Store ──
// Svelte writable store for active game state.
//
// State: { activeGame: GameType | null, session: GameSession | null, history: GameResult[] }
//
// Export functions:
// startGame(gameType, betAmount)  → POST /api/games/:type/play
// endGame()                       → clear active session
// fetchHistory()                  → GET /api/games/history
//
// Each game page will use this store + its own local state.
// The game-specific logic lives in src/games/[gameName]/engine.ts
