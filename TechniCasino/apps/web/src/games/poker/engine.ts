// ── Poker Game Engine (Texas Hold'em) ──
// Most complex game. Build this LAST.
//
// Types:
// PokerHand     → 'royal_flush' | 'straight_flush' | 'four_kind' | ... | 'high_card'
// PokerAction   → 'fold' | 'check' | 'call' | 'raise' | 'all_in'
// PokerRound    → 'preflop' | 'flop' | 'turn' | 'river' | 'showdown'
// PlayerSeat    → { userId, username, chips, cards, folded, currentBet, isDealer, isTurn }
// PokerState    → { seats: PlayerSeat[], communityCards, pot, round, currentBet, minRaise }
//
// Functions:
// evaluateHand(playerCards[2], communityCards[5]) → { rank: PokerHand, value: number }
// compareHands(hand1, hand2)                      → -1 | 0 | 1
// getAvailableActions(state, playerId)             → PokerAction[]
// calculatePotOdds(pot, callAmount)                → percentage
//
// This uses Socket.IO for real-time multiplayer.
// Players join a "table" (Socket.IO room).
// Server manages turns, dealing, pot, showdown.
// Client only renders state and sends actions.
