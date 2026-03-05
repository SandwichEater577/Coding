-- enemy.lua  –  Enemies with telegraphed attack patterns (no RNG damage)
local Enemy = {}
Enemy.__index = Enemy

---------------------------------------------------------------------------
-- Bestiary – each enemy has a PATTERN of moves the player can read
-- Patterns cycle:  "attack", "heavy", "charge", "rest"
-- On "charge" turn the enemy does nothing but next turn hits hard.
-- On "rest" turn the enemy skips → free opening.
---------------------------------------------------------------------------
Enemy.BESTIARY = {
    {
        name = "Feral Dog", hp = 24, atk = 7, def = 1,
        xp = 30,
        loot = {name="Dog Fang",type="junk",desc="Sharp canine tooth.",value=5},
        colour = {0.55,0.40,0.30},
        desc = "A snarling mutt. It lunges, then rests.",
        pattern = {"attack","attack","rest"},
        weakness = "Guard its 2nd strike, then counter.",
    },
    {
        name = "Raider", hp = 40, atk = 11, def = 3,
        xp = 60,
        loot = {name="Scrap Metal",type="junk",desc="Useful for repairs.",value=12},
        colour = {0.7,0.3,0.3},
        desc = "A desperate survivor. Charges before a big swing.",
        pattern = {"attack","charge","heavy","rest"},
        weakness = "Guard on the heavy hit after charge.",
    },
    {
        name = "Rad-Roach", hp = 16, atk = 5, def = 0,
        xp = 18,
        loot = {name="Roach Meat",type="consumable",heal=12,desc="Crunchy."},
        colour = {0.35,0.25,0.15},
        desc = "Enormous cockroach. Attacks relentlessly.",
        pattern = {"attack","attack","attack","rest"},
        weakness = "Low HP – burst it with Power Slam.",
    },
    {
        name = "Mutant", hp = 70, atk = 16, def = 5,
        xp = 110,
        loot = {name="Mutant Gland",type="junk",desc="Pulsing green organ.",value=30},
        colour = {0.3,0.65,0.2},
        desc = "Hulking irradiated mass. Slow but devastating.",
        pattern = {"charge","heavy","rest","attack","rest"},
        weakness = "Two rest turns. Punish both.",
    },
    {
        name = "Scrap Bot", hp = 50, atk = 13, def = 8,
        xp = 85,
        loot = {name="Circuit Board",type="junk",desc="Pre-war electronics.",value=20},
        colour = {0.6,0.6,0.65},
        desc = "Rogue robot. High armor, predictable.",
        pattern = {"attack","attack","charge","heavy","rest","rest"},
        weakness = "Use Focused Shot to bypass its armor.",
    },
}

---------------------------------------------------------------------------
-- Factory
---------------------------------------------------------------------------
function Enemy.random(playerLevel)
    local pool = Enemy.BESTIARY
    local template = pool[math.random(#pool)]
    return Enemy.fromTemplate(template, playerLevel)
end

function Enemy.fromTemplate(template, playerLevel)
    local self = setmetatable({}, Enemy)
    local scale = 1 + (playerLevel-1)*0.12
    self.name     = template.name
    self.desc     = template.desc
    self.colour   = template.colour
    self.weakness = template.weakness

    self.maxHP = math.floor(template.hp * scale)
    self.hp    = self.maxHP
    self.atk   = math.floor(template.atk * scale)
    self.def   = math.floor(template.def * scale)
    self.xp    = math.floor(template.xp  * scale)

    -- Deep-copy pattern
    self.pattern = {}
    for i, v in ipairs(template.pattern) do self.pattern[i] = v end
    self.patternIdx = 1   -- where in the cycle we are

    -- Deep-copy loot
    self.loot = {}
    for k, v in pairs(template.loot) do self.loot[k] = v end

    self.animTimer = 0
    self.charging  = false   -- set true on "charge" turn
    return self
end

---------------------------------------------------------------------------
-- Pattern logic
---------------------------------------------------------------------------
function Enemy:getNextMove()
    return self.pattern[self.patternIdx]
end

function Enemy:advancePattern()
    self.patternIdx = self.patternIdx + 1
    if self.patternIdx > #self.pattern then self.patternIdx = 1 end
end

function Enemy:getAttack()
    -- Deterministic: base atk. Heavy = 1.8x after charge.
    if self.charging then
        self.charging = false
        return math.floor(self.atk * 1.8)
    end
    return self.atk
end

function Enemy:takeDamage(amount)
    local reduced = math.max(1, amount - self.def)
    self.hp = math.max(0, self.hp - reduced)
    return reduced
end

function Enemy:isDead() return self.hp <= 0 end

---------------------------------------------------------------------------
-- Drawing in combat view
---------------------------------------------------------------------------
function Enemy:drawCombat(cx, cy, time)
    local bob = math.sin(time*2)*3
    local c = self.colour
    love.graphics.setColor(c[1],c[2],c[3])
    love.graphics.ellipse("fill",cx,cy+bob,40,55)

    -- Eyes
    love.graphics.setColor(1,0.2,0.1,0.9)
    love.graphics.circle("fill",cx-12,cy-20+bob,6)
    love.graphics.circle("fill",cx+12,cy-20+bob,6)
    love.graphics.setColor(0,0,0)
    love.graphics.circle("fill",cx-12,cy-20+bob,3)
    love.graphics.circle("fill",cx+12,cy-20+bob,3)
    love.graphics.setColor(0,0,0,0.7)
    love.graphics.arc("fill",cx,cy-8+bob,15,0.2,math.pi-0.2)

    -- Charging visual
    if self.charging then
        love.graphics.setColor(1,0.5,0.1,0.3+0.2*math.sin(time*8))
        love.graphics.circle("line",cx,cy+bob,50+math.sin(time*6)*5)
        love.graphics.circle("line",cx,cy+bob,55+math.sin(time*6)*5)
    end

    -- HP bar
    local bw, bh = 110, 10
    local bx, by = cx-bw/2, cy+65
    love.graphics.setColor(0.2,0.2,0.2)
    love.graphics.rectangle("fill",bx,by,bw,bh,3,3)
    local r = self.hp/self.maxHP
    love.graphics.setColor(0.8*(1-r),0.8*r,0.1)
    love.graphics.rectangle("fill",bx,by,bw*r,bh,3,3)

    love.graphics.setColor(1,1,1)
    love.graphics.printf(self.name.."  HP:"..self.hp.."/"..self.maxHP, bx-20, by+14, 150, "center")
end

---------------------------------------------------------------------------
-- World roamer drawing
---------------------------------------------------------------------------
function Enemy.drawRoamer(x, y, colour, time)
    local bob = math.sin(time*3+x)*2
    love.graphics.setColor(colour[1],colour[2],colour[3])
    love.graphics.ellipse("fill",x,y+bob,10,13)
    love.graphics.setColor(1,0.2,0.1,0.8)
    love.graphics.circle("fill",x-3,y-5+bob,2)
    love.graphics.circle("fill",x+3,y-5+bob,2)
    love.graphics.setColor(1,1,1)
end

return Enemy
