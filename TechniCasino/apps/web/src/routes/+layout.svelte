<!-- ── Root Layout ──
  Wraps all pages. Always rendered.

  Structure:
  1. Import and apply global CSS (app.css + neon.css)
  2. Render <Header /> at top
  3. Render <slot /> (page content) in <main>
  4. Render <Footer /> at bottom
  5. Render <ToastContainer /> (floating, fixed position)

  On mount:
  - Check auth state (validate existing token)
  - If authenticated, connect Socket.IO
  - Fetch wallet balance

  On destroy:
  - Disconnect Socket.IO
-->
