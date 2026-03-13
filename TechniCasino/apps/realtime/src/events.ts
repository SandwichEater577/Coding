// ── Event Broadcasting Helpers ──
// Utility functions for the Rust API to trigger broadcasts.
//
// The Rust API can publish events to Redis pub/sub.
// This service subscribes to Redis channels and broadcasts via Socket.IO.
//
// Redis channels:
//   'balance_update:{userId}'  → emit to specific user socket
//   'game_update:{sessionId}'  → emit to game room
//   'global_announcement'      → emit to all connected users
//
// This is how the Rust API communicates with Socket.IO:
//   1. Rust API publishes to Redis: PUBLISH balance_update:uuid123 '{"balance": 5000}'
//   2. This service receives the message from Redis subscriber
//   3. Finds the user's socket by userId
//   4. Emits 'balance_update' event to that socket
//
// This decouples the Rust API from Socket.IO entirely.
// Rust doesn't need to know about WebSockets at all.
