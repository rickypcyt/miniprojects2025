#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define TAPE_SIZE 1024
#define BLANK '_'
#define MAX_STATES 32
#define MAX_SYMBOLS 16
#define MAX_TRANSITIONS 128

// Dirección del movimiento del cabezal
typedef enum { LEFT, RIGHT, STAY } Direction;

// Estructura de una transición
typedef struct {
    int current_state;
    char read_symbol;
    int next_state;
    char write_symbol;
    Direction move;
} Transition;

// Estructura de la cinta
typedef struct {
    char cells[TAPE_SIZE];
    int head;
} Tape;

// Máquina de Turing
typedef struct {
    int num_states;
    int start_state;
    int accept_state;
    int reject_state;
    int current_state;
    Transition transitions[MAX_TRANSITIONS];
    int num_transitions;
    Tape tape;
} TuringMachine;

// Inicializa la cinta con una cadena
void tape_init(Tape *tape, const char *input) {
    memset(tape->cells, BLANK, TAPE_SIZE);
    int len = strlen(input);
    for (int i = 0; i < len && i < TAPE_SIZE; i++) {
        tape->cells[i] = input[i];
    }
    tape->head = 0;
}

// Imprime la cinta y la posición del cabezal, resaltando la celda actual
void tape_print_visual(const Tape *tape, int window) {
    int start = tape->head - window;
    int end = tape->head + window;
    if (start < 0) start = 0;
    if (end >= TAPE_SIZE) end = TAPE_SIZE - 1;
    for (int i = start; i <= end; i++) {
        if (i == tape->head) printf("[%c]", tape->cells[i]);
        else printf(" %c ", tape->cells[i]);
    }
    printf("\n");
}

// Busca una transición válida
Transition* find_transition(TuringMachine *tm, int state, char symbol) {
    for (int i = 0; i < tm->num_transitions; i++) {
        if (tm->transitions[i].current_state == state && tm->transitions[i].read_symbol == symbol) {
            return &tm->transitions[i];
        }
    }
    return NULL;
}

// Ejecuta un paso de la máquina
int turing_step(TuringMachine *tm) {
    char current_symbol = tm->tape.cells[tm->tape.head];
    Transition *tr = find_transition(tm, tm->current_state, current_symbol);
    if (!tr) return 0; // No hay transición válida
    tm->tape.cells[tm->tape.head] = tr->write_symbol;
    tm->current_state = tr->next_state;
    if (tr->move == LEFT && tm->tape.head > 0) tm->tape.head--;
    else if (tr->move == RIGHT && tm->tape.head < TAPE_SIZE - 1) tm->tape.head++;
    // Si STAY, no mover
    return 1;
}

// Imprime la tabla de reglas (transiciones) de la máquina de Turing
void print_rules(const TuringMachine *tm) {
    printf("Reglas de transición de la máquina de Turing:\n");
    printf("| Estado actual | Símbolo leído | Estado siguiente | Símbolo escrito | Movimiento   |\n");
    printf("|:-------------:|:------------:|:---------------:|:--------------:|:------------:|\n");
    for (int i = 0; i < tm->num_transitions; i++) {
        const char *mov = tm->transitions[i].move == LEFT ? "Izquierda" :
                          (tm->transitions[i].move == RIGHT ? "Derecha" : "Quedarse");
        printf("|      %2d       |      %c       |      %2d        |      %c        | %10s   |\n",
            tm->transitions[i].current_state,
            tm->transitions[i].read_symbol == BLANK ? '_' : tm->transitions[i].read_symbol,
            tm->transitions[i].next_state,
            tm->transitions[i].write_symbol == BLANK ? '_' : tm->transitions[i].write_symbol,
            mov);
    }
    printf("\n");
}

// Ejecuta la máquina hasta aceptar, rechazar o no encontrar transición, mostrando detalles visuales
void turing_run_visual(TuringMachine *tm, int max_steps) {
    int steps = 0;
    printf("\n--- Simulación de Máquina de Turing: Sumar 1 a un número binario ---\n");
    printf("La cinta contiene un número binario. El objetivo es sumarle 1.\n");
    printf("Cada paso muestra el estado, el símbolo bajo el cabezal, la transición y la cinta.\n\n");
    while (steps < max_steps) {
        printf("Paso %d:\n", steps+1);
        printf("  Estado actual: %d\n", tm->current_state);
        char current_symbol = tm->tape.cells[tm->tape.head];
        printf("  Símbolo bajo el cabezal: '%c'\n", current_symbol);
        tape_print_visual(&tm->tape, 8);
        if (tm->current_state == tm->accept_state) {
            printf("\n¡Aceptado! La suma ha terminado.\n");
            return;
        }
        if (tm->current_state == tm->reject_state) {
            printf("\nRechazado.\n");
            return;
        }
        Transition *tr = find_transition(tm, tm->current_state, current_symbol);
        if (!tr) {
            printf("\nNo hay transición válida.\n");
            return;
        }
        printf("  Transición: (q%d, '%c') -> (q%d, '%c', %s)\n",
            tr->current_state, tr->read_symbol, tr->next_state, tr->write_symbol,
            tr->move == LEFT ? "Izquierda" : (tr->move == RIGHT ? "Derecha" : "Quedarse"));
        // Aplicar transición
        tm->tape.cells[tm->tape.head] = tr->write_symbol;
        tm->current_state = tr->next_state;
        if (tr->move == LEFT && tm->tape.head > 0) tm->tape.head--;
        else if (tr->move == RIGHT && tm->tape.head < TAPE_SIZE - 1) tm->tape.head++;
        printf("\n");
        steps++;
    }
    printf("\nDemasiados pasos.\n");
}

// Ejemplo: suma 1 a un número binario en la cinta
int main() {
    // Definir transiciones para sumar 1 a un número binario (de derecha a izquierda)
    // Estados: 0 = buscar el final, 1 = sumar, 2 = aceptar
    TuringMachine tm = {0};
    tm.num_states = 3;
    tm.start_state = 0;
    tm.accept_state = 2;
    tm.reject_state = -1; // No usamos rechazo aquí
    tm.current_state = tm.start_state;
    tm.num_transitions = 0;

    // Estado 0: mover a la derecha hasta encontrar BLANK
    tm.transitions[tm.num_transitions++] = (Transition){0, '0', 0, '0', RIGHT};
    tm.transitions[tm.num_transitions++] = (Transition){0, '1', 0, '1', RIGHT};
    tm.transitions[tm.num_transitions++] = (Transition){0, BLANK, 1, BLANK, LEFT};

    // Estado 1: sumar 1 (si 0 -> 1 y aceptar, si 1 -> 0 y seguir a la izquierda, si BLANK -> 1 y aceptar)
    tm.transitions[tm.num_transitions++] = (Transition){1, '0', 2, '1', STAY};
    tm.transitions[tm.num_transitions++] = (Transition){1, '1', 1, '0', LEFT};
    tm.transitions[tm.num_transitions++] = (Transition){1, BLANK, 2, '1', STAY};

    // Inicializar la cinta con un número binario
    const char *input = "1011"; // 11 en decimal, resultado esperado: 1100 (12)
    tape_init(&tm.tape, input);

    printf("Cinta inicial: %s\n\n", input);
    print_rules(&tm);
    turing_run_visual(&tm, 50);
    printf("\nCinta final: ");
    tape_print_visual(&tm.tape, 8);
    printf("\nResumen: La máquina de Turing ha sumado 1 al número binario de la cinta.\n");
    return 0;
}
