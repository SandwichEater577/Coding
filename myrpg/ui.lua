-- ui.lua  –  Title screen, pause menu, HUD, minimap, inventory, game over
local UI = {}
UI.__index = UI

function UI.new()
    local self = setmetatable({}, UI)
    self.notifications = {}
    self.titleSelected = 1
    self.titleOptions  = {"New Game", "Controls", "Quit"}
    self.pauseSelected = 1
    self.pauseOptions  = {"Resume", "Inventory", "Controls", "Quit to Title"}
    self.showControls  = false
    return self
end

function UI:notify(text, dur)
    table.insert(self.notifications, {text=text, timer=dur or 3})
end

function UI:update(dt)
    for i = #self.notifications, 1, -1 do
        self.notifications[i].timer = self.notifications[i].timer - dt
        if self.notifications[i].timer <= 0 then table.remove(self.notifications, i) end
    end
end

---------------------------------------------------------------------------
-- Title Screen
---------------------------------------------------------------------------
function UI:drawTitle()
    local W = love.graphics.getWidth()
    local H = love.graphics.getHeight()
    local time = love.timer.getTime()

    -- Dark background with drifting particles
    love.graphics.setColor(0.06,0.05,0.08)
    love.graphics.rectangle("fill",0,0,W,H)

    -- Ground
    love.graphics.setColor(0.15,0.12,0.10)
    love.graphics.rectangle("fill",0,H*0.65,W,H*0.35)

    -- Stars
    love.graphics.setColor(1,1,1,0.5)
    math.randomseed(123)
    for _=1,50 do
        local sx = math.random(W)
        local sy = math.random(math.floor(H*0.65))
        local blink = 0.3+0.7*math.abs(math.sin(time*0.5+sx*0.1))
        love.graphics.setColor(1,1,1,blink*0.6)
        love.graphics.circle("fill",sx,sy,1)
    end
    math.randomseed(os.time())

    -- Atmospheric particles (dust)
    love.graphics.setColor(0.5,0.4,0.3,0.15)
    for i=1,20 do
        local px = (i*73 + time*15) % W
        local py = H*0.5 + math.sin(time*0.3 + i)*40
        love.graphics.circle("fill",px,py,2)
    end

    -- Ruined silhouette buildings
    love.graphics.setColor(0.10,0.08,0.07)
    love.graphics.polygon("fill", 100,H*0.65, 100,H*0.45, 130,H*0.42, 160,H*0.45, 160,H*0.65)
    love.graphics.polygon("fill", 200,H*0.65, 200,H*0.38, 220,H*0.35, 260,H*0.36, 280,H*0.65)
    love.graphics.polygon("fill", 700,H*0.65, 700,H*0.40, 750,H*0.37, 780,H*0.65)
    love.graphics.polygon("fill", 850,H*0.65, 850,H*0.42, 870,H*0.38, 900,H*0.40, 920,H*0.65)

    -- Title
    love.graphics.setColor(0.9,0.75,0.4)
    love.graphics.printf("WASTELAND", 0, H*0.15, W, "center")
    love.graphics.setColor(0.7,0.55,0.3)
    love.graphics.printf("C H R O N I C L E S", 0, H*0.15+30, W, "center")

    -- Subtitle
    love.graphics.setColor(0.5,0.5,0.45,0.5+0.3*math.sin(time*2))
    love.graphics.printf("A Post-Apocalyptic RPG", 0, H*0.15+60, W, "center")

    -- Menu
    if self.showControls then
        self:drawControlsOverlay()
    else
        for i, opt in ipairs(self.titleOptions) do
            local y = H*0.50 + (i-1)*40
            if i == self.titleSelected then
                love.graphics.setColor(1,0.85,0.3)
                love.graphics.printf("> "..opt.." <", 0, y, W, "center")
            else
                love.graphics.setColor(0.6,0.6,0.55)
                love.graphics.printf(opt, 0, y, W, "center")
            end
        end

        -- Footer
        love.graphics.setColor(0.35,0.35,0.3)
        love.graphics.printf("Use W/S or Up/Down to navigate  |  Enter to select", 0, H-40, W, "center")
    end
    love.graphics.setColor(1,1,1)
end

function UI:titleKeypressed(key)
    if self.showControls then
        if key == "escape" or key == "return" or key == "space" then
            self.showControls = false
        end
        return nil
    end
    if key == "up" or key == "w" then
        self.titleSelected = math.max(1, self.titleSelected-1)
    elseif key == "down" or key == "s" then
        self.titleSelected = math.min(#self.titleOptions, self.titleSelected+1)
    elseif key == "return" or key == "space" then
        local opt = self.titleOptions[self.titleSelected]
        if opt == "New Game" then return "start"
        elseif opt == "Controls" then self.showControls = true
        elseif opt == "Quit" then love.event.quit()
        end
    end
    return nil
end

---------------------------------------------------------------------------
-- Pause Menu
---------------------------------------------------------------------------
function UI:drawPause()
    local W = love.graphics.getWidth()
    local H = love.graphics.getHeight()

    love.graphics.setColor(0,0,0,0.7)
    love.graphics.rectangle("fill",0,0,W,H)

    if self.showControls then
        self:drawControlsOverlay()
        return
    end

    love.graphics.setColor(0.1,0.1,0.12,0.95)
    love.graphics.rectangle("fill",W/2-150,H/2-120,300,240,8,8)
    love.graphics.setColor(0.5,0.45,0.35)
    love.graphics.rectangle("line",W/2-150,H/2-120,300,240,8,8)

    love.graphics.setColor(1,0.85,0.4)
    love.graphics.printf("PAUSED", 0, H/2-100, W, "center")

    for i, opt in ipairs(self.pauseOptions) do
        local y = H/2-55 + (i-1)*38
        if i == self.pauseSelected then
            love.graphics.setColor(1,0.85,0.3)
            love.graphics.printf("> "..opt, 0, y, W, "center")
        else
            love.graphics.setColor(0.65,0.65,0.60)
            love.graphics.printf(opt, 0, y, W, "center")
        end
    end
    love.graphics.setColor(1,1,1)
end

function UI:pauseKeypressed(key)
    if self.showControls then
        if key == "escape" or key == "return" or key == "space" then
            self.showControls = false
        end
        return nil
    end
    if key == "up" or key == "w" then
        self.pauseSelected = math.max(1, self.pauseSelected-1)
    elseif key == "down" or key == "s" then
        self.pauseSelected = math.min(#self.pauseOptions, self.pauseSelected+1)
    elseif key == "escape" then
        return "resume"
    elseif key == "return" or key == "space" then
        local opt = self.pauseOptions[self.pauseSelected]
        if opt == "Resume" then return "resume"
        elseif opt == "Inventory" then return "inventory"
        elseif opt == "Controls" then self.showControls = true
        elseif opt == "Quit to Title" then return "title"
        end
    end
    return nil
end

---------------------------------------------------------------------------
-- Controls overlay
---------------------------------------------------------------------------
function UI:drawControlsOverlay()
    local W = love.graphics.getWidth()
    local H = love.graphics.getHeight()

    love.graphics.setColor(0.08,0.08,0.10,0.95)
    love.graphics.rectangle("fill",W/2-250,60,500,H-120,8,8)
    love.graphics.setColor(0.5,0.45,0.35)
    love.graphics.rectangle("line",W/2-250,60,500,H-120,8,8)

    love.graphics.setColor(1,0.85,0.4)
    love.graphics.printf("CONTROLS", 0, 80, W, "center")

    local controls = {
        "",
        "EXPLORATION",
        "  W/A/S/D or Arrows   Move",
        "  Shift                Sprint",
        "  E                    Interact / Loot",
        "  I                    Inventory",
        "  Escape               Pause Menu",
        "",
        "COMBAT",
        "  W/S or Up/Down       Navigate menu",
        "  Enter / Space        Confirm",
        "  Escape               Back",
        "",
        "TIPS",
        "  - Enemy attacks are telegraphed",
        "  - Use Analyse to learn patterns",
        "  - Guard before heavy attacks",
        "  - Rest at campfires to heal",
        "  - You respawn at last campfire",
    }
    love.graphics.setColor(0.85,0.85,0.8)
    for i, line in ipairs(controls) do
        if line == "EXPLORATION" or line == "COMBAT" or line == "TIPS" then
            love.graphics.setColor(1,0.7,0.3)
        else
            love.graphics.setColor(0.85,0.85,0.8)
        end
        love.graphics.print(line, W/2-210, 105 + i*22)
    end

    love.graphics.setColor(0.5,0.5,0.5)
    love.graphics.printf("[Esc / Enter to close]", 0, H-80, W, "center")
    love.graphics.setColor(1,1,1)
end

---------------------------------------------------------------------------
-- HUD
---------------------------------------------------------------------------
function UI:drawHUD(player)
    local W = love.graphics.getWidth()

    love.graphics.setColor(0,0,0,0.6)
    love.graphics.rectangle("fill",0,0,W,52)

    -- HP
    love.graphics.setColor(1,1,1)
    love.graphics.print("HP",10,6)
    local hpR = player.hp/player.maxHP
    love.graphics.setColor(0.2,0.2,0.2)
    love.graphics.rectangle("fill",35,6,160,14,3,3)
    if hpR > 0.5 then love.graphics.setColor(0.2,0.75,0.25)
    elseif hpR > 0.25 then love.graphics.setColor(0.9,0.7,0.1)
    else love.graphics.setColor(0.9,0.2,0.1) end
    love.graphics.rectangle("fill",35,6,160*hpR,14,3,3)
    love.graphics.setColor(1,1,1)
    love.graphics.print(player.hp.."/"..player.maxHP,200,5)

    -- AP
    love.graphics.print("AP",10,26)
    love.graphics.setColor(0.2,0.2,0.2)
    love.graphics.rectangle("fill",35,26,120,10,3,3)
    love.graphics.setColor(0.25,0.5,0.9)
    love.graphics.rectangle("fill",35,26,120*(player.ap/player.maxAP),10,3,3)

    -- Level/XP
    love.graphics.setColor(1,1,1)
    love.graphics.print("Lv."..player.level,280,6)
    love.graphics.setColor(0.7,0.7,0.7)
    love.graphics.print("XP:"..player.xp.."/"..player.xpNext,280,24)
    love.graphics.setColor(0.6,0.6,0.55)
    love.graphics.print("Kills:"..player.kills,380,6)

    -- Radiation
    if player.radiation > 0 then
        love.graphics.setColor(0.4,0.9,0.1,0.5+0.3*math.sin(love.timer.getTime()*5))
        love.graphics.print("RAD:"..math.floor(player.radiation).."%",380,24)
    end

    -- Weapon
    if player.equippedWeapon then
        love.graphics.setColor(0.9,0.85,0.7)
        love.graphics.print(player.equippedWeapon.name.." (ATK+"..player.equippedWeapon.atk..")",480,6)
    end

    -- Controls hint
    love.graphics.setColor(0.45,0.45,0.4)
    love.graphics.print("WASD:Move  E:Interact/Loot  I:Inventory  Esc:Pause",W-420,36)

    love.graphics.setColor(1,1,1)
end

---------------------------------------------------------------------------
-- Minimap
---------------------------------------------------------------------------
function UI:drawMinimap(map, player, npcs, roamers)
    local W = love.graphics.getWidth()
    local mmSize = 120
    local mmX, mmY = W-mmSize-15, 58
    local scale = mmSize/math.max(map.width,map.height)

    love.graphics.setColor(0,0,0,0.7)
    love.graphics.rectangle("fill",mmX-2,mmY-2,mmSize+4,mmSize+4,4,4)

    for y=1,map.height do for x=1,map.width do
        local c = map.TILE_COLOURS[map.tiles[y][x]]
        if c then
            love.graphics.setColor(c[1],c[2],c[3],0.8)
            love.graphics.rectangle("fill",mmX+(x-1)*scale,mmY+(y-1)*scale,
                math.max(1,scale),math.max(1,scale))
        end
    end end

    -- Campfires
    for _, cf in ipairs(map.campfires) do
        love.graphics.setColor(1,0.5,0.1)
        love.graphics.circle("fill",mmX+(cf.x-1)*scale+scale/2,mmY+(cf.y-1)*scale+scale/2,2.5)
    end

    if npcs then
        for _, n in ipairs(npcs) do
            love.graphics.setColor(1,0.9,0.2)
            love.graphics.circle("fill",mmX+(n.tileX-1)*scale+scale/2,mmY+(n.tileY-1)*scale+scale/2,2)
        end
    end
    if roamers then
        for _, r in ipairs(roamers) do
            love.graphics.setColor(1,0.2,0.2)
            local rx = math.floor(r.x/map.tileSize)+1
            local ry = math.floor(r.y/map.tileSize)+1
            love.graphics.circle("fill",mmX+(rx-1)*scale+scale/2,mmY+(ry-1)*scale+scale/2,2)
        end
    end

    local ptx,pty = player:getTilePos()
    love.graphics.setColor(0,1,0.5)
    love.graphics.circle("fill",mmX+(ptx-1)*scale+scale/2,mmY+(pty-1)*scale+scale/2,3)

    -- Landmark labels
    love.graphics.setColor(1,1,1,0.6)
    for _, lm in ipairs(map.landmarks) do
        love.graphics.print(lm.name, mmX+(lm.x-1)*scale-15, mmY+(lm.y-1)*scale-12)
    end

    love.graphics.setColor(1,1,1)
end

---------------------------------------------------------------------------
-- Notifications
---------------------------------------------------------------------------
function UI:drawNotifications()
    local W = love.graphics.getWidth()
    for i, n in ipairs(self.notifications) do
        local a = math.min(1, n.timer)
        love.graphics.setColor(0,0,0,0.7*a)
        love.graphics.rectangle("fill",W/2-200,60+(i-1)*35,400,28,4,4)
        love.graphics.setColor(1,0.9,0.5,a)
        love.graphics.printf(n.text,W/2-195,65+(i-1)*35,390,"center")
    end
    love.graphics.setColor(1,1,1)
end

---------------------------------------------------------------------------
-- Inventory screen
---------------------------------------------------------------------------
function UI:drawInventory(player, sel)
    local W = love.graphics.getWidth()
    local H = love.graphics.getHeight()

    love.graphics.setColor(0,0,0,0.85)
    love.graphics.rectangle("fill",0,0,W,H)

    love.graphics.setColor(1,0.85,0.3)
    love.graphics.printf("INVENTORY",0,30,W,"center")

    -- Stats
    love.graphics.setColor(0.1,0.1,0.12,0.9)
    love.graphics.rectangle("fill",30,70,280,280,6,6)
    love.graphics.setColor(0.5,0.45,0.35)
    love.graphics.rectangle("line",30,70,280,280,6,6)

    love.graphics.setColor(1,0.9,0.5)
    love.graphics.print(player.name.."  Level "..player.level,50,85)
    love.graphics.setColor(0.8,0.8,0.8)
    local stats = {
        "HP:  "..player.hp.." / "..player.maxHP,
        "AP:  "..player.ap.." / "..player.maxAP,
        "STR: "..player.str.."  (melee damage)",
        "AGI: "..player.agi.."  (escape chance)",
        "END: "..player.end_.."  (HP growth)",
        "PER: "..player.per.."  (crit rate)",
        "LCK: "..player.lck.."  (loot quality)",
        "DEF: "..player.defense,
        "RAD: "..math.floor(player.radiation).."%",
        "",
        "Kills: "..player.kills,
        "Crates: "..player.cratesLooted,
    }
    for i, s in ipairs(stats) do
        love.graphics.print(s, 50, 110+(i-1)*22)
    end

    -- Items
    love.graphics.setColor(0.1,0.1,0.12,0.9)
    love.graphics.rectangle("fill",340,70,W-370,H-120,6,6)
    love.graphics.setColor(0.5,0.45,0.35)
    love.graphics.rectangle("line",340,70,W-370,H-120,6,6)

    love.graphics.setColor(1,0.9,0.5)
    love.graphics.print("Items ("..#player.inventory..")",360,85)

    for i, item in ipairs(player.inventory) do
        local y = 110+(i-1)*28
        if i == sel then
            love.graphics.setColor(1,0.9,0.3)
            love.graphics.print("> "..item.name,360,y)
            love.graphics.setColor(0.6,0.6,0.55)
            local extra = ""
            if item.atk then extra = "  ATK+"..item.atk end
            if item.heal then extra = "  Heal+"..item.heal end
            love.graphics.printf((item.desc or "")..extra, 360, H-80, W-400)
        else
            local col = {0.7,0.7,0.7}
            if item.type=="weapon" then col={0.9,0.5,0.3} end
            if item.type=="consumable" then col={0.4,0.8,0.4} end
            if item.type=="junk" then col={0.5,0.5,0.45} end
            love.graphics.setColor(col)
            love.graphics.print("  "..item.name,360,y)
        end
    end

    if #player.inventory == 0 then
        love.graphics.setColor(0.5,0.5,0.5)
        love.graphics.print("  (empty)",360,110)
    end

    love.graphics.setColor(0.5,0.5,0.5)
    love.graphics.printf("Up/Down:navigate  Enter:use/equip  I/Esc:close",0,H-35,W,"center")
    love.graphics.setColor(1,1,1)
end

---------------------------------------------------------------------------
-- Game Over
---------------------------------------------------------------------------
function UI:drawGameOver()
    local W = love.graphics.getWidth()
    local H = love.graphics.getHeight()
    love.graphics.setColor(0.05,0,0,0.9)
    love.graphics.rectangle("fill",0,0,W,H)
    love.graphics.setColor(0.8,0.1,0.1)
    love.graphics.printf("YOU HAVE FALLEN",0,H/2-40,W,"center")
    love.graphics.setColor(0.6,0.5,0.4)
    love.graphics.printf("But the wasteland is forgiving... this time.\n\nYou respawn at your last campfire.\n\nPress R to continue.",0,H/2,W,"center")
    love.graphics.setColor(1,1,1)
end

return UI
