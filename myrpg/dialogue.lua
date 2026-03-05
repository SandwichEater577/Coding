-- dialogue.lua  –  Dialogue box overlay
local Dialogue = {}
Dialogue.__index = Dialogue

function Dialogue.new(npc)
    local self = setmetatable({}, Dialogue)
    self.npc = npc
    self.nodeIdx = 1
    self.selectedResponse = 1
    self.finished = false
    self.giveItems = {}
    return self
end

function Dialogue:getCurrentNode()
    return self.npc.dialogue[self.nodeIdx]
end

function Dialogue:keypressed(key)
    local node = self:getCurrentNode()
    if not node then self.finished = true; return end

    if key == "up" or key == "w" then
        self.selectedResponse = math.max(1, self.selectedResponse - 1)
    elseif key == "down" or key == "s" then
        self.selectedResponse = math.min(#node.responses, self.selectedResponse + 1)
    elseif key == "return" or key == "space" then
        local resp = node.responses[self.selectedResponse]
        if resp then
            if resp.give then table.insert(self.giveItems, resp.give) end
            if resp.next then
                self.nodeIdx = resp.next
                self.selectedResponse = 1
            else
                self.finished = true
            end
        end
    elseif key == "escape" then
        self.finished = true
    end
end

function Dialogue:draw()
    local W = love.graphics.getWidth()
    local H = love.graphics.getHeight()
    local node = self:getCurrentNode()
    if not node then return end

    love.graphics.setColor(0,0,0,0.5)
    love.graphics.rectangle("fill",0,0,W,H)

    local boxH = 220
    local boxY = H - boxH - 20
    love.graphics.setColor(0.08,0.08,0.10,0.95)
    love.graphics.rectangle("fill",20,boxY,W-40,boxH,8,8)
    love.graphics.setColor(0.5,0.45,0.35)
    love.graphics.rectangle("line",20,boxY,W-40,boxH,8,8)

    love.graphics.setColor(1,0.85,0.4)
    love.graphics.print(self.npc.name, 40, boxY+12)

    love.graphics.setColor(0.9,0.9,0.85)
    love.graphics.printf(node.text, 40, boxY+35, W-100)

    local ry = boxY + 120
    for i, resp in ipairs(node.responses) do
        if i == self.selectedResponse then
            love.graphics.setColor(1,0.9,0.3)
            love.graphics.print("> "..resp.text, 50, ry+(i-1)*24)
        else
            love.graphics.setColor(0.65,0.65,0.60)
            love.graphics.print("  "..resp.text, 50, ry+(i-1)*24)
        end
    end
    love.graphics.setColor(1,1,1)
end

return Dialogue
