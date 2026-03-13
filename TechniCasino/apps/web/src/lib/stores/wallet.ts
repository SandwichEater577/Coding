// ── Wallet Store ──
// Svelte writable store for wallet/balance state.
//
// State: { balance: number, transactions: Transaction[], isLoading: boolean }
//
// Export functions:
// fetchBalance()       → GET /api/wallet
// deposit(amount)      → POST /api/wallet/deposit
// withdraw(amount)     → POST /api/wallet/withdraw
// fetchTransactions()  → GET /api/wallet/transactions
//
// Balance should update in real-time via Socket.IO events
// when bets are placed or won from any game.
//
// Display balance in the Header component at all times.
