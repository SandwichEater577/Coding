-- Wasteland Chronicles  –  LÖVE2D Post-Apocalyptic RPG
-- Strategic combat, checkpoints, menus, deterministic gameplay
local Map      = require("map")
local Player   = require("player")
local Camera   = require("camera")
local Enemy    = require("enemy")
local Combat   = require("combat")
local NPC      = require("npc")
local Dialogue = require("dialogue")
local UI       = require("ui")

--[[
  Game states:
    "title"     – Main menu
    "explore"   – Walking the wasteland
    "combat"    – Turn-based strategic combat
    "dialogue"  – NPC conversation
    "inventory" – Item management
    "pause"     – Pause menu
    "gameover"  – Death screen (respawn at campfire)
]]
local state

local map, player, camera, ui
local npcs    = {}
local roamers = {}
local combat  = nil
local dialogue = nil
local invSelected = 1

local encounterCooldown = 0
local ENCOUNTER_COOLDOWN = 4   -- seconds of safety after a fight

-- Loot table for crates
local CRATE_LOOT = {
    {name="Canned Beans",  type="consumable", heal=30,  desc="Pre-war nutrition."},
    {name="Med Kit",       type="consumable", heal=50,  desc="Military-grade bandages."},
    {name="Torn Bandage",  type="consumable", heal=20,  desc="Stops bleeding."},
    {name="Rad-Away",      type="consumable", heal=15,  desc="Flushes radiation."},
    {name="Sharpened Blade",type="weapon",    atk=12,   desc="Honed scrap metal blade."},
    {name="Lead Pipe",     type="weapon",     atk=8,    desc="Solid and heavy."},
    {name="Scrap Metal",   type="junk",       desc="Useful for repairs.", value=12},
    {name="Circuit Board", type="junk",       desc="Pre-war electronics.", value=20},
}

---------------------------------------------------------------------------
-- Spawn helpers
---------------------------------------------------------------------------
local function makeRoamer(zone)
    local tx, ty
    if zone then
        tx, ty = map:findSpawnInZone(zone[1],zone[2],zone[3],zone[4])
    else
        tx, ty = map:findSpawn()
    end
    local template = Enemy.BESTIARY[math.random(#Enemy.BESTIARY)]
    return {
        x = (tx-1)*map.tileSize + map.tileSize/2,
        y = (ty-1)*map.tileSize + map.tileSize/2,
        colour = template.colour,
        templateIdx = nil, -- random on contact
        speed  = 18 + math.random(15),
        dirX   = math.random()<0.5 and -1 or 1,
        dirY   = math.random()<0.5 and -1 or 1,
        timer  = math.random()*4,
    }
end

local function spawnRoamers()
    roamers = {}
    -- Spread enemies across zones with controlled density
    local zones = {
        {2,2,20,18},      -- village zone (fewer)
        {22,10,46,40},    -- city zone (more)
        {44,2,62,20},     -- fort zone
        {2,42,22,62},     -- marsh
        {42,42,62,62},    -- glow zone
    }
    local counts = {2, 6, 4, 3, 3}
    for z, zone in ipairs(zones) do
        for _=1, counts[z] do
            table.insert(roamers, makeRoamer(zone))
        end
    end
end

local function spawnNPCs()
    npcs = {}
    -- Place NPCs near landmarks instead of random
    local placements = {
        {idx=1, tx=11, ty=9},    -- Harlan near Dusthaven
        {idx=2, tx=28, ty=20},   -- Mira near Old Metro
        {idx=3, tx=50, ty=12},   -- Merchant near Fort Ruin
    }
    for _, p in ipairs(placements) do
        -- Ensure the tile is walkable, nudge if not
        local tx, ty = p.tx, p.ty
        if map:isSolid(tx, ty) then
            tx, ty = map:findSpawnInZone(tx-3,ty-3,tx+3,ty+3)
        end
        table.insert(npcs, NPC.new(p.idx, tx, ty, map.tileSize))
    end
end

local function initGame()
    map    = Map.new(64, 64)
    player = Player.new(9, 11, map.tileSize)  -- Start near Dusthaven campfire
    camera = Camera.new()
    combat   = nil
    dialogue = nil
    encounterCooldown = 5
    spawnRoamers()
    spawnNPCs()
end

local function startNewGame()
    ui = UI.new()
    initGame()
    state = "explore"
    ui:notify("Welcome to the Wasteland. Find a campfire to save.", 5)
    ui:notify("Press E near NPCs or loot crates to interact.", 5)
end

---------------------------------------------------------------------------
-- LÖVE callbacks
---------------------------------------------------------------------------
function love.load()
    love.graphics.setDefaultFilter("nearest", "nearest")
    love.window.setTitle("Wasteland Chronicles")
    math.randomseed(os.time())
    ui = UI.new()
    state = "title"
end

function love.update(dt)
    if state == "title" then
        -- nothing to update
        return
    end

    if state == "pause" then
        return
    end

    if state == "explore" then
        player:update(dt, map)
        camera:follow(player.x, player.y, dt,
            love.graphics.getWidth(), love.graphics.getHeight())
        ui:update(dt)

        -- Campfire healing
        local ptx, pty = player:getTilePos()
        if map:isCampfire(ptx, pty) then
            if player.hp < player.maxHP then
                player:heal(math.ceil(player.maxHP * dt * 0.3))
                player:restoreAP(math.ceil(player.maxAP * dt * 0.2))
                player.radiation = math.max(0, player.radiation - dt * 20)
            end
            player:setSpawn(player.x, player.y)
        end

        -- Roamer movement & encounter
        encounterCooldown = math.max(0, encounterCooldown - dt)
        for i = #roamers, 1, -1 do
            local r = roamers[i]
            r.timer = r.timer + dt
            if r.timer > 3 then
                r.timer = 0
                r.dirX = math.random()<0.5 and -1 or 1
                r.dirY = math.random()<0.5 and -1 or 1
            end
            local nx = r.x + r.dirX*r.speed*dt
            local ny = r.y + r.dirY*r.speed*dt
            local tx = math.floor(nx/map.tileSize)+1
            local ty = math.floor(ny/map.tileSize)+1
            if not map:isSolid(tx,ty) then
                r.x, r.y = nx, ny
            else
                r.dirX, r.dirY = -r.dirX, -r.dirY
            end

            local dx = player.x - r.x
            local dy = player.y - r.y
            if math.sqrt(dx*dx+dy*dy) < 24 and encounterCooldown <= 0 then
                table.remove(roamers, i)
                combat = Combat.new(player)
                state  = "combat"
                camera:startShake(0.4, 6)
                break
            end
        end

    elseif state == "combat" then
        combat:update(dt)
        if combat.finished then
            if combat.result == "defeat" then
                state = "gameover"
            else
                state = "explore"
                encounterCooldown = ENCOUNTER_COOLDOWN
                if combat.result == "victory" then
                    ui:notify("Victory! +"..combat.enemy.xp.." XP", 3)
                    -- Respawn replacement
                    table.insert(roamers, makeRoamer(nil))
                elseif combat.result == "flee" then
                    ui:notify("Escaped!", 2)
                end
                combat = nil
            end
        end

    elseif state == "inventory" then
        ui:update(dt)
    end
end

function love.draw()
    local W = love.graphics.getWidth()
    local H = love.graphics.getHeight()

    if state == "title" then
        ui:drawTitle()
        return
    end

    if state == "combat" then
        combat:draw()
        return
    end

    if state == "gameover" then
        ui:drawGameOver()
        return
    end

    -- States that show the world underneath
    camera:apply()
    map:draw(camera.x, camera.y, W, H)

    local t = love.timer.getTime()
    for _, r in ipairs(roamers) do
        Enemy.drawRoamer(r.x, r.y, r.colour, t)
    end
    for _, n in ipairs(npcs) do n:draw(t) end
    player:draw()

    -- Interaction prompt
    local ptx, pty = player:getTilePos()
    -- NPC prompt
    for _, n in ipairs(npcs) do
        if n:canInteract(player.x, player.y) then
            love.graphics.setColor(1,0.9,0.3,0.8)
            love.graphics.print("[E] Talk to "..n.name, n.x-40, n.y-35)
            love.graphics.setColor(1,1,1)
        end
    end
    -- Campfire prompt
    if map:isCampfire(ptx, pty) then
        love.graphics.setColor(1,0.6,0.2,0.7+0.3*math.sin(t*3))
        love.graphics.print("Resting... HP restoring", player.x-50, player.y-30)
        love.graphics.setColor(1,1,1)
    end
    -- Loot crate prompt
    local crate = map:getLootCrate(ptx, pty)
    if crate then
        love.graphics.setColor(1,0.9,0.3,0.8)
        love.graphics.print("[E] Search crate", player.x-40, player.y-30)
        love.graphics.setColor(1,1,1)
    end

    camera:release()

    -- Screen-space HUD
    ui:drawHUD(player)
    ui:drawMinimap(map, player, npcs, roamers)
    ui:drawNotifications()

    -- Overlays
    if state == "dialogue" and dialogue then
        dialogue:draw()
    elseif state == "inventory" then
        ui:drawInventory(player, invSelected)
    elseif state == "pause" then
        ui:drawPause()
    end
end

function love.keypressed(key)
    -----------------------------------------------------------------
    -- TITLE
    -----------------------------------------------------------------
    if state == "title" then
        local result = ui:titleKeypressed(key)
        if result == "start" then
            startNewGame()
        end
        return
    end

    -----------------------------------------------------------------
    -- PAUSE
    -----------------------------------------------------------------
    if state == "pause" then
        local result = ui:pauseKeypressed(key)
        if result == "resume" then
            state = "explore"
        elseif result == "inventory" then
            state = "inventory"
            invSelected = 1
        elseif result == "title" then
            state = "title"
            ui = UI.new()
        end
        return
    end

    -----------------------------------------------------------------
    -- EXPLORE
    -----------------------------------------------------------------
    if state == "explore" then
        if key == "escape" then
            state = "pause"
            ui.pauseSelected = 1
            ui.showControls = false

        elseif key == "e" then
            -- NPC interaction
            local interacted = false
            for _, n in ipairs(npcs) do
                if n:canInteract(player.x, player.y) then
                    dialogue = Dialogue.new(n)
                    state = "dialogue"
                    interacted = true
                    break
                end
            end
            -- Loot crate
            if not interacted then
                local ptx, pty = player:getTilePos()
                local crate = map:getLootCrate(ptx, pty)
                if crate then
                    crate.looted = true
                    -- Give 1-2 items from loot table
                    local numItems = math.random(1, 2)
                    for _=1, numItems do
                        local loot = CRATE_LOOT[math.random(#CRATE_LOOT)]
                        -- Deep copy
                        local item = {}
                        for k,v in pairs(loot) do item[k]=v end
                        table.insert(player.inventory, item)
                        ui:notify("Found: "..item.name, 3)
                    end
                    player.cratesLooted = player.cratesLooted + 1
                end
            end

        elseif key == "i" then
            state = "inventory"
            invSelected = 1
        end

    -----------------------------------------------------------------
    -- COMBAT
    -----------------------------------------------------------------
    elseif state == "combat" then
        combat:keypressed(key)

    -----------------------------------------------------------------
    -- DIALOGUE
    -----------------------------------------------------------------
    elseif state == "dialogue" then
        dialogue:keypressed(key)
        if dialogue.finished then
            for _, item in ipairs(dialogue.giveItems) do
                table.insert(player.inventory, item)
                ui:notify("Received: "..item.name, 3)
            end
            dialogue = nil
            state = "explore"
        end

    -----------------------------------------------------------------
    -- INVENTORY
    -----------------------------------------------------------------
    elseif state == "inventory" then
        if key == "i" or key == "escape" then
            state = "explore"
        elseif key == "up" or key == "w" then
            invSelected = math.max(1, invSelected-1)
        elseif key == "down" or key == "s" then
            invSelected = math.min(#player.inventory, invSelected+1)
        elseif key == "return" or key == "space" then
            local item = player.inventory[invSelected]
            if item then
                if item.type == "consumable" then
                    if item.heal then
                        player:heal(item.heal)
                        ui:notify("Used "..item.name.." (+"..item.heal.." HP)", 2.5)
                    end
                    table.remove(player.inventory, invSelected)
                    invSelected = math.max(1, math.min(invSelected, #player.inventory))
                elseif item.type == "weapon" then
                    player.equippedWeapon = item
                    ui:notify("Equipped: "..item.name.." (ATK+"..item.atk..")", 2.5)
                end
            end
        end

    -----------------------------------------------------------------
    -- GAME OVER (respawn, not restart)
    -----------------------------------------------------------------
    elseif state == "gameover" then
        if key == "r" then
            player:respawn()
            combat = nil
            encounterCooldown = 5
            state = "explore"
            ui:notify("You awaken at the campfire...", 4)
        end
    end
end
