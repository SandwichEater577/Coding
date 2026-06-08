const loadingScreen = document.getElementById("loading-screen");
const mainContent = document.getElementById("main-content");
const logoSpanText = document.getElementById("logo-span-text");


setTimeout(() => {
    loadingScreen.classList.add("screen-unactive");
    mainContent.classList.remove("screen-unactive");
}, 1670);

if (window.location.href.includes("index.html")) {
    logoSpanText.textContent = "NesTea's Zjebcord";
}

