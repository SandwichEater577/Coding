"""
NoiseTrigger  —  press SPACE when a loud noise is detected
Requires:  pip install pygame sounddevice numpy pynput
"""
import pygame
import sounddevice as sd
import numpy as np
import threading
import time
import math
import sys
import json        
import os
from pynput.keyboard import Key, Controller

# ── constants ─────────────────────────────────────────────────────────────────
W, H        = 840, 650
FPS         = 60
BLOCK_SIZE  = 512          # audio frames per callback
CONFIG_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                           "noise_trigger_cfg.json")

# ── palette ───────────────────────────────────────────────────────────────────
BG        = (10,  12,  20)
PANEL     = (16,  20,  35)
ACCENT    = (0,  200, 140)
ACCENT2   = (255, 80, 100)
ACCENT3   = (255, 200,  50)
GREY      = (60,  70,  90)
LGREY     = (120, 140, 170)
WHITE     = (230, 235, 245)
BLACK     = (0,   0,   0)
DARK      = (8,  10,  18)
ERRCLR    = (255, 80,  60)

def lerp(a, b, t):    return a + (b - a) * t
def clamp(v, lo, hi): return max(lo, min(hi, v))

def rms_to_db(rms):
    return -96.0 if rms < 1e-9 else 20.0 * math.log10(clamp(rms, 1e-9, 1.0))

def rrect(surf, color, rect, r=10, alpha=None):
    if alpha is not None:
        s = pygame.Surface((rect[2], rect[3]), pygame.SRCALPHA)
        pygame.draw.rect(s, (*color, alpha), (0, 0, rect[2], rect[3]), border_radius=r)
        surf.blit(s, (rect[0], rect[1]))
    else:
        pygame.draw.rect(surf, color, rect, border_radius=r)


# ── device helpers ────────────────────────────────────────────────────────────
def list_input_devices():
    """Return list of (sd_index, name) for all input-capable devices."""
    devs = []
    try:
        for i, d in enumerate(sd.query_devices()):
            if d['max_input_channels'] > 0:
                devs.append((i, d['name']))
    except Exception:
        pass
    return devs

def default_input_index():
    try:
        return sd.default.device[0]   # (input, output) tuple
    except Exception:
        return None


# ── audio engine ──────────────────────────────────────────────────────────────
class AudioEngine:
    def __init__(self):
        self.stream      = None
        self.running     = False
        self.current_db  = -96.0
        self.peak_db     = -96.0
        self.peak_ts     = 0.0
        self.sensitivity = -30.0
        self.cooldown    = 0.4
        self._last_trig  = 0.0
        self.trig_flash  = 0.0
        self.keyboard    = Controller()
        self.lock        = threading.Lock()
        self.history     = []
        self.error_msg   = ""
        self.active_rate = 0
        self.device_idx  = default_input_index()

    # ── stream control ────────────────────────────────────────────────────────
    def start(self):
        self.stop()
        if self.device_idx is None:
            with self.lock:
                self.error_msg = "No input device selected"
            return
        try:
            dev_info = sd.query_devices(self.device_idx)
            rate     = int(dev_info['default_samplerate'])
            ch       = min(2, int(dev_info['max_input_channels']))
            self.stream = sd.InputStream(
                device=self.device_idx,
                channels=ch,
                samplerate=rate,
                blocksize=BLOCK_SIZE,
                dtype='float32',
                callback=self._callback,
            )
            self.stream.start()
            self.active_rate = rate
            self.running     = True
            with self.lock:
                self.error_msg = ""
        except Exception as e:
            with self.lock:
                self.error_msg = str(e)
            self.running = False

    def stop(self):
        self.running = False
        if self.stream:
            try:
                self.stream.stop()
                self.stream.close()
            except Exception:
                pass
            self.stream = None

    # ── sounddevice callback (runs in its own thread) ─────────────────────────
    def _callback(self, indata, frames, time_info, status):
        # indata shape: (frames, channels)  dtype: float32  range: [-1, 1]
        mono = indata[:, 0] if indata.ndim > 1 else indata.flatten()
        rms  = float(np.sqrt(np.mean(mono ** 2)))
        db   = rms_to_db(rms)
        now  = time.time()
        with self.lock:
            self.current_db = db
            if db > self.peak_db:
                self.peak_db = db
                self.peak_ts = now
            elif now - self.peak_ts > 2.5:
                self.peak_db = lerp(self.peak_db, db, 0.05)
            self.history.append(db)
            if len(self.history) > 200:
                self.history.pop(0)
            if db >= self.sensitivity and now - self._last_trig > self.cooldown:
                self._last_trig = now
                self.trig_flash = 1.0
                threading.Thread(target=self._press_space, daemon=True).start()

    def _press_space(self):
        try:
            self.keyboard.press(Key.space)
            time.sleep(0.05)
            self.keyboard.release(Key.space)
        except Exception as e:
            print(f"Key press error: {e}")

    def tick(self, dt):
        with self.lock:
            self.trig_flash = max(0.0, self.trig_flash - dt * 3.0)

    def close(self):
        self.stop()


# ── slider widget ─────────────────────────────────────────────────────────────
class Slider:
    def __init__(self, x, y, w, lo, hi, val, label, fmt="{:.1f}", unit=""):
        self.rect = pygame.Rect(x, y, w, 6)
        self.lo, self.hi = lo, hi
        self.val  = val
        self.label, self.fmt, self.unit = label, fmt, unit
        self.dragging = False
        self.hr = 10

    def _hx(self):
        return int(self.rect.x + clamp((self.val - self.lo) / (self.hi - self.lo), 0, 1) * self.rect.w)

    def draw(self, surf, fsm):
        pygame.draw.rect(surf, GREY, self.rect, border_radius=3)
        fw = int(clamp((self.val - self.lo) / (self.hi - self.lo), 0, 1) * self.rect.w)
        pygame.draw.rect(surf, ACCENT, (self.rect.x, self.rect.y, fw, self.rect.h), border_radius=3)
        hx, hy = self._hx(), self.rect.centery
        pygame.draw.circle(surf, WHITE,  (hx, hy), self.hr)
        pygame.draw.circle(surf, ACCENT, (hx, hy), self.hr - 3)
        lbl = fsm.render(self.label, True, LGREY)
        surf.blit(lbl, (self.rect.x, self.rect.y - 24))
        vt = fsm.render(self.fmt.format(self.val) + self.unit, True, WHITE)
        surf.blit(vt, (self.rect.right - vt.get_width(), self.rect.y - 24))

    def event(self, ev):
        if ev.type == pygame.MOUSEBUTTONDOWN and ev.button == 1:
            if math.hypot(ev.pos[0] - self._hx(), ev.pos[1] - self.rect.centery) < self.hr + 4:
                self.dragging = True
        if ev.type == pygame.MOUSEBUTTONUP:
            self.dragging = False
        if ev.type == pygame.MOUSEMOTION and self.dragging:
            t = clamp((ev.pos[0] - self.rect.x) / self.rect.w, 0, 1)
            self.val = self.lo + t * (self.hi - self.lo)


# ── dropdown widget ───────────────────────────────────────────────────────────
class Dropdown:
    def __init__(self, x, y, w, h, options):
        self.rect    = pygame.Rect(x, y, w, h)
        self.options = options   # [(index, name), ...]
        self.sel     = 0
        self.open    = False

    @property
    def value(self):
        return self.options[self.sel][0] if self.options else None

    def _clip(self, font, text, max_w):
        while font.size(text)[0] > max_w and text:
            text = text[:-1]
        return text + ("…" if font.size(text + "…")[0] <= max_w + 20 else "")

    def draw(self, surf, fsm):
        col = ACCENT if self.open else GREY
        rrect(surf, PANEL, self.rect, 8)
        pygame.draw.rect(surf, col, self.rect, 2, border_radius=8)
        label = self.options[self.sel][1] if self.options else "No input devices found"
        t = fsm.render(self._clip(fsm, label, self.rect.w - 36), True, WHITE)
        surf.blit(t, (self.rect.x + 12, self.rect.centery - t.get_height() // 2))
        arrow = fsm.render("▲" if self.open else "▼", True, LGREY)
        surf.blit(arrow, (self.rect.right - 26, self.rect.centery - arrow.get_height() // 2))
        if self.open:
            max_show = 6
            for i, (_, name) in enumerate(self.options[:max_show + (self.sel // max_show) * max_show]):
                r = pygame.Rect(self.rect.x, self.rect.bottom + i * self.rect.h,
                                self.rect.w, self.rect.h)
                rrect(surf, ACCENT if i == self.sel else (22, 28, 48), r, 4)
                pygame.draw.rect(surf, GREY, r, 1, border_radius=4)
                nt = fsm.render(self._clip(fsm, name, self.rect.w - 24), True, WHITE)
                surf.blit(nt, (r.x + 12, r.centery - nt.get_height() // 2))

    def event(self, ev):
        if ev.type == pygame.MOUSEBUTTONDOWN and ev.button == 1:
            if self.rect.collidepoint(ev.pos):
                self.open = not self.open
                return
            if self.open:
                for i in range(len(self.options)):
                    r = pygame.Rect(self.rect.x, self.rect.bottom + i * self.rect.h,
                                    self.rect.w, self.rect.h)
                    if r.collidepoint(ev.pos):
                        self.sel = i
                        self.open = False
                        return
                self.open = False


# ── main application ──────────────────────────────────────────────────────────
class App:
    def __init__(self):
        pygame.init()
        self.screen = pygame.display.set_mode((W, H))
        pygame.display.set_caption("⚡ NoiseTrigger")
        self.clock = pygame.time.Clock()

        self.font_lg = pygame.font.SysFont("Consolas",      46, bold=True)
        self.font_md = pygame.font.SysFont("Consolas",      21, bold=True)
        self.font_sm = pygame.font.SysFont("Segoe UI",      16)
        self.font_xs = pygame.font.SysFont("Segoe UI",      13)

        self.audio = AudioEngine()
        self.tab   = "main"
        self.active = True
        self._tab_rects  = {}
        self._btn_active = pygame.Rect(0, 0, 0, 0)
        self._btn_apply  = pygame.Rect(0, 0, 0, 0)
        self._btn_refresh= pygame.Rect(0, 0, 0, 0)

        devices = list_input_devices()
        self.sl_sens = Slider(330, 305, 380, -80,  0, -30.0, "Threshold", "{:.0f}", " dB")
        self.sl_cool = Slider(330, 400, 380,  0.1, 2.0, 0.4, "Cooldown",  "{:.2f}", " s")
        self.dd      = Dropdown(330, 210, 380, 38, devices)

        # pre-select matching device
        for i, (idx, _) in enumerate(devices):
            if idx == self.audio.device_idx:
                self.dd.sel = i
                break

        self._load_config()
        self.audio.sensitivity = self.sl_sens.val
        self.audio.cooldown    = self.sl_cool.val
        self.audio.start()

    # ── config ────────────────────────────────────────────────────────────────
    def _load_config(self):
        try:
            with open(CONFIG_FILE) as f:
                cfg = json.load(f)
            self.sl_sens.val = cfg.get("sensitivity", -30.0)
            self.sl_cool.val = cfg.get("cooldown", 0.4)
            saved_dev = cfg.get("device")
            for i, (idx, _) in enumerate(self.dd.options):
                if idx == saved_dev:
                    self.dd.sel = i
        except Exception:
            pass

    def _save_config(self):
        try:
            with open(CONFIG_FILE, "w") as f:
                json.dump({"sensitivity": self.sl_sens.val,
                           "cooldown":    self.sl_cool.val,
                           "device":      self.dd.value}, f)
        except Exception:
            pass

    def _apply_settings(self):
        self.audio.sensitivity = self.sl_sens.val
        self.audio.cooldown    = self.sl_cool.val
        new_dev = self.dd.value
        if new_dev != self.audio.device_idx:
            self.audio.device_idx = new_dev
            self.audio.stop()
            self.audio.start()

    def _refresh_devices(self):
        devs = list_input_devices()
        cur  = self.dd.value
        self.dd.options = devs
        self.dd.sel = 0
        for i, (idx, _) in enumerate(devs):
            if idx == cur:
                self.dd.sel = i
                break

    # ── drawing ───────────────────────────────────────────────────────────────
    def _bg(self):
        self.screen.fill(BG)
        for x in range(0, W, 40):
            pygame.draw.line(self.screen, (18, 22, 38), (x, 0), (x, H))
        for y in range(0, H, 40):
            pygame.draw.line(self.screen, (18, 22, 38), (0, y), (W, y))

    def _header(self):
        pygame.draw.rect(self.screen, DARK, (0, 0, W, 60))
        pygame.draw.line(self.screen, ACCENT, (0, 60), (W, 60), 1)
        t = self.font_md.render("⚡  NOISE TRIGGER", True, ACCENT)
        self.screen.blit(t, (24, 18))
        for label, key, tx in [("MONITOR", "main", W - 260), ("SETTINGS", "settings", W - 130)]:
            active = self.tab == key
            r = pygame.Rect(tx - 50, 14, 100, 32)
            rrect(self.screen, ACCENT if active else PANEL, r, 6)
            if active:
                rrect(self.screen, ACCENT, r, 6, alpha=30)
            lt = self.font_xs.render(label, True, WHITE if active else LGREY)
            self.screen.blit(lt, (r.centerx - lt.get_width() // 2, r.centery - lt.get_height() // 2))
            self._tab_rects[key] = r

    def _draw_main(self):
        with self.audio.lock:
            db    = self.audio.current_db
            peak  = self.audio.peak_db
            flash = self.audio.trig_flash
            hist  = list(self.audio.history)
            err   = self.audio.error_msg
            rate  = self.audio.active_rate
            running = self.audio.running

        cx = W // 2

        # error banner
        if err or not running:
            msg = err or "Stream not running"
            rrect(self.screen, (60, 10, 10), (20, 70, W - 40, 46), 8)
            pygame.draw.rect(self.screen, ERRCLR, (20, 70, W - 40, 46), 2, border_radius=8)
            et = self.font_sm.render(f"⚠  {msg}", True, ERRCLR)
            self.screen.blit(et, (cx - et.get_width() // 2, 80))
            ht = self.font_xs.render("→ Go to SETTINGS, pick your mic, click APPLY & SAVE", True, LGREY)
            self.screen.blit(ht, (cx - ht.get_width() // 2, 98))

        # big dB number
        dbc = clamp(db, -80, 0)
        col = ACCENT2 if dbc >= self.audio.sensitivity else ACCENT
        big = self.font_lg.render(f"{dbc:+.1f} dB", True, col)
        self.screen.blit(big, (cx - big.get_width() // 2, 130))
        sl = self.font_xs.render("CURRENT LEVEL", True, GREY)
        self.screen.blit(sl, (cx - sl.get_width() // 2, 184))
        if rate:
            rl = self.font_xs.render(f"{rate} Hz  |  sounddevice", True, GREY)
            self.screen.blit(rl, (cx - rl.get_width() // 2, 200))
        tl = self.font_xs.render(f"THRESHOLD  {self.audio.sensitivity:+.0f} dB", True, ACCENT3)
        self.screen.blit(tl, (cx - tl.get_width() // 2, 218))

        # VU bar
        mw, mh, mx, my = 580, 28, cx - 290, 248
        rrect(self.screen, PANEL, (mx, my, mw, mh), 6)
        t  = clamp((dbc + 80) / 80, 0, 1)
        bc = ACCENT if t < 0.6 else ACCENT3 if t < 0.85 else ACCENT2
        fw = int(t * mw)
        if fw > 0:
            rrect(self.screen, bc, (mx, my, fw, mh), 6)
        # threshold tick
        thrt = clamp((self.audio.sensitivity + 80) / 80, 0, 1)
        tx2  = int(mx + thrt * mw)
        pygame.draw.line(self.screen, ACCENT3, (tx2, my - 5), (tx2, my + mh + 5), 2)
        # peak tick
        pt  = clamp((clamp(peak, -80, 0) + 80) / 80, 0, 1)
        px2 = int(mx + pt * mw)
        pygame.draw.line(self.screen, WHITE, (px2, my), (px2, my + mh), 2)

        # history waveform
        wy, wh2, wx, ww = 290, 100, mx, mw
        rrect(self.screen, PANEL, (wx, wy, ww, wh2), 8)
        if len(hist) > 1:
            step = ww / max(len(hist) - 1, 1)
            pts  = [(wx + int(i * step),
                     wy + wh2 - int(clamp((v + 80) / 80, 0, 1) * wh2))
                    for i, v in enumerate(hist)]
            pygame.draw.lines(self.screen, ACCENT, False, pts, 2)
            tpy = wy + wh2 - int(clamp((self.audio.sensitivity + 80) / 80, 0, 1) * wh2)
            s2 = pygame.Surface((ww, 1), pygame.SRCALPHA)
            s2.fill((*ACCENT3, 160))
            self.screen.blit(s2, (wx, tpy))
        wl = self.font_xs.render("LEVEL HISTORY", True, GREY)
        self.screen.blit(wl, (wx + 8, wy + 6))

        # trigger flash panel
        fy, fw2, fh = 408, 580, 76
        fx = cx - fw2 // 2
        rrect(self.screen, ACCENT2, (fx, fy, fw2, fh), 12, alpha=int(flash * 180))
        rrect(self.screen, PANEL,   (fx, fy, fw2, fh), 12, alpha=int((1 - flash) * 200))
        pygame.draw.rect(self.screen,
                         tuple(int(a * flash + b * (1 - flash)) for a, b in zip(ACCENT2, GREY)),
                         (fx, fy, fw2, fh), 2, border_radius=12)
        ic = self.font_lg.render("▶  SPACE", True, WHITE if flash > 0.1 else LGREY)
        self.screen.blit(ic, (fx + fw2 // 2 - ic.get_width() // 2, fy + fh // 2 - ic.get_height() // 2))

        # active toggle
        btn = pygame.Rect(cx - 72, 506, 144, 42)
        rrect(self.screen, ACCENT if self.active else GREY, btn, 10)
        bl = self.font_sm.render("● ACTIVE" if self.active else "○ PAUSED", True, WHITE)
        self.screen.blit(bl, (btn.centerx - bl.get_width() // 2, btn.centery - bl.get_height() // 2))
        self._btn_active = btn

        # stats strip
        sy = 568
        stats = [("PEAK",      f"{clamp(peak,-80,0):+.1f} dB"),
                 ("THRESHOLD", f"{self.audio.sensitivity:+.0f} dB"),
                 ("COOLDOWN",  f"{self.audio.cooldown:.2f}s"),
                 ("STATUS",    "LISTENING" if running else "NO MIC")]
        sw = W // len(stats)
        for i, (k, v) in enumerate(stats):
            sx2 = i * sw + sw // 2
            kt = self.font_xs.render(k, True, GREY)
            vc = ERRCLR if k == "STATUS" and not running else WHITE
            vt = self.font_sm.render(v, True, vc)
            self.screen.blit(kt, (sx2 - kt.get_width() // 2, sy))
            self.screen.blit(vt, (sx2 - vt.get_width() // 2, sy + 18))

        # dB ruler labels under VU bar
        for db_mark in [-80, -60, -40, -20, -10, 0]:
            t_m = clamp((db_mark + 80) / 80, 0, 1)
            xm  = int(mx + t_m * mw)
            lm  = self.font_xs.render(str(db_mark), True, GREY)
            self.screen.blit(lm, (xm - lm.get_width() // 2, my + mh + 4))

    def _draw_settings(self):
        px, py, pw, ph = 30, 80, W - 60, H - 100
        rrect(self.screen, PANEL, (px, py, pw, ph), 14)
        pygame.draw.rect(self.screen, GREY, (px, py, pw, ph), 1, border_radius=14)

        t = self.font_md.render("SETTINGS", True, ACCENT)
        self.screen.blit(t, (px + 24, py + 18))

        # refresh button
        rb = pygame.Rect(px + pw - 170, py + 14, 150, 30)
        rrect(self.screen, GREY, rb, 6)
        rt = self.font_xs.render("↻  REFRESH DEVICES", True, WHITE)
        self.screen.blit(rt, (rb.centerx - rt.get_width() // 2, rb.centery - rt.get_height() // 2))
        self._btn_refresh = rb

        # device dropdown
        dl = self.font_sm.render("Input Device", True, LGREY)
        self.screen.blit(dl, (330, 185))
        self.dd.draw(self.screen, self.font_sm)

        # sliders
        self.sl_sens.draw(self.screen, self.font_sm)
        h1 = self.font_xs.render(
            "Trigger fires when level exceeds this value.  Start at –40 dB, tune up/down.", True, GREY)
        self.screen.blit(h1, (330, 348))

        self.sl_cool.draw(self.screen, self.font_sm)
        h2 = self.font_xs.render(
            "Minimum gap between space-key presses (prevents rapid-fire).", True, GREY)
        self.screen.blit(h2, (330, 443))

        # apply button
        ab = pygame.Rect(330, 470, 200, 44)
        rrect(self.screen, ACCENT, ab, 10)
        al = self.font_sm.render("APPLY & SAVE", True, BLACK)
        self.screen.blit(al, (ab.centerx - al.get_width() // 2, ab.centery - al.get_height() // 2))
        self._btn_apply = ab

        # error
        with self.audio.lock:
            err = self.audio.error_msg
        if err:
            et = self.font_xs.render(f"⚠  {err}", True, ERRCLR)
            self.screen.blit(et, (px + 24, py + ph - 40))

        note = self.font_xs.render(
            "Uses sounddevice (WASAPI on Windows)  ·  raw float32 PCM  ·  no AGC interference", True, ACCENT3)
        self.screen.blit(note, (px + 24, py + ph - 20))

    # ── main loop ─────────────────────────────────────────────────────────────
    def run(self):
        running = True
        while running:
            dt = self.clock.tick(FPS) / 1000.0
            self.audio.tick(dt)

            for ev in pygame.event.get():
                if ev.type == pygame.QUIT:
                    running = False
                if ev.type == pygame.KEYDOWN and ev.key == pygame.K_ESCAPE:
                    running = False

                # tab buttons
                if ev.type == pygame.MOUSEBUTTONDOWN and ev.button == 1:
                    for key, rect in self._tab_rects.items():
                        if rect.collidepoint(ev.pos):
                            self.tab = key

                # main tab
                if self.tab == "main" and ev.type == pygame.MOUSEBUTTONDOWN and ev.button == 1:
                    if self._btn_active.collidepoint(ev.pos):
                        self.active = not self.active
                        if self.active:
                            self.audio.start()
                        else:
                            self.audio.stop()

                # settings tab
                if self.tab == "settings":
                    self.dd.event(ev)
                    self.sl_sens.event(ev)
                    self.sl_cool.event(ev)
                    if ev.type == pygame.MOUSEBUTTONDOWN and ev.button == 1:
                        if self._btn_apply.collidepoint(ev.pos):
                            self._apply_settings()
                            self._save_config()
                            self.tab = "main"
                        if self._btn_refresh.collidepoint(ev.pos):
                            self._refresh_devices()

            # live-sync sliders → audio engine
            self.audio.sensitivity = self.sl_sens.val
            self.audio.cooldown    = self.sl_cool.val

            self._bg()
            self._header()
            if self.tab == "main":
                self._draw_main()
            else:
                self._draw_settings()
            pygame.display.flip()

        self.audio.close()
        self._save_config()
        pygame.quit()
        sys.exit()


if __name__ == "__main__":
    App().run()