const usbAnimationScreen = document.getElementById("usb-animation");
const homeScreen = document.getElementById("home-screen");
/*===========
USB Animation
===========*/

function startUSBAnimation() {
  usbAnimationScreen.classList.add("screen-unactive");
  homeScreen.classList.remove("screen-unactive");
}

startUSBAnimation();

const rootDivTerminal = document.getElementById("terminalOutput");
const levelSpan = document.getElementById("level-indicator");
const missionButton1 = document.getElementById("mission1");
const missionButton2 = document.getElementById("mission2");
const missionButton3 = document.getElementById("mission3");
const missionButton4 = document.getElementById("mission4");
const activeSpan = document.querySelector("#traceStatus");
const dateSpan = document.querySelector("#clockDate");
const timeSpan = document.querySelector("#clockTime");
const ipSpan = document.querySelector("#ipAddress");

function initializeTerminal() {
  levelSpan.textContent = "1";
  // missionButton1.classList.add("mission--active");
  // missionButton2.classList.remove("mission--active");
  // missionButton3.classList.remove("mission--active");
  // missionButton4.classList.remove("mission--active");
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

setInterval(UpdateClock, 1000);
initializeTerminal();
UpdateClock();
TraceStatus();
IpAddress();
