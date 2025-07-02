#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

void decimal_a_binario(int n) {
    if (n == 0) {
        printf("0");
        return;
    }
    int binario[32];
    int i = 0;
    while (n > 0) {
        binario[i] = n % 2;
        n = n / 2;
        i++;
    }
    for (int j = i - 1; j >= 0; j--) {
        printf("%d", binario[j]);
    }
}

int binario_a_decimal(const char *bin) {
    int decimal = 0;
    int longitud = strlen(bin);
    for (int i = 0; i < longitud; i++) {
        if (bin[i] == '1') {
            decimal += pow(2, longitud - i - 1);
        }
    }
    return decimal;
}

int main() {
    while (1) {
        int opcion;
        printf("\nElige una opción:\n");
        printf("1. Decimal a Binario\n");
        printf("2. Binario a Decimal\n");
        printf("3. Salir\n");
        printf("Opción: ");
        scanf("%d", &opcion);
        getchar(); // Limpiar el buffer
        if (opcion == 1) {
            char entrada[40];
            while (1) {
                printf("decimal -> binario : ");
                fgets(entrada, sizeof(entrada), stdin);
                entrada[strcspn(entrada, "\n")] = 0; // Quitar salto de línea
                if (strcmp(entrada, "back") == 0) break;
                char *endptr;
                int decimal = strtol(entrada, &endptr, 10);
                if (*endptr != '\0') {
                    printf("Entrada no válida.\n");
                    continue;
                }
                printf("El número en binario es: ");
                decimal_a_binario(decimal);
                printf("\n");
            }
        } else if (opcion == 2) {
            char entrada[40];
            while (1) {
                printf("binario -> decimal : ");
                fgets(entrada, sizeof(entrada), stdin);
                entrada[strcspn(entrada, "\n")] = 0; // Quitar salto de línea
                if (strcmp(entrada, "back") == 0) break;
                int valido = 1;
                for (int i = 0; entrada[i] != '\0'; i++) {
                    if (entrada[i] != '0' && entrada[i] != '1') {
                        valido = 0;
                        break;
                    }
                }
                if (!valido) {
                    printf("Entrada no válida.\n");
                    continue;
                }
                int decimal = binario_a_decimal(entrada);
                printf("El número en decimal es: %d\n", decimal);
            }
        } else if (opcion == 3) {
            printf("¡Hasta luego!\n");
            break;
        } else {
            printf("Opción no válida.\n");
        }
    }
    return 0;
}
