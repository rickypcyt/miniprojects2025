// C program that shows a desktop notification when it's time to go to the gym to eat to turn off
// the screen and sleep.

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
    // char *mensaje1 = NULL; En este caso se imprime el error
    char mensaje1[] = "";

    if (mensaje1 != NULL && mensaje1[0] != '\0') {
        puts(mensaje1);
    } else {
        printf("ERROR: No hay mensaje que printear\n");
    };
    return 0;
}
