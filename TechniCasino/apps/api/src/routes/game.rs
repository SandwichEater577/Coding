// ── routes/game.rs — Game Routes ──
// ALL routes require auth middleware.
//
// POST /api/games/:game_type/start
//   - Extract: game_type from path, Json<{ bet_amount, config }> from body
//   - Validate bet amount within game limits
//   - Validate player has sufficient balance
//   - Generate server_seed (random 64 hex chars)
//   - Hash server_seed → server_seed_hash (shown to player as commitment)
//   - Use client_seed from request (or generate default)
//   - Debit bet_amount from wallet (via wallet service)
//   - Create game_session in DB with state = initial game state
//   - Return: { session_id, server_seed_hash, initial_state }
//
// POST /api/games/:game_type/action
//   - Extract: Json<{ session_id, action, data }>
//   - Load session from DB
//   - Validate action is legal for current state
//   - Call game engine to compute new state
//   - Update session in DB
//   - If game ended: calculate payout, credit wallet, reveal server_seed
//   - Return: { new_state, result? }
//
// POST /api/games/:game_type/cashout
//   - For Mines / Chicken Road: lock current multiplier
//   - Calculate payout = bet_amount * multiplier
//   - Credit wallet
//   - Mark session as completed (WIN)
//   - Reveal server_seed
//
// GET /api/games/history
//   - Query completed game_sessions for user
//   - Pagination: ?page=1&limit=20
//   - Return: Vec<GameSession>
