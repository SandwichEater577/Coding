<!-- ── Game Card ──
  Clickable game tile for the lobby page.

  Props:
  - game: GameInfo (from shared/types)

  Features:
  - Card with game thumbnail/icon at top
  - Game name in display font
  - Short description
  - Min/Max bet display
  - "Multiplayer" badge if isMultiplayer
  - Hover: lift up (translateY), increase glow, subtle scale
  - Click: navigate to /games/[game.type]
  - Use Svelte transition for staggered entrance in lobby grid
-->
