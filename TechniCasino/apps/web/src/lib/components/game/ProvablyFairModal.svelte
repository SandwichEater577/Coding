<!-- ── Provably Fair Verification Modal ──
  Shows provably fair data for a completed game.

  Props:
  - data: ProvablyFairData
  - isOpen: boolean

  Content:
  - Server Seed Hash (shown BEFORE game, as commitment)
  - Client Seed (player can customize this)
  - Nonce (auto-incrementing counter)
  - Server Seed (revealed AFTER game)
  - "Verify" button → recalculate hash client-side, show match ✓ or mismatch ✗
  - Explanation text: how provably fair works

  Use monospace font for hashes.
  Green checkmark icon when verified.
-->
