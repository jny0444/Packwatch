local anim8 = require "lib.anim8"

local constants = require "src.constants"
local isPressed = require "src.input"

local Player = {}
Player.__index = Player

function Player.new()
    local self = setmetatable({
        x = 100,
        y = 100,
        speed = 200,
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
        down = anim8.newAnimation(grid("1-3", 1), 0.05),
        left = anim8.newAnimation(grid("1-3", 2), 0.05),
        right = anim8.newAnimation(grid("1-3", 3), 0.05),
        up = anim8.newAnimation(grid("1-3", 4), 0.05),
    }

    self.currentAnim = self.animations.down
    return self
end

function Player:move(dt)
    if isPressed("right", "d") then
        self.x = self.x + self.speed * dt
        self.direction = "right"
    elseif isPressed("left", "a") then
        self.x = self.x - self.speed * dt
        self.direction = "left"
    elseif isPressed("up", "w") then
        self.y = self.y - self.speed * dt
        self.direction = "up"
    elseif isPressed("down", "s") then
        self.y = self.y + self.speed * dt
        self.direction = "down"
    else
        return false
    end

    self.currentAnim = self.animations[self.direction]
    return true
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
