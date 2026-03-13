// ── models/wallet.rs — Wallet Data Models ──
//
// Wallet          → id (Uuid), user_id (Uuid), balance (i64, in cents!)
// Transaction     → id, wallet_id, amount (i64), tx_type (TransactionType), description, balance_after, created_at
// DepositDTO      → amount (i64)
// WithdrawDTO     → amount (i64)
// WalletResponse  → balance_display (String, formatted "$10.50"), balance_cents (i64)
//
// TransactionType as enum with sqlx::Type:
//   Deposit, Withdrawal, BetPlaced, BetWon, BetRefund
//
// IMPORTANT: ALL money as i64 cents. $10.50 = 1050_i64.
// Never use f64 for money in Rust. The compiler won't save you from float bugs
// but using i64 by convention eliminates them entirely.
