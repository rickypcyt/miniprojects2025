#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

int main() {
    char entrada[64];
    int cantidad;
    char unidad[16];

    printf("Conversor de Bits y Bytes\n");
    printf("-------------------------\n");
    printf("Escribe un número y su unidad (bits o bytes), por ejemplo: 8 bits, 10 bytes\n");
    printf("También puedes usar potencias, por ejemplo: 2^10 bits\n");
    printf("Escribe 'salir' para terminar.\n\n");

    while (1) {
        printf("> ");
        fgets(entrada, sizeof(entrada), stdin);
        // Eliminar salto de línea
        entrada[strcspn(entrada, "\n")] = 0;

        if (strcmp(entrada, "salir") == 0 || strcmp(entrada, "exit") == 0) {
            printf("Saliendo del programa.\n");
            break;
        }

        // Intentar leer potencia o número y unidad
        char base_exp[32];
        if (sscanf(entrada, "%31s %15s", base_exp, unidad) == 2) {
            // Normalizar unidad a minúsculas
            for (int i = 0; unidad[i]; i++) {
                if (unidad[i] >= 'A' && unidad[i] <= 'Z') {
                    unidad[i] = unidad[i] - 'A' + 'a';
                }
            }
            char *caret = strchr(base_exp, '^');
            if (caret) {
                int base, exp;
                if (sscanf(base_exp, "%d^%d", &base, &exp) == 2) {
                    cantidad = (int)pow(base, exp);
                } else {
                    printf("Formato de potencia no válido. Usa por ejemplo: 2^10 bits\n");
                    continue;
                }
            } else {
                cantidad = atoi(base_exp);
            }
            if (strcmp(unidad, "bits") == 0 || strcmp(unidad, "bit") == 0) {
                if (cantidad == 1) {
                    printf("1 bit es igual a 0.125 bytes.\n");
                } else if (cantidad > 1) {
                    if (cantidad % 8 == 0) {
                        printf("%d bits son igual a %d bytes.\n", cantidad, cantidad / 8);
                    } else {
                        printf("%d bits son igual a %.3f bytes.\n", cantidad, cantidad / 8.0);
                    }
                } else {
                    printf("Por favor ingresa un número entero positivo de bits.\n");
                }
            } else if (strcmp(unidad, "bytes") == 0 || strcmp(unidad, "byte") == 0) {
                if (cantidad == 1) {
                    printf("1 byte es igual a 8 bits.\n");
                } else if (cantidad > 1) {
                    printf("%d bytes son igual a %d bits.\n", cantidad, cantidad * 8);
                } else {
                    printf("Por favor ingresa un número entero positivo de bytes.\n");
                }
            } else {
                printf("Unidad no reconocida. Usa 'bits' o 'bytes'.\n");
            }
        } else {
            printf("Entrada no válida. Escribe por ejemplo: 8 bits, 10 bytes, 2^10 bits\n");
        }
    }
    return 0;
}
