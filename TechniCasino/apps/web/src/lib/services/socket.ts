// ── Socket.IO Client Service ──
// Initialize and export the Socket.IO client instance.
//
// import { io } from 'socket.io-client'
//
// Create connection to PUBLIC_WS_URL with:
// - autoConnect: false (connect manually after login)
// - auth: { token } (JWT for authentication)
// - reconnection: true
// - reconnectionDelay: 1000
// - reconnectionAttempts: 5
//
// This file creates the raw socket instance.
// The socket store (stores/socket.ts) wraps it with Svelte reactivity.
