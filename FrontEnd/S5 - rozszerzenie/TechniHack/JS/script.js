/* =========
  Constants
========== */

const rootDivTerminal = document.getElementById("terminalOutput");
const levelSpan = document.getElementById("level-indicator");
const missionList = document.getElementById("missionList");
const traceStatus = document.querySelector("#traceStatus");
const dateSpan = document.querySelector("#clockDate");
const timeSpan = document.querySelector("#clockTime");
const ipSpan = document.querySelector("#ipAddress");
const terminal = document.getElementById("terminal");

/* =========
  State
========== */

let isAnimating = false;

const MISSION_MAP = {
  mission1: { screen: "screen-mission1", trace: 0 },
  mission2: { screen: "screen-mission2", trace: 12 },
  mission3: { screen: "screen-mission3", trace: 31 },
  mission4: { screen: "screen-mission4", trace: 58 },
  mission5: { screen: "screen-mission5", trace: 74 },
  mission6: { screen: "screen-mission6", trace: 89 },
  mission7: { screen: "screen-mission7", trace: 97 },
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
  dateSpan.textContent = `${now.getFullYear()}.${String(now.getMonth() + 1).padStart(2, "0")}.${String(now.getDate()).padStart(2, "0")}`;
  timeSpan.textContent = `${String(now.getHours()).padStart(2, "0")}:${String(now.getMinutes()).padStart(2, "0")}:${String(now.getSeconds()).padStart(2, "0")}`;
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
    ipSpan.textContent = Array.from({ length: 4 }, () =>
      Math.floor(randomBetween(1, 255)),
    ).join(".");
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
    const chars = full.length;
    const delay = Math.max(12, Math.min(35, 800 / chars));
    for (let c = 0; c < full.length; c++) {
      p.innerHTML = full.slice(0, c + 1);
      await wait(delay / 1000);
    }
    await wait(randomBetween(0.02, 0.08));
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
  triggerGlitch();
  await wait(0.15);
  document
    .querySelectorAll(".terminal-screen")
    .forEach((s) => s.classList.add("screen-unactive"));
  const screen = document.getElementById(screenId);
  screen.classList.remove("screen-unactive");
  rootDivTerminal.scrollTop = 0;
  updateTrace(traceLevel);
  await typewriterScreen(screenId);
  isAnimating = false;
}

/* =========
  Active Mission
========== */

function setActiveMission(btn) {
  document
    .querySelectorAll(".mission")
    .forEach((b) => b.classList.remove("mission--active"));
  btn.classList.add("mission--active");
}

/* =========
  Listeners
========== */

missionList.addEventListener("click", (e) => {
  const btn = e.target.closest(".mission");
  if (!btn || isAnimating) return;
  const config = MISSION_MAP[btn.id];
  if (!config) return;
  setActiveMission(btn);
  showScreen(config.screen, config.trace);
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
  Init
========== */

function init() {
  updateClock();
  setInterval(updateClock, 1000);
  fetchIp();
  showScreen("screen-boot", 0);
}

init();
