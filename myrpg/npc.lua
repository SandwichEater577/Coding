-- npc.lua  –  NPCs with branching dialogue
local NPC = {}
NPC.__index = NPC

NPC.TEMPLATES = {
    {
        name = "Old Harlan",
        colour = {0.6,0.5,0.4},
        dialogue = {
            {text="Well I'll be... Another soul wandering the waste.",responses={
                {text="What is this place?",next=2},
                {text="Got anything useful?",next=3},
                {text="Goodbye.",next=nil},
            }},
            {text="This here's the Ashlands. Used to be a city.\nNow it's dust, mutants, and folks too stubborn to die.",responses={
                {text="Tell me more.",next=4},
                {text="Thanks.",next=nil},
            }},
            {text="I might have somethin'... Take this.\n[Received: Torn Bandage]",responses={
                {text="Thanks!",next=nil,give={name="Torn Bandage",type="consumable",heal=20,desc="Stops bleeding."}},
            }},
            {text="Radiation's gettin' worse. Stay clear of the green.\nThere's a camp with a fire up north if you need rest.\nFires heal you up – remember that.",responses={
                {text="Good to know.",next=nil},
            }},
        },
    },
    {
        name = "Scavenger Mira",
        colour = {0.7,0.45,0.55},
        dialogue = {
            {text="Hey! You look like you can handle yourself.\nI found some supplies. Take 'em.",responses={
                {text="What do you have?",next=2},
                {text="Who are you?",next=3},
                {text="Maybe later.",next=nil},
            }},
            {text="Here – canned beans. You look half-starved.\n[Received: Canned Beans]",responses={
                {text="Thanks!",next=nil,give={name="Canned Beans",type="consumable",heal=30,desc="Pre-war nutrition."}},
            }},
            {text="Name's Mira. I scavenge the old tech bunkers.\nDangerous work, but the loot's worth it.\nTip: use Analyse in combat to learn enemy patterns.",responses={
                {text="Good advice.",next=nil},
            }},
        },
    },
    {
        name = "Wandering Merchant",
        colour = {0.5,0.6,0.7},
        dialogue = {
            {text="Fine wares from the bunkers!\nWell... fine-ish. Take a look.",responses={
                {text="What's the news?",next=2},
                {text="Got a weapon?",next=3},
                {text="No thanks.",next=nil},
            }},
            {text="Heard rumours about a vault to the east.\nFull of pre-war tech... or ghouls.\nAlso – the enemies near Fort Ruin are tough.\nMake sure you're at least level 3.",responses={
                {text="I'll be careful.",next=nil},
            }},
            {text="Got this pipe wrench. Hits harder than your pipe.\n[Received: Pipe Wrench]",responses={
                {text="Nice!",next=nil,give={name="Pipe Wrench",type="weapon",atk=9,desc="Heavy wrench. Solid upgrade."}},
            }},
        },
    },
}

function NPC.new(idx, tileX, tileY, tileSize)
    local template = NPC.TEMPLATES[idx] or NPC.TEMPLATES[1]
    local self = setmetatable({}, NPC)
    self.name     = template.name
    self.colour   = template.colour
    self.dialogue = template.dialogue
    self.tileX    = tileX
    self.tileY    = tileY
    self.tileSize = tileSize or 32
    self.x = (tileX-1)*self.tileSize + self.tileSize/2
    self.y = (tileY-1)*self.tileSize + self.tileSize/2
    self.interactRadius = 48
    return self
end

function NPC:canInteract(px, py)
    local dx, dy = px-self.x, py-self.y
    return math.sqrt(dx*dx+dy*dy) < self.interactRadius
end

function NPC:draw(time)
    local bob = math.sin(time*1.5+self.tileX)*2
    local x, y = self.x, self.y

    love.graphics.setColor(0,0,0,0.2)
    love.graphics.ellipse("fill",x,y+12,9,3)

    love.graphics.setColor(self.colour[1],self.colour[2],self.colour[3])
    love.graphics.rectangle("fill",x-7,y-6+bob,14,16,2,2)

    love.graphics.setColor(0.85,0.72,0.58)
    love.graphics.circle("fill",x,y-10+bob,6)

    love.graphics.setColor(0.1,0.1,0.1)
    love.graphics.circle("fill",x-2,y-11+bob,1.2)
    love.graphics.circle("fill",x+2,y-11+bob,1.2)

    love.graphics.setColor(1,0.9,0.2,0.6+0.4*math.sin(time*4))
    love.graphics.print("!",x-3,y-26+bob)

    love.graphics.setColor(1,1,1)
end

return NPC
