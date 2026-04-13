const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('path');
const { exec } = require('child_process');

function createWindow () {
  const win = new BrowserWindow({
    width: 1920,
    height: 1080,
    fullscreen: true,
    transparent: true,
    frame: false,
    alwaysOnTop: true,
    skipTaskbar: true,
    webPreferences: {
      nodeIntegration: true, // For demo purposes, allow node in renderer
      contextIsolation: false,
      preload: path.join(__dirname, 'preload.js')
    }
  });

  // Load the index.html of the app.
  win.loadFile('index.html');
  
  // Make the window ignore mouse events where the background is transparent
  // This allows clicks on the "workspace" to pass through to the windows behind it
  win.setIgnoreMouseEvents(true, { forward: true });

  // Handle IPC messages from frontend
  ipcMain.on('launch-app', (event, appName) => {
    console.log(`[Electron Node] Launching: ${appName}`);
    
    // Map of internal names to actual WSL or Windows commands
    const appMap = {
      'terminal': 'x-terminal-emulator || gnome-terminal || alacritty || /mnt/c/Windows/System32/wt.exe',
      'files': 'nautilus || thunar || /mnt/c/Windows/explorer.exe .',
      'browser': 'google-chrome || firefox || x-www-browser || /mnt/c/Program\\ Files/Google/Chrome/Application/chrome.exe',
      'spotify': '/mnt/c/Users/Default/AppData/Roaming/Spotify/Spotify.exe || spotify',
      'discord': '/mnt/c/Users/Default/AppData/Local/Discord/Update.exe --processStart Discord.exe || discord'
    };

    const cmd = appMap[appName];
    if (cmd) {
      exec(cmd, (err, stdout, stderr) => {
        if (err) console.error(`Failed to launch ${appName}: ${err}`);
      });
    }
  });

  ipcMain.on('@electron/set-ignore-mouse-events', (event, ignore, options) => {
    const win = BrowserWindow.fromWebContents(event.sender);
    win.setIgnoreMouseEvents(ignore, options);
  });
}

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
