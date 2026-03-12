// ── models/game.rs — Game data models ──
//
// Structs:
//   Game           → game definition (id, name, game_type, min_bet, max_bet)
//   GameSession    → active game session (id, user_id, game_id, state, created_at)
//   GameResult     → completed game result (session_id, outcome, payout)
//   Bet            → individual bet (id, session_id, amount, choice)
//
// Enums:
//   GameType       → Blackjack, Roulette, Slots, Poker
//   GameOutcome    → Win, Lose, Push
