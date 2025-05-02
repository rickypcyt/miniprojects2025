// src/main.c

#include <clay/clay.h>
#include <wayland-client.h>
#include <pixman.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

// Función de error para Wayland
void fatal(const char *msg) {
    fprintf(stderr, "%s\n", msg);
    exit(1);
}

int main(void) {
    // Inicializamos Clay
    if (clay_init() != 0) {
        fatal("Error al inicializar Clay");
    }

    // Creamos una ventana
    struct clay_window *window = clay_create_window(800, 600, "Ventana de prueba Clay");

    // Bucle principal de eventos
    while (!clay_should_close(window)) {
        // Limpiar la pantalla
        clay_clear_color(0.1f, 0.1f, 0.1f, 1.0f);

        // Dibujar texto
        clay_begin();
        clay_font_size(32);
        clay_color_hex(0xffffff);
        clay_text(300, 300, "¡Hola desde Clay!");
        clay_end();

        // Mostrar el contenido
        clay_swap_buffers(window);

        // Procesar eventos
        clay_poll_events(window);
    }

    // Limpiar
    clay_destroy_window(window);
    clay_terminate();

    return 0;
}

