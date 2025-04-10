// C program that shows a desktop notification when it's time to go to the gym to eat to turn off
// the screen and sleep.

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main() {
    char titulo[] = "C Alarm";
    puts(titulo);

    time_t DiaHoy;
    struct tm *info_tiempo;
    char buffer[80];

    time(&DiaHoy);

    info_tiempo = localtime(&DiaHoy);

    int hora = info_tiempo->tm_hour;

    strftime(buffer, sizeof(buffer), "%B %d, %Y - %H:%M:%S", info_tiempo);

    printf("%s\n", buffer);

    char alarmaDormir[] = "Anda al gym!!"

        if (hora == 9) {
        put
    }

    /* Codigo de prueba para manejar condicionalles y verificar que no sea nulo el mensaje pero
     obviamente no es necesario porque siempre estoy poniendo info en las cadenas de texto.
        // char *mensaje1 = NULL; En este caso se imprime el error
        char mensaje1[] = "\nAlarma 1";

        if (mensaje1 != NULL && mensaje1[0] != '\0') {
            puts(mensaje1);
        } else {
            printf("ERROR: No hay mensaje que printear\n");
        };
     */

    return 0;
}
