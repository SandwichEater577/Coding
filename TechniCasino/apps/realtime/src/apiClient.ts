// ── Internal API Client ──
// HTTP client for calling the Rust API from the realtime gateway.
//
// const API_URL = process.env.API_URL || 'http://localhost:3001'
//
// export async function apiCall(method, path, body?, token?)
//   - Wraps fetch() with base URL
//   - Forwards the player's JWT token for authentication
//   - Returns parsed JSON response
//
// Used by rooms.ts to forward game actions to the Rust API.
// Example: when a roulette player places a bet via Socket.IO,
// rooms.ts calls apiCall('POST', '/api/games/roulette/bet', betData, playerToken)
// to process it through the Rust game engine.
