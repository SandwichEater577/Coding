// Matrix Digital Rain Effect
const canvas = document.getElementById('matrix-canvas');
const ctx = canvas.getContext('2d');

let width, height;

function resizeCanvas() {
    width = window.innerWidth;
    height = window.innerHeight;
    canvas.width = width;
    canvas.height = height;
}

window.addEventListener('resize', resizeCanvas);
resizeCanvas();

const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#$%^&*()_+~`|}{[]:;?><,./-=ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ';
const drops = [];
const fontSize = 16;
let columns = width / fontSize;

// Initialize drops array
for (let x = 0; x < columns; x++) {
    drops[x] = Math.random() * -100; // Start at random negative y to stagger
}

function drawMatrix() {
    // Semi-transparent black to create trailing effect
    ctx.fillStyle = 'rgba(0, 0, 0, 0.05)';
    ctx.fillRect(0, 0, width, height);

    ctx.fillStyle = '#0F0'; // Matrix Green
    ctx.font = fontSize + 'px monospace';

    // Ensure columns are updated if resized
    if(columns !== Math.floor(width / fontSize)) {
        columns = Math.floor(width / fontSize);
        for(let i=drops.length; i<columns; i++) drops[i] = Math.random() * -100;
    }

    for (let i = 0; i < drops.length; i++) {
        // Random character
        const text = chars.charAt(Math.floor(Math.random() * chars.length));

        // Draw character
        // Occasional white text for pure Matrix effect
        ctx.fillStyle = Math.random() > 0.98 ? '#FFF' : '#0F0';
        ctx.fillText(text, i * fontSize, drops[i] * fontSize);

        // Reset drop to top randomly
        if (drops[i] * fontSize > height && Math.random() > 0.975) {
            drops[i] = 0;
        }

        // Move drop down
        drops[i]++;
    }
}

// Render loop at ~30fps for that retro feel
setInterval(drawMatrix, 33);

// Clock implementation
function updateClock() {
    const now = new Date();
    const h = String(now.getHours()).padStart(2, '0');
    const m = String(now.getMinutes()).padStart(2, '0');
    const s = String(now.getSeconds()).padStart(2, '0');
    document.getElementById('clock').innerText = `${h}:${m}`;
}
setInterval(updateClock, 1000);
updateClock();

// App Launcher bridge to C++
function launchApp(appName) {
    console.log(`Requesting launch: ${appName}`);
    
    // Check if we are running inside the webview object provided by C++
    if (window.twm_api && typeof window.twm_api.launch === 'function') {
        window.twm_api.launch(appName);
    } else {
        // Fallback for browser testing
        alert(`[WM Bridge Mock] Launching application: ${appName}`);
    }
}
