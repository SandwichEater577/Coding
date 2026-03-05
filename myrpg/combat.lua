-- combat.lua  –  Strategic turn-based combat  (deterministic, pattern-based)
local Enemy = require("enemy")

local Combat = {}
Combat.__index = Combat

--[[
  Flow:
    intro → player_turn → player_anim → enemy_turn → enemy_anim → (loop)
    Also: player_item sub-state
    Ends: victory / defeat / flee
  
  Key design:
    - Enemy's NEXT move is always shown so the player can plan  
    - Damage is deterministic (no random rolls)
    - Abilities have cooldowns, not RNG
    - Guard at the right time = strategic play
]]

function Combat.new(player)
    local self = setmetatable({}, Combat)
    self.player = player
    self.enemy  = Enemy.random(player.level)
    self.state  = "intro"
    self.timer  = 0
    self.log    = {self.enemy.desc}
    self.turn   = 0

    -- Player UI
    self.selectedAbility = 1
    self.selectedItem    = 1
    self.selectedAction  = 1
    self.actions = {"Abilities", "Item", "Analyse", "Flee"}

    -- Flash effects
    self.flashTimer  = 0
    self.flashTarget = nil

    self.finished = false
    self.result   = nil
    self.showAnalysis = false
    return self
end

function Combat:addLog(msg)
    table.insert(self.log, msg)
    if #self.log > 7 then table.remove(self.log, 1) end
end

function Combat:update(dt)
    self.timer = self.timer + dt
    if self.flashTimer > 0 then self.flashTimer = self.flashTimer - dt end

    if self.state == "intro" then
        if self.timer > 1.2 then
            self.state = "player_turn"
            self:addLog("Enemy's next move: " .. self:describeEnemyIntent())
            self:addLog("Choose your action.")
        end

    elseif self.state == "player_anim" then
        if self.timer > 0.6 then
            if self.enemy:isDead() then
                self.state = "victory"; self.timer = 0
                local levelled = self.player:addXP(self.enemy.xp)
                self.player.kills = self.player.kills + 1
                self:addLog(self.enemy.name .. " destroyed! +" .. self.enemy.xp .. " XP")
                if levelled then
                    self:addLog("** LEVEL UP! Now level " .. self.player.level .. " **")
                end
            else
                self.state = "enemy_turn"; self.timer = 0
            end
        end

    elseif self.state == "enemy_turn" then
        if self.timer > 0.8 then
            self:executeEnemyTurn()
            self.state = "enemy_anim"; self.timer = 0
        end

    elseif self.state == "enemy_anim" then
        if self.timer > 0.6 then
            if self.player:isDead() then
                self.state = "defeat"; self.timer = 0
                self:addLog("You have fallen...")
            else
                self.turn = self.turn + 1
                self.player:tickCooldowns()
                self.player:restoreAP(3)  -- regen a little AP each turn
                self.state = "player_turn"; self.timer = 0
                self:addLog("Enemy's next move: " .. self:describeEnemyIntent())
                self:addLog("Choose your action.")
            end
        end

    elseif self.state == "victory" then
        if self.timer > 2.0 then
            if self.enemy.loot then
                table.insert(self.player.inventory, self.enemy.loot)
                self:addLog("Obtained: " .. self.enemy.loot.name)
            end
            self.finished = true; self.result = "victory"
        end

    elseif self.state == "defeat" then
        if self.timer > 2.0 then
            self.finished = true; self.result = "defeat"
        end

    elseif self.state == "flee" then
        if self.timer > 0.8 then
            self.finished = true; self.result = "flee"
        end
    end
end

---------------------------------------------------------------------------
-- Enemy intent description (shown to player)
---------------------------------------------------------------------------
function Combat:describeEnemyIntent()
    local move = self.enemy:getNextMove()
    if move == "attack" then
        return "ATTACK (" .. self.enemy.atk .. " dmg)"
    elseif move == "charge" then
        return "CHARGING (will hit hard next turn!)"
    elseif move == "heavy" then
        return "HEAVY STRIKE (" .. math.floor(self.enemy.atk*1.8) .. " dmg!)"
    elseif move == "rest" then
        return "RESTING (free opening!)"
    end
    return "???"
end

---------------------------------------------------------------------------
-- Enemy turn execution
---------------------------------------------------------------------------
function Combat:executeEnemyTurn()
    local move = self.enemy:getNextMove()
    self.enemy:advancePattern()

    if move == "attack" then
        local dmg = self.enemy:getAttack()
        local dealt = self.player:takeDamage(dmg)
        self:addLog(self.enemy.name .. " strikes for " .. dealt .. " damage!")
        self.flashTimer = 0.3; self.flashTarget = "player"

    elseif move == "charge" then
        self.enemy.charging = true
        self:addLog(self.enemy.name .. " is charging up!")

    elseif move == "heavy" then
        local dmg = self.enemy:getAttack()
        local dealt = self.player:takeDamage(dmg)
        self:addLog(self.enemy.name .. " SLAMS for " .. dealt .. " damage!")
        self.flashTimer = 0.4; self.flashTarget = "player"

    elseif move == "rest" then
        self:addLog(self.enemy.name .. " catches its breath.")
    end
end

---------------------------------------------------------------------------
-- Key handling
---------------------------------------------------------------------------
function Combat:keypressed(key)
    if self.state == "player_turn" then
        if self.showAnalysis then
            if key == "escape" or key == "return" or key == "space" then
                self.showAnalysis = false
            end
            return
        end

        if key == "up" or key == "w" then
            self.selectedAction = math.max(1, self.selectedAction - 1)
        elseif key == "down" or key == "s" then
            self.selectedAction = math.min(#self.actions, self.selectedAction + 1)
        elseif key == "return" or key == "space" then
            local act = self.actions[self.selectedAction]
            if act == "Abilities" then
                self.state = "abilities_menu"
                self.selectedAbility = 1
            elseif act == "Item" then
                self.state = "player_item"
                self.selectedItem = 1
            elseif act == "Analyse" then
                self.showAnalysis = true
            elseif act == "Flee" then
                -- Flee always works if enemy is resting, 50% otherwise
                local move = self.enemy:getNextMove()
                if move == "rest" then
                    self:addLog("You slip away while the enemy rests.")
                    self.state = "flee"; self.timer = 0
                elseif self.player.agi >= 8 then
                    self:addLog("Your agility lets you escape!")
                    self.state = "flee"; self.timer = 0
                else
                    self:addLog("Can't escape – enemy is too aggressive!")
                    self.state = "enemy_turn"; self.timer = 0
                end
            end
        end

    elseif self.state == "abilities_menu" then
        local abilities = self.player.abilities
        if key == "up" or key == "w" then
            self.selectedAbility = math.max(1, self.selectedAbility - 1)
        elseif key == "down" or key == "s" then
            self.selectedAbility = math.min(#abilities, self.selectedAbility + 1)
        elseif key == "escape" then
            self.state = "player_turn"
        elseif key == "return" or key == "space" then
            local a = abilities[self.selectedAbility]
            if a.cd > 0 then
                self:addLog(a.name .. " on cooldown! (" .. a.cd .. " turns)")
            elseif a.apCost > self.player.ap then
                self:addLog("Not enough AP! Need " .. a.apCost)
            else
                self.player:useAbility(self.selectedAbility)
                self:executePlayerAbility(a)
            end
        end

    elseif self.state == "player_item" then
        local cons = self:getConsumables()
        if #cons == 0 then
            self:addLog("No items!"); self.state = "player_turn"; return
        end
        if key == "up" or key == "w" then
            self.selectedItem = math.max(1, self.selectedItem - 1)
        elseif key == "down" or key == "s" then
            self.selectedItem = math.min(#cons, self.selectedItem + 1)
        elseif key == "escape" then
            self.state = "player_turn"
        elseif key == "return" or key == "space" then
            local item = cons[self.selectedItem]
            if item and item.heal then
                self.player:heal(item.heal)
                self:addLog("Used " .. item.name .. "! +" .. item.heal .. " HP")
                for i, inv in ipairs(self.player.inventory) do
                    if inv == item then table.remove(self.player.inventory, i); break end
                end
                self.state = "enemy_turn"; self.timer = 0
            end
        end
    end
end

---------------------------------------------------------------------------
-- Player ability execution
---------------------------------------------------------------------------
function Combat:executePlayerAbility(a)
    if a.type == "attack" then
        local dmg = self.player:getAttack(a.dmgMul)
        local dealt = self.enemy:takeDamage(dmg)
        self:addLog(a.name .. "! " .. dealt .. " damage!")
        self.flashTimer = 0.3; self.flashTarget = "enemy"
        self.state = "player_anim"; self.timer = 0

    elseif a.type == "defend" then
        self.player.defending = true
        self:addLog("You brace for impact! (60% block)")
        self.state = "enemy_turn"; self.timer = 0

    elseif a.type == "heal" then
        local amount = math.floor(self.player.maxHP * 0.3)
        self.player:heal(amount)
        self:addLog("First Aid! +" .. amount .. " HP")
        self.state = "enemy_turn"; self.timer = 0
    end
end

function Combat:getConsumables()
    local r = {}
    for _, item in ipairs(self.player.inventory) do
        if item.type == "consumable" then table.insert(r, item) end
    end
    return r
end

---------------------------------------------------------------------------
-- Drawing
---------------------------------------------------------------------------
function Combat:draw()
    local W = love.graphics.getWidth()
    local H = love.graphics.getHeight()
    local time = love.timer.getTime()

    -- Background
    love.graphics.setColor(0.05,0.05,0.08,0.95)
    love.graphics.rectangle("fill",0,0,W,H)
    love.graphics.setColor(0.15,0.12,0.10)
    love.graphics.rectangle("fill",0,H*0.35,W,H*0.65)
    love.graphics.setColor(0.25,0.15,0.10,0.5)
    love.graphics.rectangle("fill",0,H*0.34,W,4)

    -- Stars
    love.graphics.setColor(1,1,1,0.4)
    math.randomseed(42)
    for _=1,30 do love.graphics.circle("fill",math.random(W),math.random(math.floor(H*0.34)),1) end
    math.randomseed(os.time())

    -- Enemy flash
    if self.flashTimer > 0 and self.flashTarget == "enemy" then
        love.graphics.setColor(1,0.3,0.3,0.5)
        love.graphics.rectangle("fill",W/2-60,H*0.15,120,130)
    end
    self.enemy:drawCombat(W/2, H*0.3, time)

    -- Enemy intent banner
    love.graphics.setColor(0.1,0.1,0.14,0.9)
    love.graphics.rectangle("fill",W/2-160,H*0.55,320,28,4,4)
    local intent = self.enemy:getNextMove()
    if intent == "rest" then
        love.graphics.setColor(0.3,1,0.4)
    elseif intent == "heavy" then
        love.graphics.setColor(1,0.3,0.2)
    elseif intent == "charge" then
        love.graphics.setColor(1,0.7,0.2)
    else
        love.graphics.setColor(0.8,0.8,0.7)
    end
    love.graphics.printf("NEXT: " .. self:describeEnemyIntent(), W/2-155, H*0.555, 310, "center")

    -- Player panel
    local panelY = H - 230
    love.graphics.setColor(0.1,0.1,0.12,0.9)
    love.graphics.rectangle("fill",10,panelY,W-20,220,6,6)
    love.graphics.setColor(0.4,0.35,0.3)
    love.graphics.rectangle("line",10,panelY,W-20,220,6,6)

    if self.flashTimer > 0 and self.flashTarget == "player" then
        love.graphics.setColor(1,0.2,0.1,0.3)
        love.graphics.rectangle("fill",10,panelY,W-20,220,6,6)
    end

    -- Player HP/AP
    love.graphics.setColor(1,1,1)
    love.graphics.print(self.player.name.."  Lv."..self.player.level.."   Turn "..self.turn, 30, panelY+10)

    local hpR = self.player.hp / self.player.maxHP
    love.graphics.setColor(0.2,0.2,0.2)
    love.graphics.rectangle("fill",30,panelY+30,200,12,3,3)
    if hpR > 0.5 then love.graphics.setColor(0.2,0.75,0.25)
    elseif hpR > 0.25 then love.graphics.setColor(0.9,0.7,0.1)
    else love.graphics.setColor(0.9,0.2,0.1) end
    love.graphics.rectangle("fill",30,panelY+30,200*hpR,12,3,3)
    love.graphics.setColor(1,1,1)
    love.graphics.print("HP:"..self.player.hp.."/"..self.player.maxHP, 240, panelY+28)

    local apR = self.player.ap / self.player.maxAP
    love.graphics.setColor(0.2,0.2,0.2)
    love.graphics.rectangle("fill",30,panelY+46,140,8,3,3)
    love.graphics.setColor(0.25,0.5,0.9)
    love.graphics.rectangle("fill",30,panelY+46,140*apR,8,3,3)
    love.graphics.setColor(0.7,0.7,1)
    love.graphics.print("AP:"..self.player.ap.."/"..self.player.maxAP, 180, panelY+43)

    -- Log
    local logX, logY = 30, panelY+65
    for i, msg in ipairs(self.log) do
        love.graphics.setColor(0.7,0.75,0.65, i/#self.log)
        love.graphics.print("> "..msg, logX, logY+(i-1)*17)
    end

    -- Action menu
    if self.state == "player_turn" and not self.showAnalysis then
        local mx, my = W-200, panelY+15
        love.graphics.setColor(0.15,0.15,0.18,0.9)
        love.graphics.rectangle("fill",mx-10,my-5,190,#self.actions*28+10,4,4)
        for i, act in ipairs(self.actions) do
            if i == self.selectedAction then
                love.graphics.setColor(1,0.85,0.3)
                love.graphics.print("> "..act, mx, my+(i-1)*28)
            else
                love.graphics.setColor(0.7,0.7,0.7)
                love.graphics.print("  "..act, mx, my+(i-1)*28)
            end
        end
    end

    -- Abilities submenu
    if self.state == "abilities_menu" then
        local abilities = self.player.abilities
        local mx, my = W-350, panelY+15
        love.graphics.setColor(0.12,0.14,0.12,0.95)
        love.graphics.rectangle("fill",mx-10,my-5,330,#abilities*26+40,4,4)
        love.graphics.setColor(1,1,1)
        love.graphics.print("ABILITIES (Esc=back)", mx, my)
        for i, a in ipairs(abilities) do
            local y = my + 20 + (i-1)*26
            if i == self.selectedAbility then
                love.graphics.setColor(1,0.9,0.3)
                local cdTxt = a.cd > 0 and " [CD:"..a.cd.."]" or ""
                local apTxt = a.apCost > 0 and " ("..a.apCost.."AP)" or ""
                love.graphics.print("> "..a.name..apTxt..cdTxt, mx, y)
                love.graphics.setColor(0.6,0.6,0.55)
                love.graphics.print("  "..a.desc, mx, y+13)
            else
                local avail = a.cd == 0 and a.apCost <= self.player.ap
                love.graphics.setColor(avail and {0.7,0.7,0.7} or {0.4,0.4,0.4})
                local cdTxt = a.cd > 0 and " [CD:"..a.cd.."]" or ""
                love.graphics.print("  "..a.name..cdTxt, mx, y)
            end
        end
    end

    -- Item submenu
    if self.state == "player_item" then
        local cons = self:getConsumables()
        local mx, my = W-300, panelY+15
        love.graphics.setColor(0.12,0.14,0.12,0.95)
        love.graphics.rectangle("fill",mx-10,my-5,200,math.max(1,#cons)*28+35,4,4)
        love.graphics.setColor(1,1,1)
        love.graphics.print("USE ITEM (Esc=back)", mx, my)
        if #cons == 0 then
            love.graphics.setColor(0.5,0.5,0.5)
            love.graphics.print("  (empty)", mx, my+24)
        else
            for i, item in ipairs(cons) do
                if i == self.selectedItem then
                    love.graphics.setColor(0.3,1,0.4)
                    love.graphics.print("> "..item.name.." (+"..item.heal.."HP)", mx, my+i*24)
                else
                    love.graphics.setColor(0.7,0.7,0.7)
                    love.graphics.print("  "..item.name, mx, my+i*24)
                end
            end
        end
    end

    -- Analysis overlay
    if self.showAnalysis then
        love.graphics.setColor(0,0,0,0.8)
        love.graphics.rectangle("fill",W/2-200,100,400,200,8,8)
        love.graphics.setColor(1,0.85,0.3)
        love.graphics.printf("ANALYSIS: "..self.enemy.name, W/2-190, 115, 380, "center")
        love.graphics.setColor(0.9,0.9,0.85)
        love.graphics.printf("ATK: "..self.enemy.atk.."  DEF: "..self.enemy.def..
            "\nHP: "..self.enemy.hp.."/"..self.enemy.maxHP..
            "\n\nPattern: "..table.concat(self.enemy.pattern, " → ")..
            "\n\nTip: "..self.enemy.weakness, W/2-180, 140, 360)
        love.graphics.setColor(0.5,0.5,0.5)
        love.graphics.printf("[Enter/Esc to close]", W/2-190, 275, 380, "center")
    end

    -- State banners
    if self.state == "intro" then
        love.graphics.setColor(1,0.3,0.2)
        love.graphics.printf("! ENCOUNTER !", 0, H*0.08, W, "center")
    elseif self.state == "victory" then
        love.graphics.setColor(0.3,1,0.4)
        love.graphics.printf("VICTORY", 0, H*0.08, W, "center")
    elseif self.state == "defeat" then
        love.graphics.setColor(1,0.1,0.1)
        love.graphics.printf("DEFEATED", 0, H*0.08, W, "center")
    end

    love.graphics.setColor(1,1,1)
end

return Combat
