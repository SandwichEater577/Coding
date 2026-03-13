<!-- ── Blackjack Game Page ──

  Layout:
  - Top: Dealer's hand (one card face-down initially)
  - Center: Game status / result display
  - Bottom: Player's hand
  - Below: Action buttons (Hit, Stand, Double Down, Split)
  - Side: BetControls (before game), game history

  Game flow:
  1. Place bet → server deals 2 cards to player, 2 to dealer (1 face-down)
  2. Player actions: Hit (get card), Stand (end turn), Double Down, Split (if pair)
  3. Dealer reveals + plays (hits until 17+)
  4. Compare hands → Win / Lose / Push
  5. Blackjack (Ace + 10-value) pays 3:2

  Animations:
  - Card deal: slide from deck position with GSAP
  - Card flip: 3D rotateY for dealer reveal
  - Win: gold glow + balance count-up
  - Bust: red shake + cards fade

  All game logic runs on server. Frontend only displays state.
  Each action is a POST request, server returns new state.
-->
