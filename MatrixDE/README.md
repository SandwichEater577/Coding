# MatrixDE - Custom Tiling WM for Windows via WSLg

This project provides a full-screen, Matrix-styled web UI renderer via Electron and a C++ core that manages X11 windows in your WSL environment, effectively acting as a shell replacement on Windows 10/11 with WSL2.

## Architecture
- **Web UI (`src/ui`)**: An Electron application that renders a borderless, transparent click-through window covering the screen. It features HTML5 Canvas matrix rain, a taskbar for launching apps, and floating notification elements.
- **C++ Core (`src/core`)**: Connects to the WSLg X11 display, launches the Web UI, listens for X11 Window creation (MapNotify), and applies a basic vertical tiling layout. Features a panic shortcut to restore standard Windows Explorer.

## Setup Requirements

Prerequisites (On your Windows Host via WSL2 Ubuntu/Debian environment):
```bash
sudo apt update
sudo apt install -y cmake g++ libx11-dev nodejs npm
```

## Compilation

1. Build the UI Dependencies:
```bash
cd src/ui
npm install
```

2. Compile the C++ Core:
```bash
cd ../core
mkdir build
cd build
cmake ..
make
```

## Running

1. **Safety First**: Make sure X11 is working by testing `xeyes` in your WSL terminal.
2. Launch the Window Manager:
```bash
./MatrixCore
```

**What happens on execution**:
- It will connect to your default `$DISPLAY`.
- It will spawn `npm start` in the UI directory, pulling up the fullscreen Matrix UI.
- Any new GUI applications launched in WSL (via the taskbar or other terminals) will be tiled automatically across the screen.

**Emergency Exit & Restore:**
Press `Super + Shift + Q` (Windows + Shift + Q) at any time. The C++ application will intercept this hotkey globally, kill the Matrix DE UI, and call `taskkill /f /im explorer.exe` followed by `start explorer.exe` to guarantee your normal desktop is restored.

## Known Limitations on WSLg
Because WSLg uses Weston (Wayland) holding the master X11 `SubstructureRedirectMask`, traditional complete X11 WM behaviors might be restricted. MatrixDE bypasses this using `SubstructureNotifyMask` to arrange standard windows over its background gracefully without breaking WSLg's XWayland bridge.
