#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

// Potencia entera
unsigned long long int_pow(int base, int exp) {
    unsigned long long result = 1;
    for (int i = 0; i < exp; i++) {
        result *= base;
    }
    return result;
}

// Multiplicadores para unidades
unsigned long long unit_multiplier(const char *unit) {
    if (strcmp(unit, "b") == 0) return 1ULL;
    if (strcmp(unit, "B") == 0) return 8ULL; // 1 byte = 8 bits
    if (strcmp(unit, "Kb") == 0) return 1024ULL;
    if (strcmp(unit, "KB") == 0) return 1024ULL * 8ULL;
    if (strcmp(unit, "Mb") == 0) return 1024ULL * 1024ULL;
    if (strcmp(unit, "MB") == 0) return 1024ULL * 1024ULL * 8ULL;
    if (strcmp(unit, "Gb") == 0) return 1024ULL * 1024ULL * 1024ULL;
    if (strcmp(unit, "GB") == 0) return 1024ULL * 1024ULL * 1024ULL * 8ULL;
    return 0ULL;
}

// Mostrar ayuda
void mostrar_ayuda() {
    printf("\nTabla de conversiones (todas en bytes):\n");
    printf("---------------------------------------\n");
    printf("  1 byte (B)      = 1 byte\n");
    printf("  1 kilobyte (KB) = 1024 bytes\n");
    printf("  1 megabyte (MB) = 1,048,576 bytes\n");
    printf("  1 gigabyte (GB) = 1,073,741,824 bytes\n");
    printf("\n");
    printf("  1 bit (b)       = 0.125 bytes\n");
    printf("  1 kilobit (Kb)  = 128 bytes\n");
    printf("  1 megabit (Mb)  = 131,072 bytes\n");
    printf("  1 gigabit (Gb)  = 134,217,728 bytes\n");
    printf("\nUnidades soportadas: bit (b), byte (B), kilobit (Kb), kilobyte (KB), megabit (Mb), megabyte (MB), gigabit (Gb), gigabyte (GB)\n");
    printf("Puedes usar potencias, por ejemplo: 2^10 kilobyte\n");
    printf("Escribe 'salir' para terminar.\n\n");
}

// Parsear cantidad (soporta potencias)
long long parse_cantidad(const char *str) {
    int base, exp;
    if (sscanf(str, "%d^%d", &base, &exp) == 2) {
        if (base <= 0 || exp < 0) return -1;
        return int_pow(base, exp);
    } else {
        long long val = atoll(str);
        if (val <= 0) return -1;
        return val;
    }
}

// Normalizar unidad (acepta diminutivo y nombre completo)
void normalizar_unidad(char *unit) {
    // Convertir a minúsculas para comparar
    char lower[32];
    int len = strlen(unit);
    for (int i = 0; i < len; i++) {
        lower[i] = (unit[i] >= 'A' && unit[i] <= 'Z') ? unit[i] + 32 : unit[i];
    }
    lower[len] = '\0';
    // Mapear nombres completos y diminutivos a diminutivo estándar
    if (strcmp(lower, "bit") == 0 || strcmp(unit, "b") == 0) strcpy(unit, "b");
    else if (strcmp(lower, "byte") == 0 || strcmp(unit, "B") == 0) strcpy(unit, "B");
    else if (strcmp(lower, "kilobit") == 0 || strcmp(unit, "Kb") == 0) strcpy(unit, "Kb");
    else if (strcmp(lower, "kilobyte") == 0 || strcmp(unit, "KB") == 0) strcpy(unit, "KB");
    else if (strcmp(lower, "megabit") == 0 || strcmp(unit, "Mb") == 0) strcpy(unit, "Mb");
    else if (strcmp(lower, "megabyte") == 0 || strcmp(unit, "MB") == 0) strcpy(unit, "MB");
    else if (strcmp(lower, "gigabit") == 0 || strcmp(unit, "Gb") == 0) strcpy(unit, "Gb");
    else if (strcmp(lower, "gigabyte") == 0 || strcmp(unit, "GB") == 0) strcpy(unit, "GB");
}

// Función para imprimir números con comas
void print_with_commas(unsigned long long n) {
    char buf[32];
    char out[32];
    sprintf(buf, "%llu", n);
    int len = strlen(buf);
    int commas = (len - 1) / 3;
    int j = 0, k = 0;
    for (int i = 0; i < len; i++) {
        out[j++] = buf[i];
        if (((len - i - 1) % 3 == 0) && (i != len - 1)) {
            out[j++] = ',';
        }
    }
    out[j] = '\0';
    printf("%s", out);
}

// Devuelve el exponente si n es potencia exacta de 2, si no devuelve -1
typedef struct { int is_exact; double exp; } Potencia2Result;
Potencia2Result potencia2(unsigned long long n) {
    if (n == 0) return (Potencia2Result){0, 0};
    double exp = log2((double)n);
    unsigned long long pot = 1ULL << (unsigned long long)(exp + 0.5);
    if (pot == n) return (Potencia2Result){1, exp};
    return (Potencia2Result){0, exp};
}

// Devuelve el nombre completo de la unidad
const char* nombre_completo_unidad(const char* unidad) {
    if (strcmp(unidad, "b") == 0) return "bit";
    if (strcmp(unidad, "B") == 0) return "byte";
    if (strcmp(unidad, "Kb") == 0) return "kilobit";
    if (strcmp(unidad, "KB") == 0) return "kilobyte";
    if (strcmp(unidad, "Mb") == 0) return "megabit";
    if (strcmp(unidad, "MB") == 0) return "megabyte";
    if (strcmp(unidad, "Gb") == 0) return "gigabit";
    if (strcmp(unidad, "GB") == 0) return "gigabyte";
    return unidad;
}

// Conversión y salida con formato personalizado
void convertir(long long cantidad, const char *unidad) {
    unsigned long long bits = cantidad * unit_multiplier(unidad);
    if (bits == 0) {
        printf("Unidad no reconocida. Usa 'b', 'B', 'Kb', 'KB', 'Mb', 'MB', 'Gb', 'GB'.\n");
        return;
    }
    printf("\nEquivalencias para %lld %s (%s):\n\n", cantidad, unidad, nombre_completo_unidad(unidad));

    // Sección de bits
    printf("[ Equivalencias en bits ]\n");
    struct {
        const char *nombre;
        double valor;
        const char *unidad;
    } bits_eq[] = {
        {"bits", (double)bits, "b"},
        {"kilobits", (double)bits / 1024.0, "Kb"},
        {"megabits", (double)bits / (1024.0 * 1024.0), "Mb"},
        {"gigabits", (double)bits / (1024.0 * 1024.0 * 1024.0), "Gb"},
    };
    int n_bits = sizeof(bits_eq) / sizeof(bits_eq[0]);
    for (int i = 0; i < n_bits; i++) {
        Potencia2Result pot = potencia2((unsigned long long)(bits_eq[i].valor));
        printf("  %lld %s = %.6f %s (%s)", cantidad, unidad, bits_eq[i].valor, bits_eq[i].unidad, bits_eq[i].nombre);
        if (bits_eq[i].valor >= 1.0) {
            if (pot.is_exact) {
                printf(" = 2^%.0f", pot.exp);
            } else {
                printf(" ≈ 2^%.2f", pot.exp);
            }
        }
        printf("\n");
    }

    // Sección de bytes
    printf("\n[ Equivalencias en bytes ]\n");
    struct {
        const char *nombre;
        double valor;
        const char *unidad;
    } bytes_eq[] = {
        {"bytes", (double)bits / 8.0, "B"},
        {"kilobytes", (double)bits / (1024.0 * 8.0), "KB"},
        {"megabytes", (double)bits / (1024.0 * 1024.0 * 8.0), "MB"},
        {"gigabytes", (double)bits / (1024.0 * 1024.0 * 1024.0 * 8.0), "GB"},
    };
    int n_bytes = sizeof(bytes_eq) / sizeof(bytes_eq[0]);
    for (int i = 0; i < n_bytes; i++) {
        Potencia2Result pot = potencia2((unsigned long long)(bytes_eq[i].valor));
        printf("  %lld %s = %.6f %s (%s)", cantidad, unidad, bytes_eq[i].valor, bytes_eq[i].unidad, bytes_eq[i].nombre);
        if (bytes_eq[i].valor >= 1.0) {
            if (pot.is_exact) {
                printf(" = 2^%.0f", pot.exp);
            } else {
                printf(" ≈ 2^%.2f", pot.exp);
            }
        }
        printf("\n");
    }
}

int main() {
    char entrada[64];
    char cantidad_str[32], unidad[16];
    long long cantidad;

    printf("Conversor de Bits y Bytes\n");
    printf("-------------------------\n");
    mostrar_ayuda();

    while (1) {
        printf("> ");
        if (!fgets(entrada, sizeof(entrada), stdin)) break;
        entrada[strcspn(entrada, "\n")] = 0;

        if (strcmp(entrada, "salir") == 0 || strcmp(entrada, "exit") == 0) {
            printf("Saliendo del programa.\n");
            break;
        }
        if (strcmp(entrada, "ayuda") == 0 || strcmp(entrada, "help") == 0) {
            mostrar_ayuda();
            continue;
        }
        if (sscanf(entrada, "%31s %15s", cantidad_str, unidad) == 2) {
            normalizar_unidad(unidad);
            cantidad = parse_cantidad(cantidad_str);
            if (cantidad <= 0) {
                printf("Por favor ingresa una cantidad estrictamente positiva.\n");
                continue;
            }
            convertir(cantidad, unidad);
        } else {
            printf("Entrada no válida. Escribe por ejemplo: 8 bits, 10 bytes, 2^10 KB\n");
            printf("Escribe 'ayuda' para ver ejemplos y unidades soportadas.\n");
        }
    }
    return 0;
}
