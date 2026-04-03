local constants = require "src.constants"

local function createFonts()
    return {
        small = love.graphics.newFont(constants.FONT_PATH, 24),
        medium = love.graphics.newFont(constants.FONT_PATH, 48),
        large = love.graphics.newFont(constants.FONT_PATH, 96),
    }
end

return createFonts
