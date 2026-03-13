// ── Wallet Validators ──
// Export validation functions:
//
// validateAmount(amount: number)  → { valid: boolean, error?: string }
//   - Must be positive number
//   - Max 2 decimal places
//   - Within min/max limits
//
// validateBet(amount: number, minBet: number, maxBet: number, balance: number)
//   - Amount within game limits
//   - Player has sufficient balance
