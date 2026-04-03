_G.love = require "love"
local anim8 = require "lib.anim8"

function love.load()
    love.graphics.setDefaultFilter("nearest", "nearest")

    fonts = {
        small = love.graphics.newFont("assets/fonts/Pixelzone.ttf", 24),
        medium = love.graphics.newFont("assets/fonts/Pixelzone.ttf", 48),
        large = love.graphics.newFont("assets/fonts/Pixelzone.ttf", 96),

    }

    player = {
        x = 100,
        y = 100,
        speed = 200,
    }

    player.sprite = love.graphics.newImage("assets/sprites/player.png")

    local grid = anim8.newGrid(32, 32,
        player.sprite:getWidth(),
        player.sprite:getHeight()
    )

    player.animations = {
        down  = anim8.newAnimation(grid('1-3', 1), 0.1),
        left  = anim8.newAnimation(grid('1-3', 2), 0.1),
        right = anim8.newAnimation(grid('1-3', 3), 0.1),
        up    = anim8.newAnimation(grid('1-3', 4), 0.1),
    }

    player.currentAnim = player.animations.down

    player.direction = "down"
end

function love.update(dt)
    local moving = false

    if love.keyboard.isDown("right") or love.keyboard.isDown("d") then
        player.x = player.x + player.speed * dt
        player.currentAnim = player.animations.right
        player.direction = "right"
        moving = true
    elseif love.keyboard.isDown("left") or love.keyboard.isDown("a") then
        player.x = player.x - player.speed * dt
        player.currentAnim = player.animations.left
        player.direction = "left"
        moving = true
    elseif love.keyboard.isDown("up") or love.keyboard.isDown("w") then
        player.y = player.y - player.speed * dt
        player.currentAnim = player.animations.up
        player.direction = "up"
        moving = true
    elseif love.keyboard.isDown("down") or love.keyboard.isDown("s") then
        player.y = player.y + player.speed * dt
        player.currentAnim = player.animations.down
        player.direction = "down"
        moving = true
    end

    if moving then
        player.currentAnim:update(dt)
    else
        -- ✅ idle: switch to correct direction
        player.currentAnim = player.animations[player.direction]

        -- ✅ reset to middle frame (frame 2)
        player.currentAnim:gotoFrame(2)
    end
end

function love.draw()
    love.graphics.setFont(fonts.large)
    love.graphics.print("PackWatch", 50, 0)

    player.currentAnim:draw(
        player.sprite,
        math.floor(player.x),
        math.floor(player.y)
    )
end
