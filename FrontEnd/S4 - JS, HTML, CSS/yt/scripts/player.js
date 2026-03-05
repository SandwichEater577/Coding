const durationTimeSpan = document.querySelector("#duration-time");
const currentTimeSpan = document.querySelector("#current-time");
const fullscreenButton = document.querySelector("#fullscreen");
const progressBar = document.querySelector("#watch-progress");
const playButton = document.querySelector("#play-button");
const playIcon = document.querySelector("#play-icon");
const nextButton = document.querySelector("#next");
const video = document.querySelector("#video");
const body = document.querySelector("body");

function togglePlay() {
  if (video.paused) {
    video.play();
  } else {
    video.pause();
  }
}

playButton.addEventListener("click", () => {
  togglePlay();
});

video.addEventListener("playing", () => {
  playIcon.src = "./assets/pause.svg";
});

video.addEventListener("pause", () => {
  playIcon.src = "./assets/play.svg";
});

body.addEventListener("keydown", (event) => {
  if (event.code === "Space" || event.code === "KeyK") {
    event.preventDefault();
    togglePlay();
  }
});

nextButton.addEventListener("click", () => {
  video.src = video.src.includes("dreamybull.mp4")
    ? "./assets/video.mp4"
    : "./assets/dreamybull.mp4";
});

video.addEventListener("click", () => {
  togglePlay();
});

video.addEventListener("timeupdate", () => {
  const progress = (video.currentTimeSpan / video.duration) * 100;
  progressBar.style.width = progress + "%";
  currentTime.textContent = formatTime(video.currentTimeSpan);
});

fullscreenButton.addEventListener("click", () => {
  if (video.requestFullscreen) {
    video.requestFullscreen();
  } else if (video.webkitRequestFullscreen) {
    video.webkitRequestFullscreen();
  } else if (video.mozRequestFullScreen) {
    video.mozRequestFullScreen();
  } else if (video.msRequestFullscreen) {
    video.msRequestFullscreen();
  }
});

function formatTime(seconds) {
  return moment.utc(seconds * 1000).format("m:ss");
}
duration.textContent = formatTime(video.duration);

video.addEventListener("loadedmetadata", () => {
  duration.textContent = formatTime(video.duration);
});
