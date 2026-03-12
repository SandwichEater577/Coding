// ── models/wallet.rs — Wallet data models ──
//
// Structs:
//   Wallet         → user wallet (id, user_id, balance)
//   Transaction    → money movement (id, wallet_id, amount, tx_type, created_at)
//   DepositReq     → deposit request payload
//   WithdrawReq    → withdraw request payload
//
// Enums:
//   TransactionType → Deposit, Withdrawal, BetPlaced, BetWon
