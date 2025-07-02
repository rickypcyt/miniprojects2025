/*
Simulador didáctico de una máquina de Von Neumann
--------------------------------------------------
- Modelo Von Neumann: memoria única para instrucciones y datos.
- Ciclo fetch-decode-execute: la CPU obtiene, decodifica y ejecuta instrucciones.
- Registros:
    - PC (Program Counter): apunta a la siguiente instrucción a ejecutar.
    - ACC (Acumulador): registro principal para operaciones aritméticas.
    - Z (Zero flag): indica si el ACC es cero tras una operación.
- Instrucciones:
    - LOAD dir   : ACC = mem[dir]
    - ADD dir    : ACC += mem[dir]
    - SUB dir    : ACC -= mem[dir]
    - STORE dir  : mem[dir] = ACC
    - PRINT      : imprime ACC
    - HALT       : detiene la ejecución
    - JMP dir    : salta a dir
    - JZ dir     : salta a dir si Z==1
    - NOP        : no hace nada
- Estructura de memoria:
    - Las primeras posiciones suelen ser instrucciones (código), el resto datos.
    - Puedes agregar etiquetas a posiciones de memoria para identificarlas.
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define MEM_SIZE 19
#define CODE_SIZE 8 // Primeras 8 posiciones son código, el resto datos
#define MAX_LABELS MEM_SIZE
#define MAX_LABEL_LEN 16

// Instrucciones
#define HALT 0
#define LOAD 1
#define ADD 2
#define STORE 3
#define PRINT 4
#define SUB 5
#define JMP 6
#define JZ 7
#define NOP 8

const char *instr_names[] = {"HALT", "LOAD", "ADD", "STORE", "PRINT", "SUB", "JMP", "JZ", "NOP"};
// Longitud de cada instrucción (en posiciones de memoria)
const int instr_length[] = {1, 2, 2, 2, 1, 2, 2, 2, 1};

#define MAX_STEPS 128

typedef struct {
    int pc;
    int instr;
    int arg;
    int acc;
    int z;
    char info[64]; // Para cambios relevantes
} StepInfo;

// Etiquetas para memoria
char mem_labels[MAX_LABELS][MAX_LABEL_LEN] = {""};

// Prototipos
void print_program(int *mem);
void print_memory(int *mem, int pc);
void exec_load(int *mem, int *acc, int *pc, int *z);
void exec_add(int *mem, int *acc, int *pc, int *z);
void exec_sub(int *mem, int *acc, int *pc, int *z);
void exec_store(int *mem, int *acc, int *pc, int *z);
void exec_print(int *mem, int *acc, int *pc, int *z);
void exec_halt(int *running);
void exec_jmp(int *mem, int *pc);
void exec_jz(int *mem, int *pc, int z);
void exec_nop(int *pc);
int valid_addr(int addr);
void set_mem_label(int addr, const char *label);

// Prototipos de ejemplos
void cargar_ejemplo_suma(int *mem, int *mem_ptr);
void cargar_ejemplo_resta(int *mem, int *mem_ptr);
void cargar_ejemplo_bucle(int *mem, int *mem_ptr);

int main() {
    int memoria[MEM_SIZE] = {0}; // Memoria vacía
    int mem_ptr = 0; // Puntero para cargar instrucciones
    char input[64];
    memset(mem_labels, 0, sizeof(mem_labels));
    printf("Simulador Von Neumann interactivo\n");
    printf("Ingrese 'help' para ver ayuda y ejemplos.\n");
    print_memory(memoria, -1);
    while (1) {
        printf("> ");
        if (!fgets(input, sizeof(input), stdin)) break;
        input[strcspn(input, "\n")] = 0;
        // Comando help
        if (strcmp(input, "help") == 0) {
            printf("\nComandos disponibles:\n");
            printf("  help                Muestra esta ayuda\n");
            printf("  run                 Ejecuta el programa cargado\n");
            printf("  clear               Limpia la memoria\n");
            printf("  exit                Sale del simulador\n");
            printf("  example <nombre>    Carga un ejemplo en memoria\n");
            printf("  <dir>: <valor>      Escribe un dato en memoria\n");
            printf("  <dir>: <etiqueta>   Asigna una etiqueta a una dirección\n");
            printf("\nPara cargar instrucciones:   LOAD 8\n");
            printf("Para cargar datos:           8: 5\n");
            printf("Para etiquetar memoria:      8: resultado\n");
            printf("\nEjemplos disponibles:\n");
            printf("  example suma        Suma 2 + 2 e imprime el resultado\n");
            printf("  example resta       Resta 10 - 5 e imprime el resultado\n");
            printf("  example bucle       Cuenta hacia atrás desde 3 hasta 0\n");
            continue;
        }
        // Comando example
        if (strncmp(input, "example ", 8) == 0) {
            memset(memoria, 0, sizeof(memoria));
            memset(mem_labels, 0, sizeof(mem_labels));
            mem_ptr = 0;
            if (strcmp(input+8, "suma") == 0) {
                cargar_ejemplo_suma(memoria, &mem_ptr);
                set_mem_label(8, "dato1");
                set_mem_label(9, "dato2");
                set_mem_label(10, "resultado");
                printf("Ejemplo 'suma' cargado.\n");
            } else if (strcmp(input+8, "resta") == 0) {
                cargar_ejemplo_resta(memoria, &mem_ptr);
                set_mem_label(8, "minuendo");
                set_mem_label(9, "sustraendo");
                set_mem_label(10, "resultado");
                printf("Ejemplo 'resta' cargado.\n");
            } else if (strcmp(input+8, "bucle") == 0) {
                cargar_ejemplo_bucle(memoria, &mem_ptr);
                set_mem_label(8, "contador");
                set_mem_label(9, "decremento");
                printf("Ejemplo 'bucle' cargado.\n");
            } else {
                printf("Ejemplo no reconocido. Usa 'help' para ver la lista.\n");
            }
            print_memory(memoria, -1);
            continue;
        }
        // Comando exit
        if (strcmp(input, "exit") == 0) break;
        // Comando clear
        if (strcmp(input, "clear") == 0) {
            memset(memoria, 0, sizeof(memoria));
            memset(mem_labels, 0, sizeof(mem_labels));
            mem_ptr = 0;
            printf("Memoria limpiada.\n");
            print_memory(memoria, -1);
            continue;
        }
        // Comando run
        if (strcmp(input, "run") == 0) {
            int pc = 0, acc = 0, z = 0, running = 1;
            StepInfo steps[MAX_STEPS];
            int step_count = 0;
            printf("\n--- EJECUCIÓN ---\n");
            while (running && step_count < MAX_STEPS) {
                print_memory(memoria, pc);
                int instr = memoria[pc];
                int arg = (instr_length[instr] == 2) ? memoria[pc+1] : -1;
                printf("\nFETCH: instrucción %s, argumento %d\n", (instr >= 0 && instr <= 8) ? instr_names[instr] : "???", arg);
                printf("DECODE: preparándose para ejecutar %s\n", (instr >= 0 && instr <= 8) ? instr_names[instr] : "???");
                printf("Actual:  PC=%d, ACC=%d, Z=%d\n", pc, acc, z);
                char info[64] = "";
                // EXECUTE
                switch (instr) {
                    case LOAD:
                        snprintf(info, sizeof(info), "ACC <- mem[%d] (%d)", arg, valid_addr(arg) ? memoria[arg] : 0);
                        exec_load(memoria, &acc, &pc, &z);
                        break;
                    case ADD:
                        snprintf(info, sizeof(info), "ACC += mem[%d] (%d)", arg, valid_addr(arg) ? memoria[arg] : 0);
                        exec_add(memoria, &acc, &pc, &z);
                        break;
                    case SUB:
                        snprintf(info, sizeof(info), "ACC -= mem[%d] (%d)", arg, valid_addr(arg) ? memoria[arg] : 0);
                        exec_sub(memoria, &acc, &pc, &z);
                        break;
                    case STORE:
                        snprintf(info, sizeof(info), "mem[%d] <- ACC (%d)", arg, acc);
                        exec_store(memoria, &acc, &pc, &z);
                        break;
                    case PRINT:
                        snprintf(info, sizeof(info), "PRINT ACC (%d)", acc);
                        exec_print(memoria, &acc, &pc, &z);
                        break;
                    case HALT:
                        snprintf(info, sizeof(info), "HALT");
                        exec_halt(&running); pc += instr_length[HALT]; break;
                    case JMP:
                        snprintf(info, sizeof(info), "JMP %d", arg);
                        exec_jmp(memoria, &pc);
                        break;
                    case JZ:
                        snprintf(info, sizeof(info), "JZ %d (Z=%d)", arg, z);
                        exec_jz(memoria, &pc, z);
                        break;
                    case NOP:
                        snprintf(info, sizeof(info), "NOP");
                        exec_nop(&pc);
                        break;
                    default:
                        snprintf(info, sizeof(info), "Instrucción desconocida");
                        running = 0;
                }
                printf("Después: PC=%d, ACC=%d, Z=%d\n", pc, acc, z);
                steps[step_count].pc = pc;
                steps[step_count].instr = instr;
                steps[step_count].arg = arg;
                steps[step_count].acc = acc;
                steps[step_count].z = z;
                strncpy(steps[step_count].info, info, sizeof(steps[step_count].info)-1);
                steps[step_count].info[sizeof(steps[step_count].info)-1] = '\0';
                step_count++;
                printf("Presiona Enter para continuar...\n");
                getchar();
            }
            printf("\nEjecución finalizada.\n");
            print_memory(memoria, -1);
            printf("Memoria[10] (resultado): %d\n", memoria[10]);
            // Resumen detallado
            printf("\n--- RESUMEN DE EJECUCIÓN ---\n");
            for (int i = 0; i < step_count; i++) {
                printf("Paso %2d | PC: %2d | ", i+1, steps[i].pc);
                if (steps[i].instr >= 0 && steps[i].instr <= 8) {
                    printf("%s", instr_names[steps[i].instr]);
                    if (instr_length[steps[i].instr] == 2)
                        printf(" %d", steps[i].arg);
                } else {
                    printf("???");
                }
                printf(" | ACC: %d | Z: %d | %s\n", steps[i].acc, steps[i].z, steps[i].info);
            }
            // Limpiar memoria automáticamente
            memset(memoria, 0, sizeof(memoria));
            memset(mem_labels, 0, sizeof(mem_labels));
            mem_ptr = 0;
            printf("\nMemoria limpiada automáticamente tras la ejecución.\n");
            print_memory(memoria, -1);
            continue;
        }
        // Cargar dato: formato "N: V" o "N: etiqueta"
        int pos, val;
        char label[MAX_LABEL_LEN];
        if (sscanf(input, "%d: %d", &pos, &val) == 2) {
            if (valid_addr(pos)) {
                memoria[pos] = val;
                printf("Dato %d guardado en memoria[%d]\n", val, pos);
            } else {
                printf("Dirección fuera de rango.\n");
            }
            print_memory(memoria, -1);
            continue;
        } else if (sscanf(input, "%d: %15s", &pos, label) == 2) {
            if (valid_addr(pos)) {
                set_mem_label(pos, label);
                printf("Etiqueta '%s' asignada a memoria[%d]\n", label, pos);
            } else {
                printf("Dirección fuera de rango para etiqueta.\n");
            }
            print_memory(memoria, -1);
            continue;
        }
        // Cargar instrucción: ej "LOAD 8" o "PRINT"
        char instr_str[16];
        int arg = -1;
        int n = sscanf(input, "%15s %d", instr_str, &arg);
        int instr_code = -1;
        for (int i = 0; i < 9; i++) {
            if (strcmp(instr_str, instr_names[i]) == 0) {
                instr_code = i;
                break;
            }
        }
        if (instr_code == -1) {
            printf("Instrucción o comando no reconocido.\n");
            continue;
        }
        // Guardar instrucción y argumento (si corresponde)
        if (instr_length[instr_code] == 2) {
            if (n < 2) {
                printf("Falta argumento para %s\n", instr_str);
                continue;
            }
            if (mem_ptr + 1 >= MEM_SIZE) {
                printf("Memoria llena.\n");
                continue;
            }
            memoria[mem_ptr++] = instr_code;
            memoria[mem_ptr++] = arg;
        } else {
            if (mem_ptr >= MEM_SIZE) {
                printf("Memoria llena.\n");
                continue;
            }
            memoria[mem_ptr++] = instr_code;
        }
        print_memory(memoria, -1);
    }
    printf("Saliendo del simulador.\n");
    return 0;
}

// Imprime el listado plano de instrucciones cargadas
void print_program(int *mem) {
    printf("\nPrograma cargado:\n");
    int i = 0;
    while (i < CODE_SIZE) {
        int instr = mem[i];
        if (instr >= 0 && instr <= 8) {
            printf("%2d: %s", i, instr_names[instr]);
            if (instr_length[instr] == 2 && i+1 < CODE_SIZE) {
                printf(" %d", mem[i+1]);
            }
            printf("\n");
            i += instr_length[instr];
        } else {
            printf("%2d: ???\n", i);
            i++;
        }
    }
}

// Imprime la memoria, diferenciando código y datos, y resaltando el PC, mostrando etiquetas
void print_memory(int *mem, int pc) {
    print_program(mem);
    printf("\nMemoria:\n");
    printf("  Código:\n");
    for (int i = 0; i < CODE_SIZE; i++) {
        if (i == pc) printf("-> "); else printf("   ");
        printf("[%2d] ", i);
        int is_instr = 0;
        for (int j = 0; j < 9; j++) if (mem[i] == j && (i % 2 == 0 || mem[i] == HALT || mem[i] == PRINT || mem[i] == NOP)) is_instr = 1;
        if (is_instr) printf("%s", instr_names[mem[i]]);
        else printf("arg: %d", mem[i]);
        if (strlen(mem_labels[i]) > 0) printf("   // %s", mem_labels[i]);
        printf("\n");
    }
    printf("  Datos:\n");
    for (int i = CODE_SIZE; i < MEM_SIZE; i++) {
        printf("   [%2d] DATO: %d", i, mem[i]);
        if (strlen(mem_labels[i]) > 0) printf("   // %s", mem_labels[i]);
        printf("\n");
    }
}

// Etiquetas para memoria
void set_mem_label(int addr, const char *label) {
    if (valid_addr(addr)) {
        strncpy(mem_labels[addr], label, MAX_LABEL_LEN-1);
        mem_labels[addr][MAX_LABEL_LEN-1] = '\0';
    }
}

// Validar dirección de memoria
int valid_addr(int addr) {
    return addr >= 0 && addr < MEM_SIZE;
}

// Instrucciones modulares con comentarios didácticos
void exec_load(int *mem, int *acc, int *pc, int *z) {
    // Carga el valor de memoria en la dirección 'addr' dentro del acumulador
    int addr = mem[*pc + 1];
    if (!valid_addr(addr)) { printf("LOAD: Dirección inválida %d\n", addr); exit(1); }
    *acc = mem[addr];
    *z = (*acc == 0);
    *pc += instr_length[LOAD];
}
void exec_add(int *mem, int *acc, int *pc, int *z) {
    // Suma el valor de memoria en la dirección 'addr' al acumulador
    int addr = mem[*pc + 1];
    if (!valid_addr(addr)) { printf("ADD: Dirección inválida %d\n", addr); exit(1); }
    *acc += mem[addr];
    *z = (*acc == 0);
    *pc += instr_length[ADD];
}
void exec_sub(int *mem, int *acc, int *pc, int *z) {
    // Resta el valor de memoria en la dirección 'addr' al acumulador
    int addr = mem[*pc + 1];
    if (!valid_addr(addr)) { printf("SUB: Dirección inválida %d\n", addr); exit(1); }
    *acc -= mem[addr];
    *z = (*acc == 0);
    *pc += instr_length[SUB];
}
void exec_store(int *mem, int *acc, int *pc, int *z) {
    // Guarda el valor del acumulador en la dirección 'addr' de memoria
    int addr = mem[*pc + 1];
    if (!valid_addr(addr)) { printf("STORE: Dirección inválida %d\n", addr); exit(1); }
    mem[addr] = *acc;
    *pc += instr_length[STORE];
}
void exec_print(int *mem, int *acc, int *pc, int *z) {
    // Imprime el valor actual del acumulador
    printf("PRINT: ACC = %d\n", *acc);
    *pc += instr_length[PRINT];
}
void exec_halt(int *running) {
    // Detiene la ejecución del programa
    printf("HALT: Parando ejecución.\n");
    *running = 0;
}
void exec_jmp(int *mem, int *pc) {
    // Salta a la dirección especificada
    int addr = mem[*pc + 1];
    if (!valid_addr(addr)) { printf("JMP: Dirección inválida %d\n", addr); exit(1); }
    *pc = addr;
}
void exec_jz(int *mem, int *pc, int z) {
    // Salta a la dirección especificada si el flag Z está activo (ACC == 0)
    int addr = mem[*pc + 1];
    if (!valid_addr(addr)) { printf("JZ: Dirección inválida %d\n", addr); exit(1); }
    if (z) *pc = addr;
    else *pc += instr_length[JZ];
}
void exec_nop(int *pc) {
    // No hace nada, solo avanza el PC
    printf("NOP: No hace nada.\n");
    *pc += instr_length[NOP];
}
// Ejemplo: suma 2 + 2 y guarda el resultado en memoria[10]
void cargar_ejemplo_suma(int *mem, int *mem_ptr) {
    int prog[] = {LOAD, 8, ADD, 9, STORE, 10, PRINT, HALT, 2, 2};
    int n = sizeof(prog)/sizeof(int);
    for (int i = 0; i < n && i < MEM_SIZE; i++) mem[i] = prog[i];
    *mem_ptr = n;
}
// Ejemplo: resta 10 - 5
void cargar_ejemplo_resta(int *mem, int *mem_ptr) {
    int prog[] = {LOAD, 8, SUB, 9, PRINT, HALT, 0, 0, 10, 5};
    int n = sizeof(prog)/sizeof(int);
    for (int i = 0; i < n && i < MEM_SIZE; i++) mem[i] = prog[i];
    *mem_ptr = n;
}
// Ejemplo: bucle cuenta atrás desde 3
void cargar_ejemplo_bucle(int *mem, int *mem_ptr) {
    int prog[] = {LOAD, 8, PRINT, SUB, 9, JZ, 7, JMP, 2, HALT, 0, 0, 3, 1};
    int n = sizeof(prog)/sizeof(int);
    for (int i = 0; i < n && i < MEM_SIZE; i++) mem[i] = prog[i];
    *mem_ptr = n;
}