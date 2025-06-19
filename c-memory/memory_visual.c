#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define MEMORY_SIZE 16  // Reducimos el tamaño para que sea más fácil de ver

// Array que representa la memoria
char memory[MEMORY_SIZE];

// Función para limpiar la pantalla
void clear_screen() {
    printf("\033[H\033[J");
}

// Función para dibujar la memoria
void draw_memory() {
    printf("\nESTADO ACTUAL DE LA MEMORIA:\n");
    printf("------------------------\n");
    
    // Dibujar índices
    printf("Posición: ");
    for(int i = 0; i < MEMORY_SIZE; i++) {
        printf("%2d ", i);
    }
    printf("\n");
    
    // Dibujar valores
    printf("Memoria:  ");
    for(int i = 0; i < MEMORY_SIZE; i++) {
        if(memory[i] == 0) {
            printf(" . ");
        } else {
            printf(" %c ", memory[i]);
        }
    }
    printf("\n");
    printf("------------------------\n");
}

// Función para inicializar la memoria
void init_memory() {
    memset(memory, 0, MEMORY_SIZE);
}

// Función para asignar memoria en el stack
int allocate_stack(int size, const char* name) {
    printf("\n📥 ASIGNANDO EN STACK: %s (%d bytes)\n", name, size);
    printf("Buscando espacio libre desde el inicio...\n");
    
    for(int i = 0; i < MEMORY_SIZE - size; i++) {
        int can_allocate = 1;
        for(int j = 0; j < size; j++) {
            if(memory[i+j] != 0) {
                can_allocate = 0;
                break;
            }
        }
        if(can_allocate) {
            printf("✓ Espacio encontrado en posición %d\n", i);
            for(int j = 0; j < size; j++) {
                memory[i+j] = 'S';
            }
            return i;
        }
    }
    printf("✗ No hay espacio suficiente en el stack\n");
    return -1;
}

// Función para asignar memoria en el heap
int allocate_heap(int size, const char* name) {
    printf("\n📥 ASIGNANDO EN HEAP: %s (%d bytes)\n", name, size);
    printf("Buscando espacio libre desde el final...\n");
    
    for(int i = MEMORY_SIZE-1; i >= size-1; i--) {
        int can_allocate = 1;
        for(int j = 0; j < size; j++) {
            if(memory[i-j] != 0) {
                can_allocate = 0;
                break;
            }
        }
        if(can_allocate) {
            printf("✓ Espacio encontrado en posición %d\n", i-size+1);
            for(int j = 0; j < size; j++) {
                memory[i-j] = 'H';
            }
            return i-size+1;
        }
    }
    printf("✗ No hay espacio suficiente en el heap\n");
    return -1;
}

// Función para liberar memoria
void free_memory(int start, int size, const char* name) {
    printf("\n🗑️  LIBERANDO MEMORIA: %s (%d bytes)\n", name, size);
    printf("Liberando desde la posición %d\n", start);
    
    for(int i = 0; i < size; i++) {
        memory[start+i] = 0;
    }
}

// Función para simular el uso de la memoria
void simulate_memory_usage() {
    printf("\n🚀 INICIANDO SIMULACIÓN DE MEMORIA 🚀\n");
    printf("====================================\n");
    printf("Explicación:\n");
    printf("- '.' significa memoria libre\n");
    printf("- 'S' significa memoria en el stack\n");
    printf("- 'H' significa memoria en el heap\n");
    printf("====================================\n");
    
    sleep(2);
    
    // 1. Asignar variable en stack
    int stack_var1 = allocate_stack(2, "variable1");
    draw_memory();
    sleep(2);
    
    // 2. Asignar memoria en heap
    int heap_var1 = allocate_heap(3, "array1");
    draw_memory();
    sleep(2);
    
    // 3. Asignar otra variable en stack
    int stack_var2 = allocate_stack(1, "variable2");
    draw_memory();
    sleep(2);
    
    // 4. Liberar memoria del heap
    free_memory(heap_var1, 3, "array1");
    draw_memory();
    sleep(2);
    
    // 5. Asignar nueva memoria en heap
    int heap_var2 = allocate_heap(2, "array2");
    draw_memory();
    sleep(2);
}

int main() {
    init_memory();
    draw_memory();
    printf("\nMemoria inicializada y vacía\n");
    sleep(2);
    
    simulate_memory_usage();
    
    printf("\n✨ SIMULACIÓN COMPLETADA ✨\n");
    return 0;
} 