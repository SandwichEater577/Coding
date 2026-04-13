#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/keysym.h>
#include <iostream>
#include <vector>
#include <cstdlib>
#include <unistd.h>
#include <sys/wait.h>
#include <thread>
#include <chrono>
#include <algorithm>

// --- Config for Tiling ---
const int TILE_MARGIN_TOP = 60; // Leave space for Matrix UI elements if needed
const int TILE_MARGIN_BOTTOM = 60; // Taskbar area
const int TILE_GAP = 10;

struct WindowWM {
    Window id;
    bool mapped;
};

std::vector<WindowWM> managed_windows;
Display* dpy = nullptr;
Window root;
int screen;
int screen_width, screen_height;

void spawnProcess(const char* cmd) {
    if (fork() == 0) {
        setsid();
        execl("/bin/sh", "sh", "-c", cmd, (char*)NULL);
        exit(0);
    }
}

void retileWindows() {
    // Count mappped windows
    std::vector<WindowWM*> active_wins;
    for (auto& w : managed_windows) {
        if (w.mapped) active_wins.push_back(&w);
    }

    if (active_wins.empty()) return;

    int n = active_wins.size();
    int work_width = screen_width - (TILE_GAP * 2);
    int work_height = screen_height - TILE_MARGIN_TOP - TILE_MARGIN_BOTTOM;
    
    // Simple vertical split layout
    int win_width = (work_width - ((n - 1) * TILE_GAP)) / n;
    
    for (int i = 0; i < n; ++i) {
        int x = TILE_GAP + i * (win_width + TILE_GAP);
        int y = TILE_MARGIN_TOP;
        XMoveResizeWindow(dpy, active_wins[i]->id, x, y, win_width, work_height);
        XRaiseWindow(dpy, active_wins[i]->id); // Keep them on top if possible
    }
    XFlush(dpy);
}

void e_killExplorer() {
    std::cout << "[WM] Killing Windows Explorer to take over shell..." << std::endl;
    // Uses interop to kill explorer from WSL
    system("/mnt/c/Windows/System32/cmd.exe /c taskkill /f /im explorer.exe");
}

void e_restoreExplorer() {
    std::cout << "[WM] Restoring Windows Explorer..." << std::endl;
    system("/mnt/c/Windows/System32/cmd.exe /c start explorer.exe");
}

int main() {
    dpy = XOpenDisplay(nullptr);
    if (!dpy) {
        std::cerr << "Failed to open X display. Is WSLg/X11 running?" << std::endl;
        return 1;
    }

    screen = DefaultScreen(dpy);
    root = RootWindow(dpy, screen);
    screen_width = DisplayWidth(dpy, screen);
    screen_height = DisplayHeight(dpy, screen);

    std::cout << "[WM] Display connected: " << screen_width << "x" << screen_height << std::endl;

    // Optional: Kill Windows Explorer on startup to simulate "Shell Replacement"
    // e_killExplorer();

    // Spawn the web GUI (Electron)
    std::cout << "[WM] Injecting UI layer..." << std::endl;
    spawnProcess("cd ../ui && npm start");

    // Try to become the Window Manager
    // WSLg Weston might prevent SubstructureRedirectMask. If it fails, X11 will throw an error and exit.
    // For a robust implementation, we would set an X error handler. 
    // Here we use SubstructureNotifyMask which is safer and allowed alongside Wayland.
    XSelectInput(dpy, root, SubstructureNotifyMask | KeyPressMask);

    // Setup global hotkey: Super + Shift + Q
    KeyCode key_q = XKeysymToKeycode(dpy, XK_Q);
    // Mod4Mask is usually the Super/Windows key
    XGrabKey(dpy, key_q, Mod4Mask | ShiftMask, root, True, GrabModeAsync, GrabModeAsync);

    XEvent ev;
    std::cout << "[WM] Core event loop started." << std::endl;

    while (true) {
        XNextEvent(dpy, &ev);
        
        if (ev.type == KeyPress) {
            if (ev.xkey.keycode == key_q && (ev.xkey.state & (Mod4Mask | ShiftMask)) == (Mod4Mask | ShiftMask)) {
                std::cout << "[WM] Emergency Exit triggered! Restoring system..." << std::endl;
                // Kill electron node processes
                system("pkill -f 'electron .'"); 
                e_restoreExplorer();
                break;
            }
        } 
        else if (ev.type == CreateNotify) {
            // New window created, track it
            Window w = ev.xcreatewindow.window;
            // Ignore if it has override_redirect (e.g. menus or our UI)
            if (!ev.xcreatewindow.override_redirect) {
                managed_windows.push_back({w, false});
                XSelectInput(dpy, w, StructureNotifyMask | FocusChangeMask);
            }
        }
        else if (ev.type == MapNotify) {
            Window w = ev.xmap.window;
            auto it = std::find_if(managed_windows.begin(), managed_windows.end(), [w](const WindowWM& win){ return win.id == w; });
            if (it != managed_windows.end()) {
                it->mapped = true;
                retileWindows();
            }
        }
        else if (ev.type == UnmapNotify) {
            Window w = ev.xunmap.window;
            auto it = std::find_if(managed_windows.begin(), managed_windows.end(), [w](const WindowWM& win){ return win.id == w; });
            if (it != managed_windows.end()) {
                it->mapped = false;
                retileWindows();
            }
        }
        else if (ev.type == DestroyNotify) {
            Window w = ev.xdestroywindow.window;
            managed_windows.erase(
                std::remove_if(managed_windows.begin(), managed_windows.end(), 
                    [w](const WindowWM& win){ return win.id == w; }), 
                managed_windows.end()
            );
            retileWindows();
        }
    }

    XCloseDisplay(dpy);
    return 0;
}
