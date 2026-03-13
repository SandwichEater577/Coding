// ── User Types ──
// Define and export these TypeScript interfaces:
//
// User           → id (string UUID), username, email, avatarUrl, createdAt
// CreateUserDTO  → username, email, password (what the register form sends)
// LoginDTO       → email, password (what the login form sends)
// AuthResponse   → token (JWT string), user (User object without password)
// UserProfile    → public user info: id, username, avatarUrl, stats
// UserStats      → totalGames, totalWins, totalLosses, winRate, biggestWin
