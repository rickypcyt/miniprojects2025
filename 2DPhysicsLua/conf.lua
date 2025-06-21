function love.conf(t)
    t.window.width = 800
    t.window.height = 600
    t.window.title = "Círculo Ultra Smooth"
    t.window.resizable = false
    t.window.fullscreen = false
    t.window.fullscreentype = "desktop"
    
    -- Máxima calidad de antialiasing
    t.window.msaa = 16
    t.window.highdpi = true -- Importante para pantallas de alta resolución
    
    -- Sincronización y buffers para evitar tearing
    t.window.vsync = true
    t.modules.joystick = false
    t.modules.physics = false
end 