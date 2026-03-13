// ── games/blackjack.rs — Blackjack Game Logic ──
//
// Card representation: u8 (0-51)
//   suit = card / 13  (0=Hearts, 1=Diamonds, 2=Clubs, 3=Spades)
//   rank = card % 13  (0=Ace, 1=2, ..., 9=10, 10=Jack, 11=Queen, 12=King)
//
// State (JSONB):
//   deck: Vec<u8>           (shuffled deck from provably fair)
//   deck_pos: usize         (index of next card to deal)
//   player_hands: Vec<Vec<u8>>  (support split)
//   dealer_hand: Vec<u8>
//   current_hand: usize
//   status: "betting" | "playing" | "dealer_turn" | "resolved"
//
// pub fn init_game(server_seed, client_seed, nonce) -> BlackjackState
//   - Shuffle deck using provably_fair::result_to_card_deck()
//   - Deal 2 cards to player, 2 to dealer (one face down)
//   - Check for natural blackjack (Ace + 10-value = instant 3:2 payout)
//
// pub fn card_value(card: u8) -> Vec<u8>
//   - 2-10 → face value, J/Q/K → 10, Ace → [1, 11]
//
// pub fn hand_value(cards: &[u8]) -> u8
//   - Sum all cards, use Ace as 11 unless it busts, then use 1
//   - Return best value ≤ 21, or bust value if impossible
//
// pub fn handle_action(state: &mut BlackjackState, action: Action) -> ActionResult
//   - Hit: deal next card, check bust
//   - Stand: move to next hand or dealer turn
//   - Double: double bet, deal one card, stand
//   - Split: split pair into two hands (if first two cards same rank)
//
// pub fn dealer_play(state: &mut BlackjackState)
//   - Dealer hits until hand_value >= 17
//   - Dealer stands on soft 17 (Ace + 6)
//
// pub fn resolve(state: &BlackjackState) -> Vec<HandResult>
//   - Compare each player hand vs dealer
//   - Blackjack pays 3:2, regular win pays 1:1, push returns bet
