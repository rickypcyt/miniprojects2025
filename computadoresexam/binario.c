#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

// Convierte decimal a binario (complemento a dos) en n_bits
void decimal_a_binario(int n, int n_bits) {
    unsigned int mask = (1U << n_bits) - 1;
    unsigned int valor = (unsigned int)n & mask;
    for (int i = n_bits - 1; i >= 0; i--) {
        printf("%d", (valor >> i) & 1);
    }
}

// Convierte binario (complemento a dos) a decimal
int binario_a_decimal(const char *bin, int n_bits) {
    int longitud = strlen(bin);
    if (longitud != n_bits) {
        printf("Advertencia: la cantidad de bits no coincide con la entrada. Se usará la longitud de la entrada.\n");
        n_bits = longitud;
    }
    int negativo = (bin[0] == '1');
    int decimal = 0;
    if (!negativo) {
        // Positivo
        for (int i = 0; i < n_bits; i++) {
            if (bin[i] == '1') {
                decimal += 1 << (n_bits - i - 1);
            }
        }
    } else {
        // Negativo: complemento a dos
        // Invertir bits y sumar 1
        int valor = 0;
        for (int i = 0; i < n_bits; i++) {
            if (bin[i] == '0') {
                valor += 1 << (n_bits - i - 1);
            }
        }
        valor += 1;
        decimal = -valor;
    }
    return decimal;
}

// Pide un entero con mensaje
int pedir_entero(const char *mensaje) {
    char entrada[40];
    while (1) {
        printf("%s", mensaje);
        fgets(entrada, sizeof(entrada), stdin);
        entrada[strcspn(entrada, "\n")] = 0;
        char *endptr;
        int valor = strtol(entrada, &endptr, 10);
        if (*endptr == '\0') return valor;
        printf("Entrada no válida.\n");
    }
}

// Pide un binario válido de n_bits
void pedir_binario(char *dest, int n_bits) {
    while (1) {
        printf("binario (%d bits) -> decimal : ", n_bits);
        fgets(dest, n_bits + 10, stdin);
        dest[strcspn(dest, "\n")] = 0;
        int valido = 1;
        if ((int)strlen(dest) != n_bits) {
            printf("La entrada debe tener exactamente %d bits.\n", n_bits);
            continue;
        }
        for (int i = 0; dest[i] != '\0'; i++) {
            if (dest[i] != '0' && dest[i] != '1') {
                valido = 0;
                break;
            }
        }
        if (!valido) {
            printf("Entrada no válida.\n");
            continue;
        }
        break;
    }
}

int main() {
    while (1) {
        int opcion;
        printf("\nElige una opción:\n");
        printf("1. Decimal a Binario (complemento a dos)\n");
        printf("2. Binario a Decimal (complemento a dos)\n");
        printf("3. Salir\n");
        printf("Opción: ");
        scanf("%d", &opcion);
        getchar(); // Limpiar el buffer
        if (opcion == 1) {
            int n_bits = pedir_entero("¿Cuántos bits quieres usar? (ej: 8): ");
            while (1) {
                char entrada[40];
                printf("decimal -> binario (%d bits, complemento a dos) : ", n_bits);
                fgets(entrada, sizeof(entrada), stdin);
                entrada[strcspn(entrada, "\n")] = 0;
                if (strcmp(entrada, "back") == 0) break;
                char *endptr;
                int decimal = strtol(entrada, &endptr, 10);
                if (*endptr != '\0') {
                    printf("Entrada no válida.\n");
                    continue;
                }
                int min = -(1 << (n_bits - 1));
                int max = (1 << (n_bits - 1)) - 1;
                if (decimal < min || decimal > max) {
                    printf("El número no cabe en %d bits (rango: %d a %d).\n", n_bits, min, max);
                    continue;
                }
                printf("El número en binario (%d bits, complemento a dos) es: ", n_bits);
                decimal_a_binario(decimal, n_bits);
                printf("\n");
            }
        } else if (opcion == 2) {
            while (1) {
                char entrada[40];
                printf("binario -> decimal : ");
                fgets(entrada, sizeof(entrada), stdin);
                entrada[strcspn(entrada, "\n")] = 0;
                if (strcmp(entrada, "back") == 0) break;
                int n_bits = strlen(entrada);
                int valido = 1;
                for (int i = 0; entrada[i] != '\0'; i++) {
                    if (entrada[i] != '0' && entrada[i] != '1') {
                        valido = 0;
                        break;
                    }
                }
                if (!valido || n_bits == 0) {
                    printf("Entrada no válida.\n");
                    continue;
                }
                int decimal = binario_a_decimal(entrada, n_bits);
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
