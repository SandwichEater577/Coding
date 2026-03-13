// ── games/chicken_road.rs — Chicken Road Game Logic ──
//
// Grid: 10 rows, each with 3 lanes.
// In each row, exactly 1 lane has a "car" (death), 2 are safe.
// Player picks a lane each row. Safe → advance, Car → lose.
// Can cashout at any row.
//
// State (JSONB):
//   car_positions: Vec<usize>   (which lane has the car in each row, 0-2)
//   current_row: usize          (0 = bottom, 9 = top)
//   status: "playing" | "won" | "lost"
//
// pub fn init_game(server_seed, client_seed, nonce) -> ChickenRoadState
//   - Use provably fair hash to deterministically place 1 car per row
//
// pub fn choose_lane(state: &mut ChickenRoadState, lane: usize) -> LaneResult
//   - If lane == car_positions[current_row] → DEAD, lose bet
//   - If safe → advance current_row, increase multiplier
//   - If current_row reaches top → auto-win at max multiplier
//
// pub fn calculate_multiplier(row: usize) -> f64
//   - Each row survived: multiplier *= (3.0 / 2.0) * (1.0 - house_edge)
//   - Row 0: ~1.47x, Row 5: ~5.3x, Row 9: ~25x (approximate)
