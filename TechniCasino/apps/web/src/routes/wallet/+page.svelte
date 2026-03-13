<!-- ── Wallet Page ──

  Sections:
  1. Balance card (large, centered, neon-gold)
     - Current balance with animated counter
     - "Deposit" + "Withdraw" buttons

  2. Deposit form (DepositForm component)
     - Amount input + presets
     - (For now: fake deposits, just add to balance via API)
     - (Later: crypto payment integration)

  3. Withdraw form (WithdrawForm component)
     - Same as deposit but subtracts

  4. Transaction history (TransactionList component)
     - Table: date, type, amount (+/-), description, balance after
     - Color-coded: green for deposits/wins, red for withdrawals/losses
     - Pagination or infinite scroll
     - Filter by type

  Protected route: redirect to login if not authenticated.
-->
