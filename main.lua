_G.love = require "love"

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
        width = 32,
        height = 32,
    }
end

function love.update(dt)
    if love.keyboard.isDown("right") then
        player.x = player.x + player.speed * dt
    end

    if love.keyboard.isDown("left") then
        player.x = player.x - player.speed * dt
    end

    if love.keyboard.isDown("up") then
        player.y = player.y - player.speed * dt
    end

    if love.keyboard.isDown("down") then
        player.y = player.y + player.speed * dt
    end
end

function love.draw()
    love.graphics.setFont(fonts.large)
    love.graphics.print("PackWatch", 50, 0)

    love.graphics.rectangle(
        "fill",
        math.floor(player.x),
        math.floor(player.y),
        player.width,
        player.height
    )
end
