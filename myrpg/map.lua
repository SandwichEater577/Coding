-- map.lua  –  Structured post-apocalyptic map with zones, checkpoints, loot
local Map = {}
Map.__index = Map

Map.TILES = {
    DIRT      = 1,
    RUBBLE    = 2,
    ROAD      = 3,
    WALL      = 4,
    WATER     = 5,
    GRASS     = 6,
    BUILDING  = 7,
    DOOR      = 8,
    RADIATION = 9,
    CAMPFIRE  = 10,
    LOOT      = 11,
}

Map.TILE_COLOURS = {
    [1]  = {0.45, 0.35, 0.25},
    [2]  = {0.50, 0.45, 0.40},
    [3]  = {0.38, 0.38, 0.38},
    [4]  = {0.30, 0.28, 0.26},
    [5]  = {0.15, 0.35, 0.30},
    [6]  = {0.30, 0.42, 0.20},
    [7]  = {0.25, 0.22, 0.20},
    [8]  = {0.55, 0.40, 0.25},
    [9]  = {0.45, 0.55, 0.10},
    [10] = {0.60, 0.35, 0.15},
    [11] = {0.50, 0.50, 0.30},
}

Map.SOLID = { [4] = true, [5] = true }

Map.TILE_SIZE = 32

function Map.new(w, h)
    local self = setmetatable({}, Map)
    self.width    = w or 64
    self.height   = h or 64
    self.tileSize = Map.TILE_SIZE
    self.tiles    = {}
    self.campfires  = {}
    self.lootCrates = {}
    self.landmarks  = {}
    self:generate()
    return self
end

---------------------------------------------------------------------------
-- Helpers
---------------------------------------------------------------------------
local function inBounds(self, x, y)
    return x >= 1 and x <= self.width and y >= 1 and y <= self.height
end

function Map:grassPatch(cx, cy, r)
    for dy = -r, r do for dx = -r, r do
        local nx, ny = cx+dx, cy+dy
        if inBounds(self, nx, ny) and dx*dx+dy*dy <= r*r and math.random() < 0.7 then
            if self.tiles[ny][nx] == 1 then self.tiles[ny][nx] = 6 end
        end
    end end
end

function Map:radZone(cx, cy, r)
    for dy = -r, r do for dx = -r, r do
        local nx, ny = cx+dx, cy+dy
        if inBounds(self, nx, ny) and dx*dx+dy*dy <= r*r then
            self.tiles[ny][nx] = 9
        end
    end end
end

function Map:building(bx, by, bw, bh)
    if bx+bw > self.width or by+bh > self.height then return end
    for dy = 0, bh do for dx = 0, bw do
        self.tiles[by+dy][bx+dx] = 7
    end end
    for dx = 0, bw do self.tiles[by][bx+dx] = 4; self.tiles[by+bh][bx+dx] = 4 end
    for dy = 0, bh do self.tiles[by+dy][bx] = 4; self.tiles[by+dy][bx+bw] = 4 end
    self.tiles[by+bh][bx+math.floor(bw/2)] = 8
    for _ = 1, math.random(1,2) do
        local rx = bx+math.random(1,math.max(1,bw-1))
        local ry = by+(math.random()<0.5 and 0 or bh)
        if inBounds(self,rx,ry) then self.tiles[ry][rx] = 2 end
    end
end

function Map:campfire(tx, ty)
    if inBounds(self,tx,ty) then
        self.tiles[ty][tx] = 10
        table.insert(self.campfires, {x=tx,y=ty})
    end
end

function Map:loot(tx, ty)
    if inBounds(self,tx,ty) then
        self.tiles[ty][tx] = 11
        table.insert(self.lootCrates, {x=tx,y=ty,looted=false})
    end
end

---------------------------------------------------------------------------
-- Generation
---------------------------------------------------------------------------
function Map:generate()
    for y = 1, self.height do
        self.tiles[y] = {}
        for x = 1, self.width do self.tiles[y][x] = 1 end
    end

    -- Zone 1 – Starting village (top-left)
    self:grassPatch(8,8,8); self:grassPatch(12,14,6)
    self:building(5,5,7,5); self:building(14,5,6,5)
    self:campfire(10,10)
    self:loot(7,8)
    table.insert(self.landmarks, {name="Dusthaven",x=10,y=8})

    -- Roads
    for x = 1, self.width do self.tiles[20][x]=3; self.tiles[21][x]=3 end
    for y = 1, self.height do self.tiles[y][32]=3; self.tiles[y][33]=3 end
    for x = 1, 32 do self.tiles[40][x]=3 end

    -- Zone 2 – Ruined city (center)
    self:building(26,14,8,6); self:building(36,14,7,5)
    self:building(26,24,6,6); self:building(35,24,8,5)
    self:building(28,32,9,6)
    self:campfire(30,19); self:loot(29,17); self:loot(38,16)
    for _ = 1, 40 do
        local rx,ry=math.random(24,44),math.random(12,38)
        if self.tiles[ry][rx]==1 then self.tiles[ry][rx]=2 end
    end
    table.insert(self.landmarks, {name="Old Metro",x=32,y=20})

    -- Zone 3 – Radiation waste (bottom-right)
    self:radZone(50,50,5); self:radZone(55,45,4); self:radZone(46,55,3)
    self:loot(52,48); self:loot(48,52)
    table.insert(self.landmarks, {name="The Glow",x=50,y=50})

    -- Zone 4 – Swamp (bottom-left)
    for x = 3, 20 do
        local off = math.floor(math.sin(x*0.4)*2)
        for dy = 0, 3 do
            local ny = 50+dy+off
            if inBounds(self,x,ny) then self.tiles[ny][x]=5 end
        end
    end
    self:grassPatch(10,45,5); self:grassPatch(15,55,4)
    self:campfire(12,48); self:loot(8,46)
    table.insert(self.landmarks, {name="Murk Marsh",x=12,y=50})

    -- Zone 5 – Military base (top-right)
    self:building(48,4,10,8); self:building(50,14,8,6)
    self:campfire(52,11); self:loot(53,7); self:loot(55,17)
    table.insert(self.landmarks, {name="Fort Ruin",x=52,y=8})

    -- Scatter grass
    for _ = 1, 60 do self:grassPatch(math.random(3,self.width-3),math.random(3,self.height-3),math.random(2,4)) end

    -- Borders
    for x=1,self.width  do self.tiles[1][x]=4; self.tiles[self.height][x]=4 end
    for y=1,self.height do self.tiles[y][1]=4; self.tiles[y][self.width]=4 end
end

---------------------------------------------------------------------------
-- Queries
---------------------------------------------------------------------------
function Map:isSolid(tx, ty)
    if not inBounds(self,tx,ty) then return true end
    return Map.SOLID[self.tiles[ty][tx]] == true
end
function Map:getTile(tx, ty)
    if not inBounds(self,tx,ty) then return 4 end
    return self.tiles[ty][tx]
end
function Map:isRadiation(tx,ty) return self:getTile(tx,ty)==9 end
function Map:isCampfire(tx,ty) return self:getTile(tx,ty)==10 end

function Map:getLootCrate(tx,ty)
    for _,c in ipairs(self.lootCrates) do
        if c.x==tx and c.y==ty and not c.looted then return c end
    end
end

function Map:findSpawn()
    for _=1,1000 do
        local x,y=math.random(3,self.width-3),math.random(3,self.height-3)
        if not self:isSolid(x,y) and not self:isRadiation(x,y) then return x,y end
    end
    return 3,3
end

function Map:findSpawnInZone(x1,y1,x2,y2)
    for _=1,500 do
        local x,y=math.random(x1,x2),math.random(y1,y2)
        if not self:isSolid(x,y) and not self:isRadiation(x,y) then return x,y end
    end
    return self:findSpawn()
end

---------------------------------------------------------------------------
-- Drawing
---------------------------------------------------------------------------
function Map:draw(camX, camY, screenW, screenH)
    local ts = self.tileSize
    local sx = math.max(1, math.floor(camX/ts))
    local sy = math.max(1, math.floor(camY/ts))
    local ex = math.min(self.width,  math.ceil((camX+screenW)/ts)+1)
    local ey = math.min(self.height, math.ceil((camY+screenH)/ts)+1)
    local time = love.timer.getTime()

    for y = sy, ey do for x = sx, ex do
        local tile = self.tiles[y][x]
        local c = Map.TILE_COLOURS[tile] or {1,0,1}
        love.graphics.setColor(c[1],c[2],c[3])
        love.graphics.rectangle("fill",(x-1)*ts,(y-1)*ts,ts,ts)
        love.graphics.setColor(0,0,0,0.06)
        love.graphics.rectangle("line",(x-1)*ts,(y-1)*ts,ts,ts)

        if tile == 2 then
            love.graphics.setColor(0.4,0.38,0.33,0.6)
            love.graphics.circle("fill",(x-1)*ts+10,(y-1)*ts+14,4)
            love.graphics.circle("fill",(x-1)*ts+22,(y-1)*ts+20,3)
        elseif tile == 9 then
            love.graphics.setColor(0.6,0.8,0,0.3+0.15*math.sin(time*3+x+y))
            love.graphics.rectangle("fill",(x-1)*ts,(y-1)*ts,ts,ts)
        elseif tile == 5 then
            love.graphics.setColor(0.2,0.5,0.45,0.2+0.1*math.sin(time*2+x))
            love.graphics.rectangle("fill",(x-1)*ts,(y-1)*ts,ts,ts)
        elseif tile == 10 then
            local fl = math.sin(time*8+x)*2
            love.graphics.setColor(1,0.6,0.1,0.25+0.15*math.sin(time*5))
            love.graphics.circle("fill",(x-1)*ts+ts/2,(y-1)*ts+ts/2,ts*0.8)
            love.graphics.setColor(1,0.4,0.05,0.9)
            love.graphics.polygon("fill",
                (x-1)*ts+ts/2,(y-1)*ts+4+fl,
                (x-1)*ts+ts/2-6,(y-1)*ts+ts-4,
                (x-1)*ts+ts/2+6,(y-1)*ts+ts-4)
            love.graphics.setColor(1,0.8,0.2,0.8)
            love.graphics.polygon("fill",
                (x-1)*ts+ts/2,(y-1)*ts+10+fl,
                (x-1)*ts+ts/2-3,(y-1)*ts+ts-4,
                (x-1)*ts+ts/2+3,(y-1)*ts+ts-4)
        elseif tile == 11 then
            local looted = false
            for _,cr in ipairs(self.lootCrates) do
                if cr.x==x and cr.y==y and cr.looted then looted=true; break end
            end
            love.graphics.setColor(looted and {0.3,0.3,0.25} or {0.7,0.6,0.3})
            love.graphics.rectangle("fill",(x-1)*ts+4,(y-1)*ts+8,ts-8,ts-12,2,2)
            love.graphics.setColor(0.4,0.3,0.15)
            love.graphics.rectangle("line",(x-1)*ts+4,(y-1)*ts+8,ts-8,ts-12,2,2)
            if not looted then
                love.graphics.setColor(1,0.9,0.3,0.5+0.3*math.sin(time*3))
                love.graphics.circle("fill",(x-1)*ts+ts/2,(y-1)*ts+ts/2,3)
            end
        end
    end end
    love.graphics.setColor(1,1,1)
end

return Map
