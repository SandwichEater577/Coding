// ── Realtime Gateway Entry Point ──
// ~50 lines total. This is intentionally tiny.
//
// 1. Create HTTP server
// 2. Attach Socket.IO with:
//    - cors: allow frontend URL
//    - adapter: Redis adapter (for scaling)
// 3. On connection:
//    a. Verify JWT from socket.handshake.auth.token
//       (decode with jsonwebtoken, same secret as Rust API)
//    b. If invalid → disconnect
//    c. Store userId on socket instance
//    d. Auto-join 'global' room
//    e. Import and attach room handlers, chat handlers, event handlers
// 4. Listen on PORT (default 3002)
// 5. Log: "TechniCasino realtime gateway on port {PORT}"
