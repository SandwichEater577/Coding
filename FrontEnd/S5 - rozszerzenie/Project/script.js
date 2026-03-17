const traceStatusElement = document.querySelector("#traceStatus");
const terminalOutput = document.querySelector("#terminalOutput");
const ipAddressElement = document.querySelector("#ipAddress");
const clockDateElement = document.querySelector("#clockDate");
const clockTimeElement = document.querySelector("#clockTime");
const mission1Button = document.querySelector("#mission1");
const mission2Button = document.querySelector("#mission2");
const mission3Button = document.querySelector("#mission3");
const mission4Button = document.querySelector("#mission4");

if (mission2Button) {
  mission2Button.addEventListener("click", () => {
    // add function
    console.log("Mission 2 clicked");
  });
}

const missionButtons = [
  mission1Button,
  mission2Button,
  mission3Button,
  mission4Button,
].filter(Boolean);

function setActiveMission(activeButton) {
  missionButtons.forEach((button) => {
    button.classList.toggle("active", button === activeButton);
  });
}

function renderMission1Output() {
  if (!terminalOutput) return;

  terminalOutput.innerHTML = `
    <div id="root" style="font-size:26px;">
      <div style="display: flex; flex-direction: column; color: var(--line);" class="div">
        <span>[SYSTEM] Loading Mission: 01 - Tutorial...</span>
        <span class="system-output">[SYSTEM] Connecting to training server...</span>
        <span class="system-output">[SYSTEM] Initializing Environment...</span>
      </div>
      <div style="display: flex; flex-direction: column; color: var(--text);" class="div">
        <span class="ok-output">[OK] Connected · ping: 67ms</span>
        <span class="ok-output">[OK] Environment ready</span>
      </div>
      <div style="display: flex; flex-direction: column; color: var(--info);" class="div">
        <span>════════════════════════════════════════</span>
        <span>WELCOME, ANONYMOUS</span>
        <span>This is the Tutorial</span>
        <span>Learn the basics before you start a mission!</span>
        <span>════════════════════════════════════════</span>
      </div>
      <div style="display: flex; flex-direction: column;" class="div">
        <span style="color: var(--warning);">[!] MISSION OBJECTIVE:</span>
        <span style="color: var(--line)">&gt; Use the command '--scan' to discover a Network</span>
        <span style="color: var(--line)">&gt; Connect to a discovered host via '--connect'</span>
        <span style="color: var(--line)">&gt; Enter '--help' if you are stuck!</span>
      </div>
      <div style="display: flex; flex-direction: column;" class="div">
        <span style="color: var(--warning);">[!] AVAILABLE COMMANDS:</span>
        <span style="color: var(--line)">&gt; --help ══ Lists all commands</span>
        <span style="color: var(--line)">&gt; --scan ══ Scans for networks</span>
        <span style="color: var(--line)">&gt; --connect 'name' ══ Connects to a host</span>
        <span style="color: var(--line)">&gt; --clear ══ Clears the terminals output</span>
        <span style="color: var(--line)">&gt; --start ══ Starts the mission from the start</span>
      </div>
      <div style="display: flex; flex-direction: column;" class="div">
        <span style="color: var(--text);">[MISSION] Enter '--start' to start</span>
      </div>
    </div>
  `;
}

if (mission1Button) {
  mission1Button.addEventListener("click", () => {
    setActiveMission(mission1Button);
    renderMission1Output();
  });
}

if (mission2Button) {
  mission2Button.addEventListener("click", () => {
    setActiveMission(mission2Button);

    if (terminalOutput) {
      terminalOutput.innerHTML = `
        <div id="root">
          <div></div>
        </div>
      `;
    }
  });
}

if (traceStatusElement) {
  traceStatusElement.textContent = "ACTIVE";
  traceStatusElement.style.color = "var(--info)";
}

async function updatePublicIp() {
  if (!ipAddressElement) {
    return;
  }

  ipAddressElement.textContent = "LOADING...";
  ipAddressElement.style.color = "var(--warning)";

  try {
    const response = await fetch("https://api.ipify.org?format=json");
    const data = await response.json();

    if (data && data.ip) {
      ipAddressElement.textContent = data.ip;
      ipAddressElement.style.color = "var(--info)";
      return;
    }
  } catch (error) {
    console.error("Could not fetch public IP:", error);
    ipAddressElement.style.color = "var(--error)";
  }

  ipAddressElement.textContent = "N/A";
}

function formatClockDate(date) {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");

  return `${year}.${month}.${day}`;
}

function formatClockTime(date) {
  const hours = String(date.getHours()).padStart(2, "0");
  const minutes = String(date.getMinutes()).padStart(2, "0");
  const seconds = String(date.getSeconds()).padStart(2, "0");

  return `${hours}:${minutes}:${seconds}`;
}

function updateClock() {
  const now = new Date();

  if (clockDateElement) {
    clockDateElement.textContent = formatClockDate(now);
  }

  if (clockTimeElement) {
    clockTimeElement.textContent = formatClockTime(now);
  }
}

updateClock();
updatePublicIp();
if (mission1Button) {
  setActiveMission(mission1Button);
}
renderMission1Output();
setInterval(updateClock, 1000);
