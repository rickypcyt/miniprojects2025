-- TODOLIST:
-- Hacer que al darle click cambie de color la bola, que sea random con todo
-- RGB.
-- Seleccionar que modelo quieres ser, tringulo, cuadrado.
-- Pantalla inicial que te deje elehir que juego quieres jugar:
-- Hacer Pong
-- Flappy Bird
-- Doodle Jump
--

function love.load()
    -- Las dimensiones ahora se cogen de conf.lua
    w, h = love.graphics.getDimensions()

    -- Imprimir información de la GPU en la consola
    local name, version, vendor, device = love.graphics.getRendererInfo()
    print("--- GPU Info ---")
    print("Vendor: " .. vendor)
    print("Device: " .. device)
    print("OpenGL: " .. version)
    print("----------------")

    -- Configuración para gráficos más smooth
    love.graphics.setDefaultFilter("linear", "linear", 1)
    love.graphics.setLineStyle("smooth")
    love.graphics.setLineJoin("miter")
    love.graphics.setLineWidth(1)

    circle = {
        x = 0,
        y = 0,
        radius = 50,
    }

    -- Calcula bordes en coordenadas de pantalla
    border = {
        left = -(w / 2) + circle.radius,
        right = (w / 2) - circle.radius,
        top = -(h / 2) + circle.radius,
        bot = (h / 2) - circle.radius,
    }
    print("Bordes calculados:")
    print("Left: " .. border.left)
    print("Right: " .. border.right)
    print("Top: " .. border.top)
    print("Bottom: " .. border.bot)

    color = {
        black = { 0, 0, 0 },
        white = { 1, 1, 1 },
    }

    vel_y = 0
    vel_x = 0
    grav = 9.8 * 100
    
    -- La configuración de la ventana se ha movido a conf.lua
end

function love.draw()
    love.graphics.clear(color.black)
    love.graphics.translate(love.graphics.getWidth() / 2, love.graphics.getHeight() / 2)
    
    -- Configuración para dibujo ultra smooth
    love.graphics.setLineWidth(1)
    
    -- Efecto de glow suave
    love.graphics.setColor(0.3, 0.3, 0.3, 0.5)
    love.graphics.circle("fill", circle.x + 1, circle.y + 1, circle.radius + 2)
    
    -- Círculo principal con mejor calidad
    love.graphics.setColor(color.white)
    love.graphics.circle("fill", circle.x, circle.y, circle.radius)
    
    -- Borde suave para definición
    love.graphics.setColor(0.9, 0.9, 0.9, 0.8)
    love.graphics.circle("line", circle.x, circle.y, circle.radius)
    
    -- Debug info
    love.graphics.setColor(1, 1, 0, 1)
    love.graphics.print("Pos: (" .. math.floor(circle.x) .. ", " .. math.floor(circle.y) .. ")", -w/2 + 10, -h/2 + 10)
    love.graphics.print("Bordes (centro): L:" .. border.left .. " R:" .. border.right .. " T:" .. border.top .. " B:" .. border.bot, -w/2 + 10, -h/2 + 30)
    
    -- Mostrar GPU en pantalla
    local name, version, vendor, device = love.graphics.getRendererInfo()
    love.graphics.setColor(0, 1, 1, 1) -- Color cian para la info de la GPU
    love.graphics.print("GPU: " .. device, -w/2 + 10, -h/2 + 50)

    -- Mostrar FPS en pantalla
    love.graphics.setColor(0, 1, 0, 1) -- Color verde para los FPS
    love.graphics.print("FPS: " .. love.timer.getFPS(), -w/2 + 10, -h/2 + 70)

    -- Dibujar bordes de la ventana para que sea más intuitivo
    love.graphics.setColor(1, 0, 0, 0.5)
    love.graphics.rectangle("line", -w/2, -h/2, w, h)
end

function love.update(dt)
    -- Física más suave
    vel_y = vel_y + grav * dt -- gravedad
    
    -- Aplicar velocidad al movimiento
    circle.y = circle.y + vel_y * dt
    
    -- Colisión con bordes con rebote
    if circle.y > border.bot then
        circle.y = border.bot
        vel_y = -vel_y * 0.7 -- Rebote con pérdida de energía
    end
    if circle.y < border.top then
        circle.y = border.top
        vel_y = -vel_y * 0.7 -- Rebote con pérdida de energía
    end
    
    -- Colisión horizontal también
    if circle.x > border.right then
        circle.x = border.right
    end
    if circle.x < border.left then
        circle.x = border.left
    end
    
    -- Movimiento horizontal más suave
    local moveSpeed = 300 -- Píxeles por segundo
    if love.keyboard.isDown("w") and circle.y > border.top then
        circle.y = circle.y - moveSpeed * dt
    end
    if love.keyboard.isDown("a") and circle.x > border.left then
        circle.x = circle.x - moveSpeed * dt
    end
    if love.keyboard.isDown("s") and circle.y < border.bot then
        circle.y = circle.y + moveSpeed * dt
    end
    if love.keyboard.isDown("d") and circle.x < border.right then
        circle.x = circle.x + moveSpeed * dt
    end
    if love.keyboard.isDown("escape") then
        love.event.quit()
    end
end
