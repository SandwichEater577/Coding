## Phase 1: Database (foundation)

- **001_init.sql** — Write the actual CREATE TABLE statements. Everything else depends on this schema.

## Phase 2: Backend core (Rust)

- ⚙️ **config.rs** — Load .env vars
- ⚙️ **pool.rs** — DB connection pool
- ⚙️ **hash.rs** — Password hashing
- ⚙️ **jwt.rs** — Token encode/decode
- ⚙️ **errors.rs** — AppError enum
- ⚙️ **user.rs** — User structs
- ⚙️ **wallet.rs** — Wallet structs
- ⚙️ **game.rs** — Game structs

## Phase 3: Backend routes (API)

- ⚙️ **auth.rs** — JWT guard, Register + Login
- ⚙️ **user.rs** — Profile
- ⚙️ **wallet.rs** — Balance, deposit, withdraw
- ⚙️ **game.rs** — Game list, sessions
- ⚙️ **mod.rs** — Wire routes together
- ⚙️ **main.rs** — Boot the server

## Phase 4: Frontend types & services

- 📘 **user.ts** — Mirror backend User types
- 📘 **wallet.ts** — Mirror Wallet types
- 📘 **game.ts** — Mirror Game types
- 📘 **dom.ts** — DOM helpers
- 📘 **validators.ts** — Input validation
- 📘 **formatters.ts** — Currency/date formatting
- 📘 **constants.ts** — Route paths, storage keys
- 📘 **api.ts** — Base fetch client
- 📘 **auth.ts** — Login/register calls
- 📘 **websocket.ts** — WS client

## Phase 5: Frontend UI

- 🎨 **variables.css** — Design tokens
- 🎨 **global.css** — Reset + base styles
- 🔘 **button.ts** — Reusable button
- 🔘 **modal.ts** — Modal dialog
- 🔘 **toast.ts** — Notifications
- 🔘 **header.ts** — Navbar
- 🔘 **footer.ts** — Footer
- 🔘 **gamecard.ts** — Game tile
- 🔘 **router.ts** — SPA router
- 🔘 **login.ts** — Login form
- 🔘 **register.ts** — Register form
- 🔘 **home.ts** — Landing page
- 🔘 **lobby.ts** — Game lobby
- 🔘 **wallet.ts** — Wallet page
- 🔘 **profile.ts** — Profile page
- 🔘 **main.ts** — Wire router + render

## Phase 6: Games (last — most complex)

- 🎰 **slots.ts** — Easiest game
- 🎰 **blackjack.ts**
- 🎰 **roulette.ts**
- 🎰 **poker.ts** — Hardest (multiplayer)
- ⚙️ **session.rs** — WS game sessions
- ⚙️ **lobby.rs** — Multiplayer rooms

## Phase 1: Database (foundation)

- **001_init.sql** — Write the actual CREATE TABLE statements. Everything else depends on this schema.

## Phase 2: Backend core (Rust)

- 🦀 **config.rs** — Load .env vars
- 🦀 **pool.rs** — DB connection pool
- 🦀 **hash.rs** — Password hashing
- 🦀 **jwt.rs** — Token encode/decode
- 🦀 **errors.rs** — AppError enum
- 🦀 **user.rs** — User structs
- 🦀 **wallet.rs** — Wallet structs
- 🦀 **game.rs** — Game structs

## Phase 3: Backend routes (API)

- 🦀 **auth.rs** — JWT guard, Register + Login
- 🦀 **user.rs** — Profile
- 🦀 **wallet.rs** — Balance, deposit, withdraw
- 🦀 **game.rs** — Game list, sessions
- 🦀 **mod.rs** — Wire routes together
- 🦀 **main.rs** — Boot the server

## Phase 4: Frontend types & services

- 🔷 **user.ts** — Mirror backend User types
- 🔷 **wallet.ts** — Mirror Wallet types
- 🔷 **game.ts** — Mirror Game types
- 🔷 **dom.ts** — DOM helpers
- 🔷 **validators.ts** — Input validation
- 🔷 **formatters.ts** — Currency/date formatting
- 🔷 **constants.ts** — Route paths, storage keys
- 🔷 **api.ts** — Base fetch client
- 🔷 **auth.ts** — Login/register calls
- 🔷 **websocket.ts** — WS client

## Phase 5: Frontend UI

- 🎨 **variables.css** — Design tokens
- 🎨 **global.css** — Reset + base styles
- 🟠 **button.ts** — Reusable button
- 🟠 **modal.ts** — Modal dialog
- 🟠 **toast.ts** — Notifications
- 🟠 **header.ts** — Navbar
- 🟠 **footer.ts** — Footer
- 🟠 **gamecard.ts** — Game tile
- 🟠 **router.ts** — SPA router
- 🟠 **login.ts** — Login form
- 🟠 **register.ts** — Register form
- 🟠 **home.ts** — Landing page
- 🟠 **lobby.ts** — Game lobby
- 🟠 **wallet.ts** — Wallet page
- 🟠 **profile.ts** — Profile page
- 🟠 **main.ts** — Wire router + render

## Phase 6: Games (last — most complex)

- 🎮 **slots.ts** — Easiest game
- 🎮 **blackjack.ts**
- 🎮 **roulette.ts**
- 🎮 **poker.ts** — Hardest (multiplayer)
- 🦀 **session.rs** — WS game sessions
- 🦀 **lobby.rs** — Multiplayer rooms
