<!-- ── Mines Game Page ──
  BUILD THIS GAME FIRST — it's the simplest.

  Layout:
  - Left: BetControls + mine count selector (1-24 mines in 5x5 grid)
  - Center: 5x5 MineGrid
  - Right: Multiplier display, cashout button, game history

  Game flow:
  1. Player sets bet amount + number of mines
  2. Player clicks "Start Game" → sends to server
  3. Server generates mine positions (provably fair)
  4. Player clicks tiles one by one:
     - Safe tile → reveal gem, multiplier increases
     - Mine tile → BOOM, lose bet, reveal all mines
  5. Player can "Cash Out" at any time to lock in current multiplier

  Animations:
  - Tile hover: subtle lift + glow
  - Safe reveal: scale up + green glow + gem icon
  - Mine reveal: shake + red flash + explosion effect
  - Cashout: gold confetti / particles

  This game uses REST API (not WebSocket) since it's single-player.
  Server validates every tile click (prevents cheating).
-->
