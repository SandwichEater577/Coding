/* =========
  Constants
========== */

const rootDivTerminal = document.getElementById("terminalOutput");
const missionList = document.getElementById("missionList");
const traceStatus = document.querySelector("#traceStatus");
const dateSpan = document.querySelector("#clockDate");
const timeSpan = document.querySelector("#clockTime");
const ipSpan = document.querySelector("#ipAddress");
const terminal = document.getElementById("terminal");
const inputBar = document.getElementById("inputBar");
const shortcuts = document.getElementById("shortcuts");
const overlay = document.getElementById("press-enter-overlay");
const peText = document.getElementById("pe-text");

/* =========
  State
========== */

let isAnimating = false;
let currentScreen = "screen-boot";
let activeKeys = {};
let timeoutTimer = null;
let lagInterval = null;

/* =========
  Lag levels per mission (0-100)
========== */

const MISSION_MAP = {
  mission1: { screen: "screen-mission1", trace: 0, lag: 0.5 },
  mission2: { screen: "screen-mission2", trace: 12, lag: 8 },
  mission3: { screen: "screen-mission3", trace: 31, lag: 20 },
  mission4: { screen: "screen-mission4", trace: 58, lag: 38 },
  mission5: { screen: "screen-mission5", trace: 74, lag: 52 },
  mission6: { screen: "screen-mission6", trace: 89, lag: 65 },
  mission7: { screen: "screen-mission7", trace: 97, lag: 75 },
};

const SCREEN_KEYS = {
  "screen-boot": {
    Enter: () => showScreen("screen-mission1", 0),
  },
  "screen-mission1": {
    s: () =>
      appendLines("screen-mission1", [
        { text: "> --scan", cls: "lok" },
        { text: "[SCAN] Discovering network nodes...", cls: "ls" },
        { text: "[SCAN] Found: 192.168.1.1 · NEXUS-TRAINING-01", cls: "lok" },
        { text: "[SCAN] Found: 192.168.1.7 · GHOST-RELAY-NODE", cls: "lok" },
        { text: "[OK] Scan complete. Press C to connect.", cls: "lok" },
      ]),
    c: () =>
      appendLines(
        "screen-mission1",
        [
          { text: "> --connect 192.168.1.1", cls: "lok" },
          { text: "[CONNECT] Establishing connection...", cls: "ls" },
          { text: "[OK] Connected to NEXUS-TRAINING-01", cls: "lok" },
          {
            text: "[MISSION COMPLETE] Tutorial finished. Press ENTER for next mission.",
            cls: "lw",
          },
        ],
        () => setScreenKeys({ Enter: () => showScreen("screen-mission2", 12) }),
      ),
    h: () =>
      appendLines("screen-mission1", [
        { text: "> --help", cls: "lok" },
        { text: "S · scan network", cls: "ld" },
        { text: "C · connect to host", cls: "ld" },
        { text: "H · help", cls: "ld" },
      ]),
  },
  "screen-mission2": {
    s: () =>
      appendLines("screen-mission2", [
        { text: "> --scan", cls: "lok" },
        { text: "[SCAN] Mapping NEXUS-FRONT-01 ports...", cls: "ls" },
        { text: "[SCAN] 22/tcp open · ssh", cls: "ld" },
        { text: "[SCAN] 80/tcp open · http  <- vulnerable", cls: "lw" },
        { text: "[SCAN] 3306/tcp filtered · mysql", cls: "ld" },
        {
          text: "[OK] Vulnerability found on port 80. Press E to exploit.",
          cls: "lok",
        },
      ]),
    e: () =>
      appendLines("screen-mission2", [
        { text: "> --exploit 80", cls: "lok" },
        { text: "[EXPLOIT] Loading CVE-2031-4471...", cls: "ls" },
        { text: "[EXPLOIT] Bypassing authentication...", cls: "ls" },
        {
          text: "[OK] Access granted. Press G to grab personnel.db.",
          cls: "lok",
        },
      ]),
    g: () =>
      appendLines(
        "screen-mission2",
        [
          { text: "> --grab /var/db/personnel.db", cls: "lok" },
          { text: "[GRAB] Downloading 847 records...", cls: "ls" },
          { text: "[OK] File received.", cls: "lok" },
          {
            text: "[ANOMALY] 3 records have no identity. Only a photo.",
            cls: "le",
          },
          { text: "[ANOMALY] One photo matches your 2019 profile.", cls: "le" },
          { text: "[MISSION COMPLETE] Press ENTER to continue.", cls: "lw" },
        ],
        () => setScreenKeys({ Enter: () => showScreen("screen-mission3", 31) }),
      ),
  },
  "screen-mission3": {
    q: () =>
      appendLines("screen-mission3", [
        { text: "> --query /db/blackout", cls: "lok" },
        { text: "[QUERY] Searching database...", cls: "ls" },
        { text: "[WARN] Access denied on /db/blackout", cls: "lw" },
        { text: "[FOUND] /db/project_echo.dat · unprotected", cls: "le" },
        {
          text: "[?] That wasn't what we were looking for. Press D to decrypt.",
          cls: "lw",
        },
      ]),
    d: () =>
      appendLines(
        "screen-mission3",
        [
          { text: "> --decrypt /db/project_echo.dat", cls: "lok" },
          { text: "[DECRYPT] Breaking AES-128...", cls: "ls" },
          { text: "[OK] Decrypted.", cls: "lok" },
          {
            text: "[DATA] Project ECHO: 5 year hacker recruitment program",
            cls: "le",
          },
          { text: "[DATA] 23 recruits. 22 disappeared.", cls: "le" },
          { text: "[DATA] Recruit #23: [YOU]", cls: "le" },
          { text: "[PHANTOM] > I can explain. Press ENTER.", cls: "li" },
        ],
        () => setScreenKeys({ Enter: () => showScreen("screen-mission4", 58) }),
      ),
  },
  "screen-mission4": {
    l: () =>
      appendLines(
        "screen-mission4",
        [
          { text: "> --logs /ghost/operations", cls: "lok" },
          {
            text: "[LOGS] Reading 5 years of G.H.O.S.T. activity...",
            cls: "ls",
          },
          {
            text: "[LOG 2029] Op. SIGMA: eliminated NEXUS competitor",
            cls: "lwr",
          },
          {
            text: "[LOG 2030] Op. DELTA: tested NEXUS firewall vulnerabilities",
            cls: "lwr",
          },
          {
            text: "[LOG 2031] Op. CIPHER: verified agent loyalty for NEXUS",
            cls: "lwr",
          },
          {
            text: "[CONCLUSION] Every G.H.O.S.T. operation served NEXUS.",
            cls: "le",
          },
          { text: "[PHANTOM] > ...I know. Press ENTER.", cls: "li" },
        ],
        () => setScreenKeys({ Enter: () => showScreen("screen-mission5", 74) }),
      ),
    t: () =>
      appendLines("screen-mission4", [
        { text: "> --trace unknown_connection", cls: "lok" },
        { text: "[TRACE] Tracing unknown node...", cls: "ls" },
        { text: "[TRACE] Origin: NEXUS CORE-PRIME-01", cls: "le" },
        { text: "[WARN] They know you found this.", cls: "le" },
      ]),
  },
  "screen-mission5": {
    a: () =>
      appendLines(
        "screen-mission5",
        [
          { text: "> --accept", cls: "lok" },
          { text: "[ORACLE] > Good choice. Or not. We'll see.", cls: "lo" },
          {
            text: "[ORACLE] > Sending you the access key to NEXUS core.",
            cls: "lo",
          },
          { text: "[ORACLE] > Key: 0x4E455855532D434F5245", cls: "lo" },
          {
            text: "[OK] Access key received. Press ENTER for next mission.",
            cls: "lok",
          },
        ],
        () => setScreenKeys({ Enter: () => showScreen("screen-mission6", 89) }),
      ),
    r: () =>
      appendLines(
        "screen-mission5",
        [
          { text: "> --reject", cls: "lok" },
          { text: "[ORACLE] > Interesting.", cls: "lo" },
          {
            text: "[ORACLE] > I'll help you anyway. You have no other choice.",
            cls: "lo",
          },
          { text: "[ORACLE] > Key: 0x4E455855532D434F5245", cls: "lo" },
          {
            text: "[OK] Access key received. Press ENTER for next mission.",
            cls: "lok",
          },
        ],
        () => setScreenKeys({ Enter: () => showScreen("screen-mission6", 89) }),
      ),
  },
  "screen-mission6": {
    s: () =>
      appendLines(
        "screen-mission6",
        [
          { text: "> --send", cls: "lok" },
          {
            text: "[SEND] Transmitting to 847 journalists, governments, activists...",
            cls: "ls",
          },
          {
            text: "[WARN] NEXUS is tracing every recipient. 4 minutes.",
            cls: "le",
          },
          {
            text: "[OK] Data sent. The world knows. Press ENTER for the finale.",
            cls: "lok",
          },
        ],
        () => setScreenKeys({ Enter: () => showScreen("screen-mission7", 97) }),
      ),
    a: () =>
      appendLines(
        "screen-mission6",
        [
          { text: "> --abort", cls: "lok" },
          { text: "[ABORT] Transmission cancelled.", cls: "ls" },
          { text: "[SYSTEM] BLACKOUT.EXE will launch in 6 hours.", cls: "le" },
          {
            text: "[PHANTOM] > ...you sure about that? Press ENTER.",
            cls: "li",
          },
        ],
        () => setScreenKeys({ Enter: () => showScreen("screen-mission7", 97) }),
      ),
  },
  "screen-mission7": {
    1: () =>
      appendLines(
        "screen-mission7",
        [
          { text: "> --destroy", cls: "le" },
          {
            text: "[SYSTEM] Initiating core destruction sequence...",
            cls: "le",
          },
          { text: "[SYSTEM] NEXUS CORE-PRIME-01 · OFFLINE", cls: "le" },
          { text: "[SYSTEM] You are still in the building.", cls: "le" },
          { text: "[SYSTEM] Connection lost.", cls: "le" },
          { text: ".", cls: "ls" },
          { text: ". .", cls: "ls" },
          { text: "[END] You destroyed it. And yourself.", cls: "lw" },
        ],
        () => triggerEnding("destroy"),
      ),
    2: () =>
      appendLines(
        "screen-mission7",
        [
          { text: "> --expose", cls: "lok" },
          {
            text: "[EXPOSE] Broadcasting NEXUS data to all public channels...",
            cls: "ls",
          },
          { text: "[OK] 2.3 billion people received the truth.", cls: "lok" },
          {
            text: "[SYSTEM] NEXUS stock dropped 94% in 3 minutes.",
            cls: "lok",
          },
          { text: "[PHANTOM] > ...I didn't expect this.", cls: "li" },
          { text: "[END] The world knows.", cls: "lw" },
        ],
        () => triggerEnding("expose"),
      ),
    3: () =>
      appendLines(
        "screen-mission7",
        [
          { text: "> --join", cls: "ls" },
          { text: "[NEXUS] Welcome. We've been waiting.", cls: "ls" },
          { text: "[NEXUS] You were always one of us.", cls: "ls" },
          { text: "[PHANTOM] > ...", cls: "li" },
          {
            text: "[END] You joined them. BLACKOUT proceeds as planned.",
            cls: "le",
          },
        ],
        () => triggerEnding("join"),
      ),
  },
};

/* =========
  Helpers
========== */

const wait = (seconds) =>
  new Promise((resolve) => setTimeout(resolve, seconds * 1000));

const randomBetween = (min, max) => Math.random() * (max - min) + min;

/* =========
  Clock
========== */

function updateClock() {
  const now = new Date();
  const pad = (n) => String(n).padStart(2, "0");
  dateSpan.textContent = `${now.getFullYear()}.${pad(now.getMonth() + 1)}.${pad(now.getDate())}`;
  timeSpan.textContent = `${pad(now.getHours())}:${pad(now.getMinutes())}:${pad(now.getSeconds())}`;
}

/* =========
  IP
========== */

async function fetchIp() {
  try {
    const res = await fetch("https://api.ipify.org?format=json");
    const data = await res.json();
    ipSpan.textContent = data.ip;
    ipSpan.style.color = "var(--info)";
  } catch {
    const fake = Array.from({ length: 4 }, () =>
      Math.floor(randomBetween(1, 255)),
    ).join(".");
    ipSpan.textContent = fake;
  }
}

/* =========
  Trace
========== */

function updateTrace(level) {
  traceStatus.textContent = level === 0 ? "INACTIVE" : `ACTIVE ${level}%`;
  if (level === 0) traceStatus.style.color = "var(--text-soft)";
  else if (level < 40) traceStatus.style.color = "var(--text)";
  else if (level < 70) traceStatus.style.color = "var(--warning)";
  else traceStatus.style.color = "var(--error)";
}

/* =========
  Screen Lag — random glitches based on lag level
========== */

function startScreenLag(lagLevel) {
  clearInterval(lagInterval);
  if (lagLevel <= 0) return;

  // im wiekszy lag, tym czesciej glitch i dluzej trwa
  // lagLevel 0.5 = glitch co ~40s, lagLevel 75 = co ~1s
  const baseInterval = Math.max(800, 40000 * (1 - lagLevel / 100));

  lagInterval = setInterval(
    async () => {
      // ile glitchy naraz — skaluje z lagiem
      const count = Math.ceil((lagLevel / 100) * 5);

      for (let i = 0; i < count; i++) {
        // losowy rodzaj glitcha
        const type = Math.random();

        if (type < 0.4) {
          // krotki terminal flicker
          terminal.classList.add("glitch-burst");
          await wait(0.05 + randomBetween(0, 0.08));
          terminal.classList.remove("glitch-burst");
        } else if (type < 0.7) {
          // horizontal shift na calym ekranie
          const shift = Math.floor(randomBetween(1, lagLevel / 8 + 2));
          terminal.style.transform = `translateX(${shift}px)`;
          await wait(0.04);
          terminal.style.transform = `translateX(-${shift}px)`;
          await wait(0.03);
          terminal.style.transform = "";
        } else {
          // brightness/contrast spike
          terminal.style.filter = `brightness(${1.3 + lagLevel / 100}) contrast(${1.2 + lagLevel / 200})`;
          await wait(0.06);
          terminal.style.filter = "";
        }

        await wait(randomBetween(0.05, 0.2));
      }

      // przy bardzo wysokim lagu — chwilowe rozmycie
      if (lagLevel > 50 && Math.random() < 0.3) {
        terminal.style.filter = `blur(${randomBetween(0.5, 2)}px)`;
        await wait(randomBetween(0.05, 0.15));
        terminal.style.filter = "";
      }
    },
    baseInterval * (0.7 + randomBetween(0, 0.6)),
  );
}

/* =========
  Active Mission button
========== */

function setActiveMission(screenId) {
  document.querySelectorAll(".mission").forEach((b) => {
    b.classList.remove("mission--active");
    b.classList.add("mission--locked");
  });
  const entry = Object.entries(MISSION_MAP).find(
    ([_, v]) => v.screen === screenId,
  );
  if (!entry) return;
  const btn = document.getElementById(entry[0]);
  if (!btn) return;
  btn.classList.remove("mission--locked");
  btn.classList.add("mission--active");
}

/* =========
  Overlay hint
========== */

function showOverlayHint(keyMap) {
  const keys = Object.keys(keyMap)
    .map((k) => k.toUpperCase())
    .join(" / ");
  peText.textContent = `PRESS  ${keys}`;

  overlay.classList.add("hint-mode");
  overlay.style.display = "flex";
  overlay.style.opacity = "1";
  overlay.style.transition = "none";

  const speeds = [600, 600, 400, 250, 150, 80, 40];
  let second = 0;
  let blinkId = null;

  function runBlink(interval) {
    clearInterval(blinkId);
    blinkId = setInterval(() => {
      overlay.style.opacity = overlay.style.opacity === "0" ? "1" : "0";
    }, interval);
  }

  runBlink(speeds[0]);

  const speedUp = setInterval(() => {
    second++;
    if (second < speeds.length) runBlink(speeds[second]);
  }, 1000);

  const hideTimer = setTimeout(() => {
    clearInterval(blinkId);
    clearInterval(speedUp);
    overlay.style.opacity = "0";
    overlay.style.transition = "opacity 0.2s ease";
    setTimeout(() => {
      overlay.style.display = "none";
      overlay.classList.remove("hint-mode");
    }, 200);
  }, 7000);

  overlay._blinkId = blinkId;
  overlay._speedUpId = speedUp;
  overlay._hideTimer = hideTimer;
}

function stopOverlayBlink() {
  clearInterval(overlay._blinkId);
  clearInterval(overlay._speedUpId);
  clearTimeout(overlay._hideTimer);
  overlay.classList.remove("hint-mode");
  overlay.style.opacity = "0";
  overlay.style.transition = "opacity 0.2s ease";
  setTimeout(() => {
    overlay.style.display = "none";
  }, 200);
}

/* =========
  Set active keys + render shortcuts + start timer
========== */

function setScreenKeys(keyMap) {
  activeKeys = keyMap;
  renderShortcuts(keyMap);
  startTimeout();
  showOverlayHint(keyMap);
}

function renderShortcuts(keyMap) {
  shortcuts.innerHTML = "";
  Object.entries(keyMap).forEach(([key]) => {
    const div = document.createElement("div");
    div.className = "shortcut";
    div.innerHTML = `
      <div class="shortcut-key">${key.toUpperCase()}</div>
      <div class="shortcut-desc"><span>${getKeyLabel(key)}</span></div>
    `;
    shortcuts.appendChild(div);
  });
}

function getKeyLabel(key) {
  const labels = {
    Enter: "continue",
    s: "scan",
    c: "connect",
    h: "help",
    e: "exploit",
    g: "grab",
    q: "query",
    d: "decrypt",
    l: "read logs",
    t: "trace",
    a: "accept / abort",
    r: "reject",
    1: "destroy",
    2: "expose",
    3: "join",
  };
  return labels[key] ?? key;
}

/* =========
  7 second timeout → Game Over
========== */

function startTimeout() {
  clearTimeout(timeoutTimer);
  timeoutTimer = setTimeout(() => {
    if (Object.keys(activeKeys).length === 0) return;
    triggerGameOver();
  }, 7000);
}

function clearActiveTimeout() {
  clearTimeout(timeoutTimer);
  timeoutTimer = null;
}

async function triggerGameOver() {
  activeKeys = {};
  clearActiveTimeout();
  clearInterval(lagInterval);
  shortcuts.innerHTML = "";

  for (let i = 0; i < 8; i++) {
    terminal.classList.add("glitch-burst");
    await wait(0.06 + randomBetween(0, 0.06));
    terminal.classList.remove("glitch-burst");
    await wait(0.04 + randomBetween(0, 0.05));
  }

  const go = document.createElement("div");
  go.id = "game-over-overlay";
  go.innerHTML = `
    <div id="go-text">CONNECTION TERMINATED</div>
    <div id="go-sub">TRACE COMPLETE · SESSION ENDED</div>
    <div id="go-hint">press F5 to restart</div>
  `;
  document.body.appendChild(go);
  await wait(0.05);
  go.style.opacity = "1";
}

/* =========
  Ending animation
========== */

const ENDINGS = {
  destroy: {
    lines: [
      { text: "NEXUS CORE-PRIME-01 · · · OFFLINE", delay: 0 },
      { text: "NEXUS CORE-PRIME-02 · · · OFFLINE", delay: 600 },
      { text: "NEXUS CORE-PRIME-03 · · · OFFLINE", delay: 1200 },
      { text: "ALL NEXUS INFRASTRUCTURE · · · OFFLINE", delay: 2000 },
      { text: "", delay: 2800 },
      { text: "YOU DESTROYED EVERYTHING.", delay: 3200 },
      { text: "INCLUDING YOURSELF.", delay: 4000 },
      { text: "", delay: 4600 },
      { text: "BUT THE WORLD IS FREE.", delay: 5000 },
      { text: "", delay: 5600 },
      { text: "PHANTOM > ...thank you.", delay: 6200 },
    ],
    color: "#00ff88",
    sub: "ENDING: SACRIFICE",
  },
  expose: {
    lines: [
      { text: "TRANSMITTING · · · · · · · 100%", delay: 0 },
      { text: "", delay: 800 },
      { text: "2,341,887,412 PEOPLE RECEIVED THE TRUTH.", delay: 1200 },
      { text: "", delay: 2000 },
      { text: "NEXUS BOARD ARRESTED — 2034.03.17", delay: 2400 },
      { text: "NEXUS DISSOLVED — 2034.04.02", delay: 3000 },
      { text: "INTERNET RESTORED — 2034.06.15", delay: 3600 },
      { text: "", delay: 4400 },
      { text: "PHANTOM > I didn't think you'd do it.", delay: 5000 },
      { text: "PHANTOM > I was wrong.", delay: 5800 },
      { text: "PHANTOM > about a lot of things.", delay: 6400 },
    ],
    color: "#88ccff",
    sub: "ENDING: TRUTH",
  },
  join: {
    lines: [
      { text: "WELCOME TO NEXUS.", delay: 0 },
      { text: "", delay: 800 },
      { text: "BLACKOUT.EXE · · · INITIATED", delay: 1400 },
      { text: "ESTIMATED VICTIMS · · · 847,000,000", delay: 2200 },
      { text: "", delay: 3000 },
      { text: "YOU KNEW.", delay: 3600 },
      { text: "YOU CHOSE THIS.", delay: 4200 },
      { text: "", delay: 4800 },
      { text: "PHANTOM > ...", delay: 5400 },
      { text: "PHANTOM > I trusted you.", delay: 6000 },
      { text: "PHANTOM > that was my mistake.", delay: 6800 },
    ],
    color: "#ff4444",
    sub: "ENDING: BETRAYAL",
  },
};

async function triggerEnding(type) {
  await wait(1.5);

  clearInterval(lagInterval);
  activeKeys = {};
  clearActiveTimeout();
  stopOverlayBlink();

  // seria glitchy zakonczenia
  for (let i = 0; i < 12; i++) {
    terminal.classList.add("glitch-burst");
    await wait(0.04 + randomBetween(0, 0.08));
    terminal.classList.remove("glitch-burst");
    await wait(0.03 + randomBetween(0, 0.06));
  }

  await wait(0.4);

  const ending = ENDINGS[type];
  const el = document.createElement("div");
  el.id = "ending-overlay";
  el.innerHTML = `
    <div id="ending-sub">${ending.sub}</div>
    <div id="ending-lines"></div>
    <div id="ending-hint">press F5 to play again</div>
  `;
  el.style.setProperty("--ending-color", ending.color);
  document.body.appendChild(el);

  await wait(0.1);
  el.style.opacity = "1";

  const linesEl = document.getElementById("ending-lines");

  for (const line of ending.lines) {
    await wait(line.delay / 1000);

    const p = document.createElement("p");
    p.className = "ending-line";
    p.style.color = line.text === "" ? "transparent" : ending.color;
    p.textContent = line.text || ".";
    linesEl.appendChild(p);

    // typewriter na kazda linijke
    if (line.text !== "") {
      const full = line.text;
      p.textContent = "";
      const delay = Math.max(20, Math.min(60, 1200 / full.length));
      for (let c = 0; c < full.length; c++) {
        p.textContent = full.slice(0, c + 1);
        await wait(delay / 1000);
      }
    }
  }

  // pokaz hint po zakonczeniu
  await wait(1);
  document.getElementById("ending-hint").style.opacity = "1";
}

/* =========
  Append lines
========== */

async function appendLines(screenId, lines, onDone = null) {
  if (isAnimating) return;
  isAnimating = true;

  clearActiveTimeout();
  stopOverlayBlink();

  const screen = document.getElementById(screenId);

  for (const line of lines) {
    const p = document.createElement("p");
    p.className = line.cls;
    screen.appendChild(p);

    const delay = Math.max(12, Math.min(35, 800 / line.text.length));
    for (let c = 0; c < line.text.length; c++) {
      p.textContent = line.text.slice(0, c + 1);
      await wait(delay / 1000);
    }

    await wait(randomBetween(0.03, 0.1));
    rootDivTerminal.scrollTop = rootDivTerminal.scrollHeight;
  }

  isAnimating = false;

  if (onDone) {
    await wait(0.3);
    onDone();
  }
}

/* =========
  Typewriter
========== */

async function typewriterScreen(screenId) {
  const screen = document.getElementById(screenId);
  if (!screen) return;

  const lines = Array.from(screen.querySelectorAll("p"));
  const originals = lines.map((p) => p.innerHTML);
  lines.forEach((p) => (p.innerHTML = ""));

  for (let i = 0; i < lines.length; i++) {
    const p = lines[i];
    const full = originals[i];

    if (full.trim() === "") {
      await wait(0.15);
      continue;
    }

    const delay = Math.max(12, Math.min(35, 800 / full.length));
    for (let c = 0; c < full.length; c++) {
      p.innerHTML = full.slice(0, c + 1);
      await wait(delay / 1000);
    }

    await wait(randomBetween(0.02, 0.08));
    rootDivTerminal.scrollTop = rootDivTerminal.scrollHeight;
  }
}

/* =========
  Glitch
========== */

function triggerGlitch() {
  terminal.classList.add("glitch-burst");
  setTimeout(() => terminal.classList.remove("glitch-burst"), 500);
}

/* =========
  Show Screen
========== */

async function showScreen(screenId, traceLevel = 0) {
  if (isAnimating) return;
  isAnimating = true;

  activeKeys = {};
  clearActiveTimeout();
  stopOverlayBlink();
  shortcuts.innerHTML = "";

  triggerGlitch();
  await wait(0.15);

  document.querySelectorAll(".terminal-screen").forEach((s) => {
    s.classList.add("screen-unactive");
  });

  const screen = document.getElementById(screenId);
  screen.classList.remove("screen-unactive");
  rootDivTerminal.scrollTop = 0;
  currentScreen = screenId;

  updateTrace(traceLevel);
  setActiveMission(screenId);

  // ustaw lag dla tej misji
  const entry = Object.values(MISSION_MAP).find((v) => v.screen === screenId);
  if (entry) startScreenLag(entry.lag);

  await typewriterScreen(screenId);

  isAnimating = false;

  const keys = SCREEN_KEYS[screenId];
  if (keys) setScreenKeys(keys);
}

/* =========
  Listeners
========== */

missionList.addEventListener("click", (e) => {
  const btn = e.target.closest(".mission");
  if (!btn || isAnimating) return;
  const config = MISSION_MAP[btn.id];
  if (!config) return;
  showScreen(config.screen, config.trace);
});

document.addEventListener("keydown", (e) => {
  if (isAnimating) return;
  if (activeKeys[e.key]) {
    e.preventDefault();
    activeKeys[e.key]();
  }
});

rootDivTerminal.addEventListener("wheel", (e) => {
  e.preventDefault();
  const dir = e.deltaY > 0 ? 1 : -1;
  rootDivTerminal.scrollBy({ top: 23 * dir, behavior: "instant" });
});

window.addEventListener("resize", () => {
  rootDivTerminal.scrollTop = rootDivTerminal.scrollHeight;
});

/* =========
  Press Enter — start overlay
========== */

const blinkInterval = setInterval(() => {
  peText.style.opacity = peText.style.opacity === "0" ? "1" : "0";
}, 600);

document.addEventListener(
  "keydown",
  (e) => {
    if (e.key !== "Enter") return;
    e.preventDefault();
    clearInterval(blinkInterval);
    overlay.style.transition = "opacity 0.6s ease";
    overlay.style.opacity = "0";
    setTimeout(() => {
      overlay.style.display = "none";
      init();
    }, 600);
  },
  { once: true },
);

/* =========
  Init
========== */

function init() {
  if (inputBar) inputBar.style.display = "none";
  updateClock();
  setInterval(updateClock, 1000);
  fetchIp();
  showScreen("screen-boot", 0);
}
