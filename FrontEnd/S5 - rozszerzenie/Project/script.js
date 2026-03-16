const traceStatusElement = document.getElementById("traceStatus");
const ipAddressElement = document.getElementById("ipAddress");
const clockDateElement = document.getElementById("clockDate");
const clockTimeElement = document.getElementById("clockTime");

if (traceStatusElement) {
  traceStatusElement.textContent = "ACTIVE";
}

async function updatePublicIp() {
  if (!ipAddressElement) {
    return;
  }

  ipAddressElement.textContent = "LOADING...";

  try {
    const response = await fetch("https://api.ipify.org?format=json");
    const data = await response.json();

    if (data && data.ip) {
      ipAddressElement.textContent = data.ip;
      return;
    }
  } catch (error) {
    console.error("Could not fetch public IP:", error);
  }

  ipAddressElement.textContent = "N/A";
}

function formatClockDate(date) {
  const year = date.getFullYear();
  let month = date.getMonth() + 1;
  if (month < 10) month = "0" + month;
  const day = date.getDate();
  if (day < 10) day = "0" + day;

  return year + "." + month + "." + day;
}

function formatClockTime(date) {
  let hours = date.getHours();
  if (hours < 10) hours = "0" + hours;
  let minutes = date.getMinutes();
  if (minutes < 10) minutes = "0" + minutes;
  let seconds = date.getSeconds();
  if (seconds < 10) seconds = "0" + seconds;

  return hours + ":" + minutes + ":" + seconds;
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
setInterval(updateClock, 1000);
