// ── Auth Store ──
// Svelte writable store for authentication state.
//
// State shape: { user: User | null, token: string | null, isLoading: boolean }
//
// Export functions:
// login(email, password)   → call auth service, set token + user, save token to cookie
// register(...)            → call auth service, redirect to login
// logout()                 → clear token + user, remove cookie, redirect to home
// checkAuth()              → validate existing token, refresh user data
//
// Use SvelteKit's cookies (not localStorage!) for token storage.
// This makes SSR work and is more secure.
//
// Export derived stores:
// isAuthenticated → boolean derived from user !== null
// currentUser     → User | null
