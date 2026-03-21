const usbAnimationScreen = document.getElementById("usb-animation");
const homeScreen = document.getElementById("home-screen");

const rootDivTerminal = document.getElementById("terminalOutput");
const levelSpan = document.getElementById("level-indicator");
const missionButton1 = document.getElementById("mission1");
const missionButton2 = document.getElementById("mission2");
const missionButton3 = document.getElementById("mission3");
const missionButton4 = document.getElementById("mission4");
const missionButton5 = document.getElementById("mission5");
const missionButton6 = document.getElementById("mission6");
const missionButton7 = document.getElementById("mission7");
const activeSpan = document.querySelector("#traceStatus");
const dateSpan = document.querySelector("#clockDate");
const timeSpan = document.querySelector("#clockTime");
const ipSpan = document.querySelector("#ipAddress");

/*=========
 Functions
=========*/

function wait(seconds) {
  return new Promise((resolve) => setTimeout(resolve, seconds * 1000));
}

async function loadBootTerminal() {
  function getRandomWait() {
    return Math.random() * (1 - 0.25) + 0.25;
  }

  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span></span>
    </div>
  </div>
  `;
  await wait(getRandomWait());
  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span>[OK]</span>
    </div>
  </div>
  `;
  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Loading AES-256 encryption layer... </span>
      <span></span>
    </div>
  </div>
  `;
  await wait(getRandomWait());
  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Loading AES-256 encryption layer... </span>
      <span>[OK]</span>
    </div>
  </div>
  `;
  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Loading AES-256 encryption layer... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Establishing secure tunnel... </span>
      <span></span>
    </div>
  </div>
  `;
  await wait(getRandomWait());
  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Loading AES-256 encryption layer... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Establishing secure tunnel... </span>
      <span>[OK]</span>
    </div>
  </div>
  `;
  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Loading AES-256 encryption layer... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Establishing secure tunnel... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Spoofing MAC address... </span>
      <span></span>
    </div>
  </div>
  `;
  await wait(getRandomWait());
  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Loading AES-256 encryption layer... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Establishing secure tunnel... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Spoofing MAC address... </span>
      <span>[OK]</span>
    </div>
  </div>
  `;
  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Loading AES-256 encryption layer... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Establishing secure tunnel... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Spoofing MAC address... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">  
      <span>[BOOT] Bypassing local IDS... </span>
      <span></span>
    </div>
  </div>
  `;
  await wait(getRandomWait());
  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Loading AES-256 encryption layer... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Establishing secure tunnel... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Spoofing MAC address... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Bypassing local IDS... </span>
      <span>[OK]</span>
    </div>
  </div>
  `;
  rootDivTerminal.innerHTML = `
  <div style="display: flex; flex-direction: column;">
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Initializing kernel modules... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Loading AES-256 encryption layer... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Establishing secure tunnel... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Spoofing MAC address... </span>
      <span>[OK]</span>
    </div>
    <div style="display: flex; flex-direction: row;">
      <span>[BOOT] Bypassing local IDS... </span>
      <span>[OK]</span>
    </div>
  </div>
  <div>
    <span>[SYSTEM] All modules loaded.</span><br>
    <span>[SYSTEM] Session Started, Welcome back.</span>
  </div>
  <div>
    <span>══════════════════════════════════════════════</span><br>
    <span>[PHANTOM] &gt; you are 6 days late.</span><br>
    <span>[PHANTOM] &gt; NEXUS already knows you're here.</span><br>
    <span>[PHANTOM] &gt; doesn't matter. too late to turn back.</span><br>
    <span>[PHANTOM] &gt; you have 4 missions. start with the tutorial.</span><br>
    <span>[PHANTOM] &gt; and don't open port 8472. never.</span><br>
    <span>══════════════════════════════════════════════</span>
  </div>
  <div>
    <span>[SYSTEM] Click a mission in the sidebar to begin.</span><br>
    <span>[SYSTEM] Type '--help' to see available commands.</span>
  </div>
  `;
}

function IpAddress() {
  ipSpan.textContent = "67.67.67.67";
}

function TraceStatus() {
  activeSpan.textContent = "ACTIVE";
}

function UpdateClock() {
  const now = new Date();
  const date = now.toLocaleDateString();
  const time = now.toLocaleTimeString();
  dateSpan.textContent = date;
  timeSpan.textContent = time;
}

/*=========
 Listeners
=========*/
/*==============
 Initialization
==============*/

setInterval(UpdateClock, 1000);
UpdateClock();
TraceStatus();
IpAddress();
loadBootTerminal();
