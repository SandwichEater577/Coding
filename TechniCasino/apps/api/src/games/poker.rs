// ── games/poker.rs — Texas Hold'em Poker ──
// BUILD THIS GAME LAST — most complex, multiplayer.
//
// The Rust API handles:
// - Game state management (whose turn, pot size, community cards)
// - Hand evaluation (ranking poker hands)
// - Determining winner at showdown
// - All financial transactions
//
// The Node.js realtime gateway handles:
// - Player joining/leaving tables (Socket.IO rooms)
// - Turn timer
// - Broadcasting state updates to all players
//
// Hand rankings (highest to lowest):
//   RoyalFlush, StraightFlush, FourOfAKind, FullHouse, Flush,
//   Straight, ThreeOfAKind, TwoPair, OnePair, HighCard
//
// pub fn evaluate_hand(hole_cards: [u8; 2], community: &[u8]) -> HandRank
//   - Find best 5-card hand from 7 cards (2 hole + 5 community)
//   - This is the most complex function in the entire project
//   - Consider all C(7,5) = 21 possible 5-card combinations
//
// pub fn compare_hands(a: HandRank, b: HandRank) -> Ordering
//
// Game phases: Preflop → Flop (3 cards) → Turn (1 card) → River (1 card) → Showdown
// Each phase has a betting round.
