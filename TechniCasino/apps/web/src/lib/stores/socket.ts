// ── Socket.IO Store ──
// Manages the Socket.IO client connection.
//
// Export functions:
// connect(token)        → establish Socket.IO connection with JWT auth
// disconnect()          → close connection
// joinRoom(roomName)    → join a game room or chat room
// leaveRoom(roomName)   → leave a room
// emit(event, data)     → send event to server
// on(event, callback)   → listen for server events
//
// Auto-reconnect on disconnect.
// Attach JWT token as auth handshake.
//
// Events to handle:
// 'balance_update'  → update wallet store
// 'game_update'     → update game store
// 'chat_message'    → display in chat
// 'error'           → show toast
