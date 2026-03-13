# TechniCasino API Documentation

Document every endpoint:

## Auth

- POST /api/auth/register
- POST /api/auth/login
- POST /api/auth/me (get current user from token)

## User

- GET /api/user/profile
- PUT /api/user/profile
- GET /api/user/stats

## Wallet

- GET /api/wallet (get balance)
- POST /api/wallet/deposit
- POST /api/wallet/withdraw
- GET /api/wallet/transactions

## Games

- GET /api/games (list all games)
- POST /api/games/:type/start (start a session)
- POST /api/games/:type/action (game action: hit, stand, reveal tile, etc.)
- POST /api/games/:type/cashout (cash out current game)
- GET /api/games/history (user's game history)

For each: method, path, request body, response body, auth required?
