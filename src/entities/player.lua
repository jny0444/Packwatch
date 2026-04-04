local anim8 = require "lib.anim8"

local constants = require "src.constants.constants"
local isPressed = require "src.core.input"

local Player = {}
Player.__index = Player

-- Direction vectors for grid movement
local DIR_VECTORS = {
    up    = { dx =  0, dy = -1 },
    down  = { dx =  0, dy =  1 },
    left  = { dx = -1, dy =  0 },
    right = { dx =  1, dy =  0 },
}

function Player.new()
    local gridSize = constants.GRID_SIZE

    -- Snap the starting position to a grid cell
    local startGridX = 3
    local startGridY = 3

    local self = setmetatable({
        -- Grid coordinates (which cell the player is in / moving toward)
        gridX = startGridX,
        gridY = startGridY,

        -- Pixel position (for smooth interpolation)
        x = startGridX * gridSize,
        y = startGridY * gridSize,

        -- Movement state
        moving = false,
        moveTimer = 0,
        moveDuration = 0.12, -- seconds to cross one tile
        startX = 0,
        startY = 0,
        targetX = 0,
        targetY = 0,

        direction = "down",
    }, Player)

    self.sprite = love.graphics.newImage(constants.PLAYER_SPRITE_PATH)

    local grid = anim8.newGrid(
        constants.TILE_SIZE,
        constants.TILE_SIZE,
        self.sprite:getWidth(),
        self.sprite:getHeight()
    )

    self.animations = {
        down  = anim8.newAnimation(grid("1-3", 1), 0.05),
        left  = anim8.newAnimation(grid("1-3", 2), 0.05),
        right = anim8.newAnimation(grid("1-3", 3), 0.05),
        up    = anim8.newAnimation(grid("1-3", 4), 0.05),
    }

    self.currentAnim = self.animations.down
    return self
end

--- Try to begin a one-tile move in the given direction.
function Player:beginMove(dir)
    if self.moving then return end

    local vec = DIR_VECTORS[dir]
    if not vec then return end

    local gridSize = constants.GRID_SIZE

    self.direction = dir
    self.moving = true
    self.moveTimer = 0

    self.startX = self.x
    self.startY = self.y

    self.gridX = self.gridX + vec.dx
    self.gridY = self.gridY + vec.dy

    self.targetX = self.gridX * gridSize
    self.targetY = self.gridY * gridSize

    self.currentAnim = self.animations[self.direction]
end

--- Returns the direction currently being held, or nil.
function Player:heldDirection()
    if isPressed("right", "d") then return "right" end
    if isPressed("left",  "a") then return "left"  end
    if isPressed("up",    "w") then return "up"     end
    if isPressed("down",  "s") then return "down"   end
    return nil
end

function Player:move(dt)
    if self.moving then
        self.moveTimer = self.moveTimer + dt
        local t = self.moveTimer / self.moveDuration

        if t >= 1 then
            -- Snap to target
            self.x = self.targetX
            self.y = self.targetY
            self.moving = false

            -- If a key is still held, immediately begin next move for fluidity
            local held = self:heldDirection()
            if held then
                self:beginMove(held)
            end
        else
            -- Linearly interpolate toward the target
            self.x = self.startX + (self.targetX - self.startX) * t
            self.y = self.startY + (self.targetY - self.startY) * t
        end

        return true
    else
        -- Not moving — check for a new input
        local held = self:heldDirection()
        if held then
            self:beginMove(held)
            return true
        end
        return false
    end
end

function Player:updateAnimation(dt, moving)
    if moving then
        self.currentAnim:update(dt)
        return
    end

    self.currentAnim = self.animations[self.direction]
    self.currentAnim:gotoFrame(constants.IDLE_FRAME)
end

function Player:draw()
    self.currentAnim:draw(
        self.sprite,
        math.floor(self.x),
        math.floor(self.y)
    )
end

return Player
