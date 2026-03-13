// ── API Client ──
// Base HTTP client wrapping fetch().
//
// Create a function apiClient(endpoint, options?) that:
// 1. Prepends PUBLIC_API_URL to the endpoint
// 2. Adds 'Content-Type: application/json' header
// 3. Adds 'Authorization: Bearer <token>' from auth store
// 4. Handles response: parse JSON, throw on non-2xx status
// 5. Handles network errors gracefully
//
// Export convenience methods:
// api.get(endpoint)
// api.post(endpoint, body)
// api.put(endpoint, body)
// api.delete(endpoint)
//
// All methods return Promise<T> with generic typing.
