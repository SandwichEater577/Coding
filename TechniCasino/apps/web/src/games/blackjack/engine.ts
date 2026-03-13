// ── Blackjack Game Engine ──
// Client-side state for Blackjack. Server handles all logic.
//
// Types:
// Card          → { suit: 'hearts'|'diamonds'|'clubs'|'spades', rank: 'A'|'2'...'K', faceUp: boolean }
// Hand          → { cards: Card[], value: number, isBust: boolean, isBlackjack: boolean }
// BlackjackState → { playerHands: Hand[], dealerHand: Hand, status, bet, currentHandIndex }
// BlackjackStatus → 'betting' | 'playing' | 'dealer_turn' | 'resolved'
// BlackjackAction → 'hit' | 'stand' | 'double' | 'split'
//
// Functions:
// calculateHandValue(cards: Card[])  → number (handle Ace as 1 or 11)
// canSplit(hand: Hand)               → boolean (first 2 cards same rank)
// canDoubleDown(hand: Hand)          → boolean (only on first 2 cards)
// getCardImagePath(card: Card)       → string (for displaying card)
// formatHand(hand: Hand)             → display string like "A♠ K♥ = 21"
//
// Card values: 2-10 face value, J/Q/K = 10, Ace = 1 or 11 (whichever doesn't bust)
