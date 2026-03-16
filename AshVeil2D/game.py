import pygame
import math

pygame.init()
screen = pygame.display.set_mode((0, 0), pygame.FULLSCREEN)
SCREEN_W, SCREEN_H = screen.get_size()
pygame.display.set_caption("AshVeil2D")

TILE_SIZE = 32

# ─── PALETTE ────────────────────────────────────────────────────────────────
C_BG        = (18,  20,  26)
C_FLOOR     = (52,  58,  72)
C_FLOOR_ALT = (48,  54,  67)
C_WALL      = (28,  30,  40)
C_WALL_TOP  = (42,  45,  58)
C_WINDOW    = (80, 140, 180)
C_WINDOW_G  = (130, 200, 230)
C_DOOR      = (140,  85,  35)
C_DOOR_LT   = (180, 120,  55)
C_VOID      = (12,  13,  18)
C_PLAYER    = (220, 200, 160)   # skin
C_SHIRT     = (60,  90, 140)    # shirt
C_PANTS     = (35,  45,  65)    # pants
C_HAIR      = (40,  30,  20)    # hair
C_UI_BG     = (10,  12,  18, 200)
C_UI_ACCENT = (80, 140, 200)
C_UI_TEXT   = (210, 215, 225)
C_UI_DIM    = (90,  95, 110)

# ─── TILE IDs ────────────────────────────────────────────────────────────────
FLOOR  = 0
WALL   = 1
WINDOW = 2
DOOR   = 3
VOID   = 4

TILE_DATA = {
    FLOOR:  (C_FLOOR,  False),
    WALL:   (C_WALL,   True),
    WINDOW: (C_WINDOW, True),
    DOOR:   (C_DOOR,   False),
    VOID:   (C_VOID,   False),
}
TILE_NAMES = {FLOOR:"floor", WALL:"wall", WINDOW:"window", DOOR:"door", VOID:"void"}

# ─── MAP SETUP ───────────────────────────────────────────────────────────────
ROOM_W = 45
ROOM_H = 65
CORRIDOR = 25
ROOMS_PER_SIDE = 3

side        = 1 + ROOM_H + 1 + CORRIDOR
rooms_span  = ROOMS_PER_SIDE * ROOM_W + (ROOMS_PER_SIDE + 1)
MAP_W = side + rooms_span + side + 1
MAP_H = side + rooms_span + side + 1
MAP   = [FLOOR] * (MAP_W * MAP_H)

def set_tile(x,y,t):
    if 0<=x<MAP_W and 0<=y<MAP_H: MAP[y*MAP_W+x]=t
def get_tile(x,y):
    if 0<=x<MAP_W and 0<=y<MAP_H: return MAP[y*MAP_W+x]
    return WALL
def fill_rect(x,y,w,h,t):
    for ty in range(y,y+h):
        for tx in range(x,x+w): set_tile(tx,ty,t)
def draw_hline(x1,x2,y,t=WALL):
    for x in range(min(x1,x2),max(x1,x2)+1): set_tile(x,y,t)
def draw_vline(x,y1,y2,t=WALL):
    for y in range(min(y1,y2),max(y1,y2)+1): set_tile(x,y,t)
def draw_rect_outline(x,y,w,h,t=WALL):
    draw_hline(x,x+w-1,y,t); draw_hline(x,x+w-1,y+h-1,t)
    draw_vline(x,y,y+h-1,t); draw_vline(x+w-1,y,y+h-1,t)

# ─── COMPUTE KEY COORDS ──────────────────────────────────────────────────────
room_wall_left  = 1 + ROOM_H
corr_left       = room_wall_left + 1
inner_left      = corr_left + CORRIDOR
void_left       = inner_left + 1
void_right      = MAP_W - 1 - (1 + ROOM_H + 1 + CORRIDOR + 1)
inner_right     = void_right - 1
corr_right      = inner_right - CORRIDOR
room_wall_right = corr_right - 1
room_right      = room_wall_right + 1

room_wall_top    = 1 + ROOM_H
corr_top         = room_wall_top + 1
inner_top        = corr_top + CORRIDOR
void_top         = inner_top + 1
void_bottom      = MAP_H - 1 - (1 + ROOM_H + 1 + CORRIDOR + 1)
inner_bottom     = void_bottom - 1
corr_bottom      = inner_bottom - CORRIDOR
room_wall_bottom = corr_bottom - 1
room_bottom      = room_wall_bottom + 1

# ─── BUILD MAP ───────────────────────────────────────────────────────────────
draw_rect_outline(0,0,MAP_W,MAP_H,WALL)
draw_rect_outline(inner_left,inner_top,inner_right-inner_left+1,inner_bottom-inner_top+1,WALL)
fill_rect(void_left,void_top,void_right-void_left+1,void_bottom-void_top+1,VOID)

for x in range(inner_left,inner_right+1):
    if x%4==0: set_tile(x,inner_top,WINDOW); set_tile(x,inner_bottom,WINDOW)
for y in range(inner_top,inner_bottom+1):
    if y%4==0: set_tile(inner_left,y,WINDOW); set_tile(inner_right,y,WINDOW)

draw_hline(0,MAP_W-1,room_wall_top,WALL)
draw_hline(0,MAP_W-1,room_wall_bottom,WALL)
draw_vline(room_wall_left,0,MAP_H-1,WALL)
draw_vline(room_wall_right,0,MAP_H-1,WALL)

fill_rect(1,corr_top,MAP_W-2,CORRIDOR,FLOOR)
fill_rect(1,corr_bottom,MAP_W-2,CORRIDOR,FLOOR)
fill_rect(corr_left,1,CORRIDOR,MAP_H-2,FLOOR)
fill_rect(corr_right,1,CORRIDOR,MAP_H-2,FLOOR)
fill_rect(1,1,MAP_W-2,ROOM_H,FLOOR)
fill_rect(1,room_bottom,MAP_W-2,ROOM_H,FLOOR)
fill_rect(1,1,ROOM_H,MAP_H-2,FLOOR)
fill_rect(room_right,1,ROOM_H,MAP_H-2,FLOOR)

draw_hline(0,MAP_W-1,room_wall_top,WALL)
draw_hline(0,MAP_W-1,room_wall_bottom,WALL)
draw_vline(room_wall_left,0,MAP_H-1,WALL)
draw_vline(room_wall_right,0,MAP_H-1,WALL)

for i in range(ROOMS_PER_SIDE):
    rx = room_wall_left+1+i*(ROOM_W+1)
    if i>0: draw_vline(rx-1,0,room_wall_top,WALL); draw_vline(rx-1,room_wall_bottom,MAP_H-1,WALL)
    set_tile(rx+ROOM_W//2,room_wall_top,DOOR)
    set_tile(rx+ROOM_W//2,room_wall_bottom,DOOR)

for i in range(ROOMS_PER_SIDE):
    ry = room_wall_top+1+i*(ROOM_W+1)
    if i>0: draw_hline(0,room_wall_left,ry-1,WALL); draw_hline(room_wall_right,MAP_W-1,ry-1,WALL)
    set_tile(room_wall_left,ry+ROOM_W//2,DOOR)
    set_tile(room_wall_right,ry+ROOM_W//2,DOOR)

ex = inner_left+(inner_right-inner_left)//2
for dx in (-1,0,1): set_tile(ex+dx,inner_bottom,DOOR)


# ─── TILE SURFACE CACHE ──────────────────────────────────────────────────────
def make_tile_surface(tile_id):
    surf = pygame.Surface((TILE_SIZE, TILE_SIZE))
    if tile_id == FLOOR:
        # checkerboard floor
        surf.fill(C_FLOOR)
        pygame.draw.rect(surf, C_FLOOR_ALT, (0,0,TILE_SIZE,TILE_SIZE))
        # subtle grid line
        pygame.draw.line(surf, (58,65,80), (0,0),(TILE_SIZE-1,0))
        pygame.draw.line(surf, (58,65,80), (0,0),(0,TILE_SIZE-1))
    elif tile_id == WALL:
        surf.fill(C_WALL)
        pygame.draw.rect(surf, C_WALL_TOP, (0,0,TILE_SIZE,4))
        pygame.draw.rect(surf, (20,22,30), (0,TILE_SIZE-2,TILE_SIZE,2))
    elif tile_id == WINDOW:
        surf.fill(C_WALL)
        inner = pygame.Rect(4,4,TILE_SIZE-8,TILE_SIZE-8)
        pygame.draw.rect(surf, C_WINDOW, inner)
        pygame.draw.rect(surf, C_WINDOW_G, inner, 1)
        pygame.draw.line(surf, C_WINDOW_G, (TILE_SIZE//2,4),(TILE_SIZE//2,TILE_SIZE-4))
        pygame.draw.line(surf, C_WINDOW_G, (4,TILE_SIZE//2),(TILE_SIZE-4,TILE_SIZE//2))
    elif tile_id == DOOR:
        surf.fill(C_DOOR)
        pygame.draw.rect(surf, C_DOOR_LT, (4,2,TILE_SIZE-8,TILE_SIZE-4))
        pygame.draw.circle(surf, (220,180,80), (TILE_SIZE-7, TILE_SIZE//2), 3)
    elif tile_id == VOID:
        surf.fill(C_VOID)
        for i in range(0,TILE_SIZE,8):
            pygame.draw.line(surf,(20,22,30),(0,i),(i,0))
    return surf

TILE_SURFS = {t: make_tile_surface(t) for t in TILE_DATA}


# ─── PLAYER SPRITE ───────────────────────────────────────────────────────────
def make_player_sprite(direction="down"):
    surf = pygame.Surface((TILE_SIZE, TILE_SIZE), pygame.SRCALPHA)
    # shadow
    pygame.draw.ellipse(surf, (0,0,0,60), (6, TILE_SIZE-8, 20, 6))
    # body (shirt)
    pygame.draw.rect(surf, C_SHIRT, (8, 14, 16, 12), border_radius=3)
    # pants
    pygame.draw.rect(surf, C_PANTS, (9, 22, 6, 8), border_radius=2)
    pygame.draw.rect(surf, C_PANTS, (17, 22, 6, 8), border_radius=2)
    # skin - arms
    if direction in ("down","up"):
        pygame.draw.rect(surf, C_PLAYER, (5, 14, 4, 9), border_radius=2)
        pygame.draw.rect(surf, C_PLAYER, (23, 14, 4, 9), border_radius=2)
    # head
    pygame.draw.ellipse(surf, C_PLAYER, (9, 4, 14, 13))
    # hair
    pygame.draw.ellipse(surf, C_HAIR, (9, 4, 14, 7))
    # face
    if direction == "down":
        pygame.draw.circle(surf, (30,20,15), (14,12), 1)
        pygame.draw.circle(surf, (30,20,15), (18,12), 1)
        pygame.draw.arc(surf, (180,100,80), (13,13,6,4), math.pi, 2*math.pi, 1)
    elif direction == "up":
        pass  # back of head
    return surf

PLAYER_SPRITES = {d: make_player_sprite(d) for d in ("down","up","left","right")}


# ─── WORLD ───────────────────────────────────────────────────────────────────
class World:
    def __init__(self):
        self.surface = pygame.Surface((MAP_W*TILE_SIZE, MAP_H*TILE_SIZE))
        self.solids  = []
        self._build()

    def _build(self):
        self.surface.fill(C_BG)
        for ty in range(MAP_H):
            for tx in range(MAP_W):
                tile = get_tile(tx,ty)
                color, solid = TILE_DATA[tile]
                px, py = tx*TILE_SIZE, ty*TILE_SIZE
                self.surface.blit(TILE_SURFS[tile], (px,py))
                if solid:
                    self.solids.append(pygame.Rect(px,py,TILE_SIZE,TILE_SIZE))

    def draw(self, screen, cam_x, cam_y):
        screen.blit(self.surface, (-cam_x, -cam_y))

    def get_nearby_solids(self, x, y, r=200):
        return [s for s in self.solids if abs(s.x-x)<r and abs(s.y-y)<r]


# ─── PLAYER ──────────────────────────────────────────────────────────────────
class Player:
    def __init__(self, x, y):
        self.x     = float(x)
        self.y     = float(y)
        self.speed = 4
        self.facing = "down"
        self.walk_frame = 0
        self.walk_timer = 0

    def move(self, keys, solids):
        dx = dy = 0
        if keys[pygame.K_w] or keys[pygame.K_UP]:    dy -= 1; self.facing="up"
        if keys[pygame.K_s] or keys[pygame.K_DOWN]:  dy += 1; self.facing="down"
        if keys[pygame.K_a] or keys[pygame.K_LEFT]:  dx -= 1; self.facing="left"
        if keys[pygame.K_d] or keys[pygame.K_RIGHT]: dx += 1; self.facing="right"

        speed = 8 if (keys[pygame.K_LSHIFT] or keys[pygame.K_RSHIFT]) else self.speed
        moving = dx!=0 or dy!=0

        if moving:
            self.walk_timer += 1
            if self.walk_timer >= 8:
                self.walk_frame = (self.walk_frame+1) % 2
                self.walk_timer = 0

        if dx!=0 and dy!=0:
            dx *= 0.7071; dy *= 0.7071

        self.x += dx*speed
        rect = pygame.Rect(self.x, self.y, 28, 28)
        for w in solids:
            if rect.colliderect(w):
                self.x = w.right if dx<0 else w.left-28
                break

        self.y += dy*speed
        rect = pygame.Rect(self.x, self.y, 28, 28)
        for w in solids:
            if rect.colliderect(w):
                self.y = w.bottom if dy<0 else w.top-28
                break

        return moving

    def draw(self, screen, cam_x, cam_y):
        sprite = PLAYER_SPRITES.get(self.facing, PLAYER_SPRITES["down"])
        # slight bob when walking
        bob = math.sin(self.walk_frame * math.pi) * 1.5 if self.walk_frame else 0
        screen.blit(sprite, (self.x - cam_x, self.y - cam_y + bob))


# ─── HUD ─────────────────────────────────────────────────────────────────────
def draw_hud(screen, clock, player, font, small_font):
    fps    = int(clock.get_fps())
    tile_x = int(player.x // TILE_SIZE)
    tile_y = int(player.y // TILE_SIZE)
    tname  = TILE_NAMES.get(get_tile(tile_x, tile_y), "?")

    bar_h = 32
    bar_surf = pygame.Surface((SCREEN_W, bar_h), pygame.SRCALPHA)
    bar_surf.fill((8,10,16,210))
    screen.blit(bar_surf, (0,0))
    pygame.draw.line(screen, (40,50,70), (0,bar_h), (SCREEN_W,bar_h))

    # FPS badge
    fps_color = (80,200,100) if fps>=55 else (220,180,60) if fps>=30 else (220,80,80)
    fps_text = small_font.render(f"FPS {fps}", True, fps_color)
    screen.blit(fps_text, (12, 8))

    # Position
    pos_text = small_font.render(f"({tile_x}, {tile_y})", True, C_UI_DIM)
    screen.blit(pos_text, (SCREEN_W//2 - pos_text.get_width()//2, 8))

    # Tile name
    tile_text = small_font.render(tname.upper(), True, C_UI_ACCENT)
    screen.blit(tile_text, (SCREEN_W - tile_text.get_width() - 12, 8))

    # Controls hint (bottom)
    hint_surf = pygame.Surface((SCREEN_W, 24), pygame.SRCALPHA)
    hint_surf.fill((8,10,16,160))
    screen.blit(hint_surf, (0, SCREEN_H-24))
    hints = small_font.render("WASD / ↑↓←→  Move     SHIFT  Run     ESC  Menu", True, (55,60,75))
    screen.blit(hints, (SCREEN_W//2 - hints.get_width()//2, SCREEN_H-20))


# ─── MENU ────────────────────────────────────────────────────────────────────
def menu_screen(clock):
    title_font = pygame.font.SysFont("monospace", 68, bold=True)
    sub_font   = pygame.font.SysFont("monospace", 20)
    menu_font  = pygame.font.SysFont("monospace", 30)

    options  = ["Play", "Quit"]
    selected = 0
    tick     = 0

    while True:
        tick += 1
        for event in pygame.event.get():
            if event.type == pygame.QUIT: return "quit"
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_ESCAPE: return "quit"
                if event.key in (pygame.K_w, pygame.K_UP):
                    selected = (selected-1) % len(options)
                if event.key in (pygame.K_s, pygame.K_DOWN):
                    selected = (selected+1) % len(options)
                if event.key in (pygame.K_RETURN, pygame.K_SPACE):
                    return options[selected].lower()

        screen.fill(C_BG)

        # animated grid
        for i in range(0, SCREEN_W, 40):
            alpha = int(20 + 10*math.sin(tick*0.02 + i*0.05))
            pygame.draw.line(screen, (20,25,35), (i,0),(i,SCREEN_H))
        for j in range(0, SCREEN_H, 40):
            pygame.draw.line(screen, (20,25,35), (0,j),(SCREEN_W,j))

        # title glow
        glow = pygame.Surface((420,90), pygame.SRCALPHA)
        g = int(30 + 15*math.sin(tick*0.03))
        pygame.draw.ellipse(glow, (C_UI_ACCENT[0]//4, C_UI_ACCENT[1]//4, C_UI_ACCENT[2]//4, g), (0,0,420,90))
        screen.blit(glow, (SCREEN_W//2-210, SCREEN_H//3-30))

        # title
        title = title_font.render("AshVeil2D", True, (220,225,235))
        screen.blit(title, (SCREEN_W//2 - title.get_width()//2, SCREEN_H//3 - 20))

        # subtitle
        sub = sub_font.render("A School Adventure", True, C_UI_DIM)
        screen.blit(sub, (SCREEN_W//2 - sub.get_width()//2, SCREEN_H//3 + 58))

        # divider
        dw = 200
        pygame.draw.line(screen, C_UI_ACCENT,
                         (SCREEN_W//2-dw//2, SCREEN_H//3+85),
                         (SCREEN_W//2+dw//2, SCREEN_H//3+85), 1)

        # options
        for i, opt in enumerate(options):
            active = i == selected
            col    = (230,235,245) if active else C_UI_DIM
            prefix = "▶  " if active else "   "
            text   = menu_font.render(prefix + opt, True, col)
            y      = SCREEN_H//2 + 30 + i*52
            if active:
                hl = pygame.Surface((text.get_width()+20, 38), pygame.SRCALPHA)
                hl.fill((C_UI_ACCENT[0],C_UI_ACCENT[1],C_UI_ACCENT[2],25))
                screen.blit(hl, (SCREEN_W//2-text.get_width()//2-10, y-4))
            screen.blit(text, (SCREEN_W//2-text.get_width()//2, y))

        # footer
        footer = sub_font.render("W/S  Navigate     Enter  Select", True, (40,45,58))
        screen.blit(footer, (SCREEN_W//2-footer.get_width()//2, SCREEN_H-45))

        pygame.display.flip()
        clock.tick(60)


# ─── GAME LOOP ───────────────────────────────────────────────────────────────
def game_loop(clock):
    font       = pygame.font.SysFont("monospace", 18)
    small_font = pygame.font.SysFont("monospace", 15)

    world  = World()
    player = Player(
        (corr_left + CORRIDOR//2) * TILE_SIZE,
        (corr_top  + CORRIDOR//2) * TILE_SIZE
    )

    try:
        pygame.mixer.music.load("Assets/Fog Over Lumen Town.mp3")
        pygame.mixer.music.set_volume(0.6)
        pygame.mixer.music.play(-1)
    except Exception:
        pass

    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT: return
            if event.type == pygame.KEYDOWN and event.key == pygame.K_ESCAPE: return

        keys   = pygame.key.get_pressed()
        solids = world.get_nearby_solids(player.x, player.y)
        player.move(keys, solids)

        cam_x = int(player.x - SCREEN_W//2 + 14)
        cam_y = int(player.y - SCREEN_H//2 + 14)

        screen.fill(C_BG)
        world.draw(screen, cam_x, cam_y)
        player.draw(screen, cam_x, cam_y)
        draw_hud(screen, clock, player, font, small_font)

        pygame.display.flip()
        clock.tick(60)


# ─── MAIN ────────────────────────────────────────────────────────────────────
clock = pygame.time.Clock()

try:
    pygame.mixer.music.load("Assets/Enigmatic Little Town.mp3")
    pygame.mixer.music.set_volume(0.5)
    pygame.mixer.music.play(-1)
except Exception:
    pass

choice = menu_screen(clock)
if choice == "play":
    game_loop(clock)

pygame.quit()