const { ipcRenderer } = require('electron');

// We expose this API before the main web code loads
window.twm_api = {
    launch: (appName) => {
        ipcRenderer.send('launch-app', appName);
    }
};

// Also inject logic to handle mouse events properly
window.addEventListener('DOMContentLoaded', () => {
    const taskbar = document.getElementById('taskbar');
    const notif = document.getElementById('notification-center');
    
    // Elements that need to capture mouse clicks
    const interactiveElements = [taskbar, notif];
    
    interactiveElements.forEach(el => {
        if (!el) return;
        el.addEventListener('mouseenter', () => {
            ipcRenderer.send('@electron/set-ignore-mouse-events', false);
        });
        el.addEventListener('mouseleave', () => {
            ipcRenderer.send('@electron/set-ignore-mouse-events', true, { forward: true });
        });
    });
});
