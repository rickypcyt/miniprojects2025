#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Variables globales (segmento de datos)
int global_var = 42;
static int static_var = 100;
const char* global_string = "Soy una cadena global";

// Función para imprimir una línea de separación
void print_separator(const char* title) {
    printf("\n");
    for(int i = 0; i < 50; i++) printf("=");
    printf("\n%s\n", title);
    for(int i = 0; i < 50; i++) printf("=");
    printf("\n");
}

// Función para imprimir información de memoria
void print_memory_info(const char* name, const void* address, size_t size) {
    printf("Nombre: %-20s | Dirección: %p | Tamaño: %zu bytes\n", 
           name, address, size);
}

// Función para demostrar el stack
void demonstrate_stack() {
    print_separator("DEMOSTRACIÓN DEL STACK");
    
    int local_var = 10;
    char local_char = 'A';
    double local_double = 3.14;
    
    printf("Variables locales en el stack:\n");
    print_memory_info("local_var", &local_var, sizeof(local_var));
    print_memory_info("local_char", &local_char, sizeof(local_char));
    print_memory_info("local_double", &local_double, sizeof(local_double));
    
    // Demostrar que las variables locales se crean en orden
    printf("\nOrden de creación en el stack:\n");
    printf("local_var:   %p\n", &local_var);
    printf("local_char:  %p\n", &local_char);
    printf("local_double: %p\n", &local_double);
}

// Función para demostrar el heap
void demonstrate_heap() {
    print_separator("DEMOSTRACIÓN DEL HEAP");
    
    // Reservar memoria en el heap
    int* heap_int = (int*)malloc(sizeof(int));
    char* heap_string = (char*)malloc(50);
    double* heap_array = (double*)malloc(5 * sizeof(double));
    
    if (heap_int && heap_string && heap_array) {
        *heap_int = 42;
        strcpy(heap_string, "Hola desde el heap!");
        for(int i = 0; i < 5; i++) {
            heap_array[i] = i * 1.1;
        }
        
        printf("Memoria reservada en el heap:\n");
        print_memory_info("heap_int", heap_int, sizeof(int));
        print_memory_info("heap_string", heap_string, 50);
        print_memory_info("heap_array", heap_array, 5 * sizeof(double));
        
        // Liberar memoria
        free(heap_int);
        free(heap_string);
        free(heap_array);
        printf("\nMemoria liberada correctamente\n");
    }
}

// Función para demostrar variables globales y estáticas
void demonstrate_global_static() {
    print_separator("DEMOSTRACIÓN DE VARIABLES GLOBALES Y ESTÁTICAS");
    
    printf("Variables en el segmento de datos:\n");
    print_memory_info("global_var", &global_var, sizeof(global_var));
    print_memory_info("static_var", &static_var, sizeof(static_var));
    print_memory_info("global_string", (const void*)global_string, strlen(global_string) + 1);
}

// Función para demostrar punteros y referencias
void demonstrate_pointers() {
    print_separator("DEMOSTRACIÓN DE PUNTEROS");
    
    int value = 100;
    int* ptr = &value;
    int** ptr_to_ptr = &ptr;
    
    printf("Valor original: %d\n", value);
    printf("Dirección de value: %p\n", (void*)&value);
    printf("Valor del puntero: %p\n", (void*)ptr);
    printf("Valor al que apunta el puntero: %d\n", *ptr);
    printf("Dirección del puntero: %p\n", (void*)&ptr);
    printf("Valor del puntero a puntero: %p\n", (void*)ptr_to_ptr);
    printf("Valor al que apunta el puntero a puntero: %d\n", **ptr_to_ptr);
}

// Función para demostrar memory leaks
void demonstrate_memory_leak() {
    print_separator("DEMOSTRACIÓN DE MEMORY LEAK");
    
    printf("⚠️  ADVERTENCIA: Esta función demuestra un memory leak\n");
    printf("No liberamos la memoria reservada con malloc\n\n");
    
    int* leaky_ptr = (int*)malloc(sizeof(int));
    *leaky_ptr = 42;
    
    printf("Memoria reservada pero no liberada:\n");
    print_memory_info("leaky_ptr", leaky_ptr, sizeof(int));
    printf("Valor en la memoria: %d\n", *leaky_ptr);
    
    // No liberamos la memoria intencionalmente para demostrar el leak
    // En un programa real, deberíamos usar: free(leaky_ptr);
}

int main() {
    printf("\n🚀 DEMOSTRACIÓN COMPLETA DE MEMORIA EN C 🚀\n");
    
    demonstrate_stack();
    demonstrate_heap();
    demonstrate_global_static();
    demonstrate_pointers();
    demonstrate_memory_leak();
    
    printf("\n✨ Fin de la demostración ✨\n");
    return 0;
} 