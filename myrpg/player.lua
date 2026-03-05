-- player.lua  –  Player with deterministic stats, abilities, respawn
local Player = {}
Player.__index = Player

function Player.new(tileX, tileY, tileSize)
    local self = setmetatable({}, Player)
    self.tileSize = tileSize or 32
    self.x = (tileX-1)*self.tileSize + self.tileSize/2
    self.y = (tileY-1)*self.tileSize + self.tileSize/2
    self.spawnX = self.x
    self.spawnY = self.y
    self.speed = 155

    self.dir = "down"
    self.moving = false
    self.animTimer = 0
    self.animFrame = 1

    -- Stats
    self.name   = "Wanderer"
    self.level  = 1
    self.xp     = 0
    self.xpNext = 80

    self.maxHP = 60
    self.hp    = 60
    self.maxAP = 30
    self.ap    = 30

    self.str   = 8    -- melee damage
    self.agi   = 6    -- dodge / crit
    self.end_  = 7    -- HP growth
    self.per   = 5    -- crit rate
    self.lck   = 4    -- loot quality
    self.defense = 3

    -- Abilities (cooldowns in turns)
    self.abilities = {
        {name="Strike",      desc="Reliable attack. No cooldown.",              cd=0, cdMax=0, dmgMul=1.0, apCost=0,  type="attack"},
        {name="Power Slam",  desc="Heavy hit dealing 1.8x damage.",            cd=0, cdMax=2, dmgMul=1.8, apCost=8,  type="attack"},
        {name="Guard",       desc="Block 60% of next enemy attack.",           cd=0, cdMax=1, dmgMul=0,   apCost=0,  type="defend"},
        {name="First Aid",   desc="Heal 30% of max HP.",                       cd=0, cdMax=3, dmgMul=0,   apCost=10, type="heal"},
        {name="Focused Shot", desc="Guaranteed critical hit (2x damage).",     cd=0, cdMax=4, dmgMul=2.0, apCost=12, type="attack"},
    }

    -- Inventory
    self.inventory = {
        {name="Rusty Pipe",   type="weapon",     atk=5,  desc="Bent metal pipe. Better than fists."},
        {name="Torn Bandage", type="consumable",  heal=20, desc="Stops bleeding, mostly."},
        {name="Canned Beans", type="consumable",  heal=30, desc="Pre-war nutrition."},
    }
    self.equippedWeapon = self.inventory[1]
    self.radiation = 0
    self.defending = false       -- set during combat guard
    self.kills = 0
    self.cratesLooted = 0

    return self
end

---------------------------------------------------------------------------
-- Ability helpers
---------------------------------------------------------------------------
function Player:tickCooldowns()
    for _, a in ipairs(self.abilities) do
        if a.cd > 0 then a.cd = a.cd - 1 end
    end
end

function Player:useAbility(idx)
    local a = self.abilities[idx]
    if not a then return false end
    if a.cd > 0 then return false end
    if a.apCost > self.ap then return false end
    a.cd = a.cdMax
    self.ap = self.ap - a.apCost
    return true
end

---------------------------------------------------------------------------
-- Levelling  (deterministic)
---------------------------------------------------------------------------
function Player:addXP(amount)
    self.xp = self.xp + amount
    local levelled = false
    while self.xp >= self.xpNext do
        self.xp     = self.xp - self.xpNext
        self.level  = self.level + 1
        self.xpNext = math.floor(self.xpNext * 1.4)
        self.maxHP  = self.maxHP + 5 + self.end_
        self.hp     = self.maxHP
        self.maxAP  = self.maxAP + 4
        self.ap     = self.maxAP
        self.str    = self.str + 1
        self.agi    = self.agi + 1
        self.per    = self.per + 1
        self.defense = self.defense + 1
        levelled = true
    end
    return levelled
end

---------------------------------------------------------------------------
-- Combat  (deterministic base damage)
---------------------------------------------------------------------------
function Player:getAttack(multiplier)
    local weaponAtk = self.equippedWeapon and self.equippedWeapon.atk or 0
    return math.floor((self.str + weaponAtk) * (multiplier or 1))
end

function Player:takeDamage(amount)
    local mult = self.defending and 0.4 or 1.0
    local reduced = math.max(1, math.floor(amount * mult) - self.defense)
    self.hp = math.max(0, self.hp - reduced)
    self.defending = false
    return reduced
end

function Player:isDead() return self.hp <= 0 end

function Player:heal(amount)
    self.hp = math.min(self.maxHP, self.hp + amount)
end

function Player:restoreAP(amount)
    self.ap = math.min(self.maxAP, self.ap + (amount or self.maxAP))
end

function Player:fullHeal()
    self.hp = self.maxHP
    self.ap = self.maxAP
    self.radiation = 0
    for _, a in ipairs(self.abilities) do a.cd = 0 end
end

function Player:respawn()
    self.hp = self.maxHP
    self.ap = self.maxAP
    self.radiation = 0
    self.defending = false
    self.x = self.spawnX
    self.y = self.spawnY
    for _, a in ipairs(self.abilities) do a.cd = 0 end
end

function Player:setSpawn(px, py)
    self.spawnX = px
    self.spawnY = py
end

---------------------------------------------------------------------------
-- Movement
---------------------------------------------------------------------------
function Player:update(dt, map)
    local dx, dy = 0, 0
    self.moving = false

    if love.keyboard.isDown("w","up")    then dy=-1; self.dir="up";    self.moving=true end
    if love.keyboard.isDown("s","down")  then dy= 1; self.dir="down";  self.moving=true end
    if love.keyboard.isDown("a","left")  then dx=-1; self.dir="left";  self.moving=true end
    if love.keyboard.isDown("d","right") then dx= 1; self.dir="right"; self.moving=true end

    local spd = self.speed
    if love.keyboard.isDown("lshift") then spd = spd * 1.55 end

    if dx~=0 and dy~=0 then
        local inv = 1/math.sqrt(2); dx=dx*inv; dy=dy*inv
    end

    local newX = self.x + dx*spd*dt
    local newY = self.y + dy*spd*dt
    local hw, hh = 10, 10
    local ts = self.tileSize

    local ttx = math.floor((newX + (dx>0 and hw or -hw))/ts)+1
    local tty = math.floor(self.y/ts)+1
    if not map:isSolid(ttx, tty) then self.x = newX end

    ttx = math.floor(self.x/ts)+1
    tty = math.floor((newY + (dy>0 and hh or -hh))/ts)+1
    if not map:isSolid(ttx, tty) then self.y = newY end

    if self.moving then
        self.animTimer = self.animTimer + dt*8
        self.animFrame = math.floor(self.animTimer%4)+1
    else
        self.animFrame = 1; self.animTimer = 0
    end

    -- Radiation
    local ptx = math.floor(self.x/ts)+1
    local pty = math.floor(self.y/ts)+1
    if map:isRadiation(ptx,pty) then
        self.radiation = self.radiation + dt*5
        if self.radiation > 100 then self.hp = math.max(0, self.hp-dt*10) end
    else
        self.radiation = math.max(0, self.radiation - dt*0.5)
    end
end

function Player:getTilePos()
    return math.floor(self.x/self.tileSize)+1, math.floor(self.y/self.tileSize)+1
end

---------------------------------------------------------------------------
-- Drawing
---------------------------------------------------------------------------
function Player:draw()
    local x, y = self.x, self.y
    local bob = self.moving and math.sin(self.animTimer*2)*2 or 0

    love.graphics.setColor(0,0,0,0.25)
    love.graphics.ellipse("fill",x,y+12,10,4)

    love.graphics.setColor(0.55,0.40,0.28)
    love.graphics.rectangle("fill",x-8,y-6+bob,16,16,2,2)

    love.graphics.setColor(0.82,0.68,0.55)
    love.graphics.circle("fill",x,y-10+bob,7)

    local ex,ey = 0,0
    if self.dir=="left" then ex=-2 elseif self.dir=="right" then ex=2 end
    if self.dir=="up" then ey=-2 elseif self.dir=="down" then ey=2 end
    love.graphics.setColor(0.1,0.1,0.1)
    love.graphics.circle("fill",x-3+ex,y-11+ey+bob,1.5)
    love.graphics.circle("fill",x+3+ex,y-11+ey+bob,1.5)

    love.graphics.setColor(0.6,0.15,0.15)
    love.graphics.rectangle("fill",x-7,y-17+bob,14,4,2,2)

    local legOff = self.moving and math.sin(self.animTimer*2)*3 or 0
    love.graphics.setColor(0.35,0.30,0.25)
    love.graphics.rectangle("fill",x-5,y+10+bob,4,6+legOff)
    love.graphics.rectangle("fill",x+1,y+10+bob,4,6-legOff)

    if self.equippedWeapon then
        love.graphics.setColor(0.6,0.6,0.6)
        if self.dir=="left" then
            love.graphics.rectangle("fill",x-12,y-4+bob,3,14)
        else
            love.graphics.rectangle("fill",x+9,y-4+bob,3,14)
        end
    end
    love.graphics.setColor(1,1,1)
end

return Player
