/*=======
 Triples
=======*/

const bootSequenceTerminalOuput = document.getElementById(
  "terminal-output-boot-sequence-screen",
);

/*=========
 Constants
=========*/

const usbAnimationScreen = document.getElementById("usb-animation");
const rootDivTerminal = document.getElementById("terminalOutput");
const levelSpan = document.getElementById("level-indicator");
const missionButton1 = document.getElementById("mission1");
const missionButton2 = document.getElementById("mission2");
const missionButton3 = document.getElementById("mission3");
const missionButton4 = document.getElementById("mission4");
const missionButton5 = document.getElementById("mission5");
const missionButton6 = document.getElementById("mission6");
const missionButton7 = document.getElementById("mission7");
const homeScreen = document.getElementById("home-screen");
const activeSpan = document.querySelector("#traceStatus");
const dateSpan = document.querySelector("#clockDate");
const timeSpan = document.querySelector("#clockTime");
const ipSpan = document.querySelector("#ipAddress");

/*====
 Lets
====*/

let traceVariable = "None";

/*=========
 Functions
=========*/

function wait(seconds) {
  return new Promise((resolve) => setTimeout(resolve, seconds * 1000));
}

function IpAddress() {
  ipSpan.textContent = "67.67.67.67";
}

function TraceStatus() {
  activeSpan.textContent = traceVariable;
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

rootDivTerminal.addEventListener("wheel", (event) => {
  event.preventDefault();

  const lineHeight = 23;
  const direction = event.deltaY > 0 ? 1 : -1;

  rootDivTerminal.scrollBy({
    top: lineHeight * direction,
    behavior: "instant",
  });
});

/*==============
 Initialization
==============*/

async function loadTerminal() {
  setInterval(UpdateClock, 1000);
  UpdateClock();
  TraceStatus();
  IpAddress();
}

loadTerminal();
