<!-- ── Bet Controls ──
  Reusable betting interface used by all games.

  Props:
  - minBet: number
  - maxBet: number
  - balance: number (from wallet store)
  - disabled: boolean (during active game)

  Features:
  - Current bet amount display (large, neon-gold)
  - Preset buttons: 0.10, 0.50, 1, 5, 10, 25, 50, 100
  - "Half" button (÷2), "Double" button (×2), "Max" button
  - Manual input field for custom amount
  - Validate: can't bet more than balance or above maxBet
  - "Place Bet" button (primary, large, glowing)

  Dispatch event: on:bet with { amount }
-->
