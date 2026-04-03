_G.love = require "love"
local constants = require "src.constants"
local createFonts = require "src.fonts"
local Player = require "src.player"
local Grid = require "src.grid"

local fonts = {}
local player = nil
local grid = nil

function love.load()
    love.graphics.setDefaultFilter("nearest", "nearest")
    fonts = createFonts()
    player = Player.new()
    grid = Grid.new()
end

function love.update(dt)
    local moving = player:move(dt)
    player:updateAnimation(dt, moving)
end

function love.draw()
    love.graphics.setFont(fonts.large)
    love.graphics.print(constants.TITLE_TEXT, constants.TITLE_X, constants.TITLE_Y)
    player:draw()
    grid:draw()
end

function love.keypressed(key)
    if key == constants.TOGGLE_GRID_KEY then
        grid:toggle()
    end
end
