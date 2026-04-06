_G.love = require "love"
local constants = require "src.constants.constants"
local createFonts = require "src.ui.fonts"
local Player = require "src.entities.player"
local Grid = require "src.world.grid"
local Camera = require "lib.hump.camera"
local cam

local fonts = {}
local player = nil
local grid = nil

function love.load()
    love.graphics.setDefaultFilter("nearest", "nearest")
    fonts = createFonts()
    player = Player.new()
    grid = Grid.new()
    cam = Camera(love.graphics.getWidth()/2, love.graphics.getHeight()/2)
end

function love.update(dt)
    local moving = player:move(dt)
    player:updateAnimation(dt, moving)
    cam:lookAt(player.x + constants.TILE_SIZE / 2, player.y + constants.TILE_SIZE / 2)
end

function love.draw()
    love.graphics.setFont(fonts.large)
    love.graphics.print(constants.TITLE_TEXT, constants.TITLE_X, constants.TITLE_Y)

    cam:attach()
        player:draw()
        grid:draw()
    cam:detach()
end

function love.keypressed(key)
    if key == constants.TOGGLE_GRID_KEY then
        grid:toggle()
    end
end
