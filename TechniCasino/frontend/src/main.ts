// ── main.ts — Application entry point ──
// Imports global styles, initializes router, renders initial page.

import "./styles/global.css";

// Small TypeScript starter example — edit here to build your app.

interface User {
  id: string;
  username: string;
  email?: string;
}

interface AppState {
  user?: User;
  balance?: number;
}

const root = document.getElementById("root")!;

function formatCurrency(n: number) {
  return `$${n.toFixed(2)}`;
}

async function mockGetBalance(): Promise<number> {
  // replace with real fetch to /api/wallet
  return new Promise((res) => setTimeout(() => res(123.45), 300));
}

function render(state: AppState = {}) {
  root.innerHTML = `
		<header><h1>TechniCasino</h1></header>
		<main id="app">
			<p>Welcome${state.user ? ", " + state.user.username : ""}</p>
			<p>Balance: ${formatCurrency(state.balance ?? 0)}</p>
			<div id="controls"></div>
		</main>
	`;

  const controls = document.getElementById("controls")!;
  const btn = document.createElement("button");
  btn.textContent = "Get Balance (demo)";
  btn.addEventListener("click", async () => {
    btn.disabled = true;
    try {
      const balance = await mockGetBalance();
      render({ ...state, balance });
    } finally {
      btn.disabled = false;
    }
  });
  controls.appendChild(btn);
}

render();
