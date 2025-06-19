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
    -- 800 es ancho y 600 es alto
    w = 800
    h = 600

    circle = {
        x = 0,
        y = 0,
        radius = 50,
    }

    -- Calcula bordes en coordenadas de pantalla
    border = {
        left = -(w / 2) + circle.radius, -- -910
        right = (w / 2) - circle.radius, --  910
        top = -(h / 2) + circle.radius, -- -490
        bot = (h / 2) - circle.radius, --  490    }
    }
    print(border.right)

    color = {
        black = { 0, 0, 0 },
        white = { 1, 1, 1 },
    }

    vel_y = 0
    vel_x = 0
    grav = 9.8 * 100
    -- primero va el ancho luego el alto 800x600
    love.window.setMode(w, h, { resizable = false, msaa = 8 })
    love.window.setTitle("Círculo Centrado")
end

function love.draw()
    love.graphics.clear(color.black)
    love.graphics.translate(love.graphics.getWidth() / 2, love.graphics.getHeight() / 2)
    love.graphics.setColor(color.white)
    love.graphics.circle("fill", circle.x, circle.y, circle.radius)
end

function love.update(dt)
    vel_y = vel_y + grav * dt -- gravedad

    circle.x = math.max(math.min(circle.x, border.right), border.left)
    circle.y = math.max(math.min(circle.y, border.bot), border.top)

    if love.keyboard.isDown("w") and circle.y > border.top then
        circle.y = circle.y - 10
    end
    if love.keyboard.isDown("a") and circle.x > border.left then
        circle.x = circle.x - 10
    end
    if love.keyboard.isDown("s") and circle.y < border.bot then
        circle.y = circle.y + 10
    end
    if love.keyboard.isDown("d") and circle.x < border.right then
        circle.x = circle.x + 10
    end
    if love.keyboard.isDown("escape") then
        os.exit()
    end
end
