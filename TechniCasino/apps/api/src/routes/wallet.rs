// ── routes/wallet.rs — Wallet Routes ──
// ALL routes require auth middleware.
//
// GET /api/wallet
//   - Get wallet for authenticated user
//   - Return: { balance_cents, balance_display }
//
// POST /api/wallet/deposit
//   - Extract: Json<DepositDTO>
//   - Validate amount > 0
//   - BEGIN transaction:
//     1. INSERT INTO transactions (amount = +deposit, type = DEPOSIT)
//     2. UPDATE wallets SET balance = balance + amount
//     3. Return new balance
//   - COMMIT (or ROLLBACK on error)
//
// POST /api/wallet/withdraw
//   - Same as deposit but:
//   - Check balance >= amount → InsufficientFunds error
//   - amount is stored as NEGATIVE in transactions
//
// GET /api/wallet/transactions
//   - Query transactions for user's wallet
//   - Support pagination: ?page=1&limit=20
//   - Return: Vec<Transaction>
