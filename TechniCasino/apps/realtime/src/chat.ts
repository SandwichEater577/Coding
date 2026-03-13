// ── Chat Handler ──
// Global chat + per-table chat.
//
// 'chat_send' → { message, room? }
//   - Sanitize message (strip HTML, limit length to 200 chars)
//   - Rate limit: max 5 messages per 10 seconds per user
//   - Broadcast to room (or 'global' if no room specified):
//     'chat_message' → { userId, username, message, timestamp }
//
// 'chat_history' → { room }
//   - Return last 50 messages from Redis list
//   - Store chat messages in Redis with TTL (expire after 1 hour)
