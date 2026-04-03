local constants = require "src.constants"

local Grid = {}
Grid.__index = Grid

function Grid.new()
    return setmetatable({
        enabled = false,
        size = constants.GRID_SIZE,
        color = constants.GRID_COLOR,
    }, Grid)
end

function Grid:toggle()
    self.enabled = not self.enabled
end

function Grid:draw()
    if not self.enabled then
        return
    end

    local width = love.graphics.getWidth()
    local height = love.graphics.getHeight()
    local oldR, oldG, oldB, oldA = love.graphics.getColor()

    love.graphics.setColor(self.color)

    for x = 0, width, self.size do
        love.graphics.line(x, 0, x, height)
    end

    for y = 0, height, self.size do
        love.graphics.line(0, y, width, y)
    end

    love.graphics.setColor(oldR, oldG, oldB, oldA)
end

return Grid
