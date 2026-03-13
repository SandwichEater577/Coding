// ── games/mines.rs — Mines Game Logic ──
// BUILD THIS GAME FIRST — it's the simplest.
//
// State (stored as JSONB in game_session.state):
//   mine_positions: Vec<(usize, usize)>  (hidden from player until game ends)
//   revealed: Vec<(usize, usize)>        (tiles player has clicked)
//   grid_size: usize                     (always 5 for 5x5)
//   mine_count: usize                    (1-24, chosen by player)
//   status: "playing" | "won" | "lost"
//
// pub fn init_game(server_seed: &str, client_seed: &str, nonce: u32, mine_count: usize) -> MinesState
//   - Use provably_fair::result_to_mines_grid() to place mines
//   - Return initial state with empty revealed list
//
// pub fn reveal_tile(state: &mut MinesState, x: usize, y: usize) -> RevealResult
//   - If (x,y) is in mine_positions → BOOM, status = "lost", return all mine positions
//   - If (x,y) is safe → add to revealed, calculate new multiplier
//   - If all safe tiles revealed → auto-win
//
// pub fn calculate_multiplier(mine_count: usize, revealed_count: usize) -> f64
//   - Formula: product of (remaining_tiles / safe_remaining) for each reveal
//   - With mine_count mines in 25 tiles, after revealing k safe tiles:
//     multiplier = (25! / (25-k)!) / ((25-mine_count)! / (25-mine_count-k)!) * (1 - house_edge)
//   - This increases exponentially as you reveal more tiles
//
// pub fn cashout(state: &MinesState, bet_amount: i64) -> i64
//   - payout = (bet_amount as f64 * multiplier) as i64
//   - Return payout in cents
