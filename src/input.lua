local function isPressed(primary, alternate)
    return love.keyboard.isDown(primary) or love.keyboard.isDown(alternate)
end

return isPressed
