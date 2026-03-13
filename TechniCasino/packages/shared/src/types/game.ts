// ── Game Types ──
// Define and export:
//
// GameType        → enum: BLACKJACK, ROULETTE, SLOTS, POKER, MINES, CHICKEN_ROAD
// GameInfo        → id, name, type (GameType), description, minBet, maxBet, imageUrl, isMultiplayer
// GameSession     → id, userId, gameId, state (GameState), bet, result, createdAt
// GameState       → enum: WAITING, IN_PROGRESS, COMPLETED
// GameResult      → sessionId, outcome (WIN/LOSE/PUSH), payout, multiplier, provablyFairData
// Bet             → id, sessionId, amount, choice (game-specific data as JSON)
//
// ProvablyFairData → serverSeedHash, clientSeed, nonce, serverSeed (revealed after game)
//
// Each game will have its own specific types in its own engine.ts files
