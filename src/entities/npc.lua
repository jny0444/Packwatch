local NPC = {}
NPC.__index = NPC

function NPC.new(x, y, behaviour)
    local obj = {
        x = x,
        y = y,
        state = "idle"
    }
end