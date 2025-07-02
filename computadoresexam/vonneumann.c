#include <stdio.h>

#define MEM_SIZE 16

// Instrucciones
#define HALT 0
#define LOAD 1
#define ADD 2
#define STORE 3
#define PRINT 4

int main() {
    int memoria[MEM_SIZE] = {
        LOAD, 8,      // 0: Cargar memoria[8] en acumulador
        ADD, 9,       // 2: Sumar memoria[9] al acumulador
        STORE, 10,    // 4: Guardar acumulador en memoria[10]
        PRINT,        // 6: Imprimir acumulador
        HALT,         // 7: Parar
        2,            // 8: memoria[8] = 2
        2,            // 9: memoria[9] = 2
        0,            // 10: memoria[10] (resultado)
        0, 0, 0, 0, 0, 0
    };
    int pc = 0; // Program Counter
    int acc = 0; // Acumulador
    int running = 1;

    while (running) {
        int instr = memoria[pc];
        printf("\nPC: %d | Instrucción: ", pc);
        switch (instr) {
            case LOAD:
                printf("LOAD (Cargar)\n");
                printf("  Argumento: memoria[%d] = %d\n", memoria[pc + 1], memoria[memoria[pc + 1]]);
                printf("  Acumulador antes: %d\n", acc);
                acc = memoria[memoria[pc + 1]];
                printf("  Acumulador después: %d\n", acc);
                pc += 2;
                break;
            case ADD:
                printf("ADD (Sumar)\n");
                printf("  Argumento: memoria[%d] = %d\n", memoria[pc + 1], memoria[memoria[pc + 1]]);
                printf("  Acumulador antes: %d\n", acc);
                acc += memoria[memoria[pc + 1]];
                printf("  Acumulador después: %d\n", acc);
                pc += 2;
                break;
            case STORE:
                printf("STORE (Guardar)\n");
                printf("  Guardando acumulador (%d) en memoria[%d]\n", acc, memoria[pc + 1]);
                memoria[memoria[pc + 1]] = acc;
                printf("  memoria[%d] ahora vale: %d\n", memoria[pc + 1], memoria[memoria[pc + 1]]);
                pc += 2;
                break;
            case PRINT:
                printf("PRINT (Imprimir)\n");
                printf("  Acumulador: %d\n", acc);
                pc += 1;
                break;
            case HALT:
                printf("HALT (Parar)\n");
                running = 0;
                break;
            default:
                printf("Instrucción desconocida en %d\n", pc);
                running = 0;
        }
    }
    printf("\nEjecución finalizada.\n");
    printf("Memoria[10] (resultado): %d\n", memoria[10]);
    return 0;
}
