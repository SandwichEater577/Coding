// ── services/wallet.rs — Wallet Business Logic ──
//
// ALL money operations use sqlx transactions for atomicity.
//
// pub async fn get_balance(pool: &PgPool, user_id: Uuid) -> Result<i64, AppError>
//
// pub async fn deposit(pool: &PgPool, user_id: Uuid, amount: i64) -> Result<i64, AppError>
//   - amount must be > 0
//   - BEGIN
//   - SELECT balance FROM wallets WHERE user_id = $1 FOR UPDATE (row lock!)
//   - new_balance = balance + amount
//   - INSERT INTO transactions (wallet_id, amount, tx_type='DEPOSIT', balance_after=new_balance)
//   - UPDATE wallets SET balance = new_balance
//   - COMMIT
//   - Return new_balance
//
// pub async fn place_bet(pool: &PgPool, user_id: Uuid, amount: i64) -> Result<i64, AppError>
//   - Same as withdraw but tx_type = 'BET_PLACED'
//   - Check balance >= amount → InsufficientFunds
//   - SELECT ... FOR UPDATE is critical to prevent race conditions!
//     (Two simultaneous bets can't both read the same balance)
//
// pub async fn credit_win(pool: &PgPool, user_id: Uuid, amount: i64) -> Result<i64, AppError>
//   - Credit winnings, tx_type = 'BET_WON'
//
// The FOR UPDATE lock is essential in a casino:
// Without it, if player has $100 and places two $80 bets simultaneously,
// both read balance=$100, both pass the check, player bets $160 with only $100.
// FOR UPDATE locks the row so the second query waits for the first to commit.
