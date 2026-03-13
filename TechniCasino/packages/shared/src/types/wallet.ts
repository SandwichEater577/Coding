// ── Wallet Types ──
// Define and export:
//
// Wallet           → id, userId, balance (number, in cents to avoid float issues)
// Transaction      → id, walletId, amount, type (TransactionType), description, createdAt
// TransactionType  → enum: DEPOSIT, WITHDRAWAL, BET_PLACED, BET_WON, BET_REFUND
// DepositDTO       → amount
// WithdrawDTO      → amount
// WalletResponse   → balance formatted as string with 2 decimals
//
// IMPORTANT: Store money as integers (cents). $10.50 = 1050.
// This avoids floating point bugs like 0.1 + 0.2 = 0.30000000000000004
