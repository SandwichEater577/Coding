-- camera.lua  –  Smooth-follow camera with screen shake
local Camera = {}
Camera.__index = Camera

function Camera.new()
    local self = setmetatable({}, Camera)
    self.x = 0
    self.y = 0
    self.smoothing = 5
    self.shake = 0
    self.shakeIntensity = 0
    return self
end

function Camera:follow(tx, ty, dt, sw, sh)
    local gx = tx - sw/2
    local gy = ty - sh/2
    self.x = self.x + (gx-self.x)*self.smoothing*dt
    self.y = self.y + (gy-self.y)*self.smoothing*dt
    if self.shake > 0 then self.shake = self.shake - dt end
end

function Camera:startShake(dur, int)
    self.shake = dur
    self.shakeIntensity = int or 4
end

function Camera:apply()
    local ox, oy = 0, 0
    if self.shake > 0 then
        ox = (math.random()-0.5)*2*self.shakeIntensity
        oy = (math.random()-0.5)*2*self.shakeIntensity
    end
    love.graphics.push()
    love.graphics.translate(-math.floor(self.x+ox), -math.floor(self.y+oy))
end

function Camera:release()
    love.graphics.pop()
end

return Camera
