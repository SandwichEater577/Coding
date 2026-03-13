// ── Room Management ──
// Handles joining/leaving game rooms (especially Roulette + Poker tables).
//
// Socket events to handle:
//
// 'join_table' → { gameType, tableId }
//   - socket.join(`${gameType}:${tableId}`)
//   - Broadcast to room: 'player_joined' with username
//   - For poker: track seat assignments
//
// 'leave_table' → { gameType, tableId }
//   - socket.leave(`${gameType}:${tableId}`)
//   - Broadcast: 'player_left'
//
// 'roulette_bet' → { tableId, bets }
//   - Forward bet to Rust API: POST /api/games/roulette/bet
//   - Broadcast updated bet display to room
//
// 'roulette_spin' → triggered by timer
//   - Call Rust API: POST /api/games/roulette/spin
//   - Broadcast result to entire room: 'spin_result'
//
// For Roulette tables: run a server-side timer loop
//   30s betting phase → spin → 10s result display → repeat
//   Use setInterval or setTimeout, broadcast timer ticks to room
