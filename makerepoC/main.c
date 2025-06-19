#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>

// Función para ejecutar un comando en la terminal
int execute_command(const char *command) {
    int result = system(command);
    if (result == -1) {
        perror("Error al ejecutar el comando");
        return 0;
    }
    return 1;
}

int main() {
    // Obtener el directorio actual
    char current_dir[1024];
    if (getcwd(current_dir, sizeof(current_dir)) == NULL) {
        perror("Error al obtener el directorio actual");
        return 1;
    }

    // Obtener el nombre de la carpeta actual (repositorio)
    char *repo_name = strrchr(current_dir, '/');
    if (repo_name == NULL) {
        fprintf(stderr, "No se pudo determinar el nombre del directorio\n");
        return 1;
    }
    repo_name++; // Avanzamos al siguiente carácter para omitir la "/"

    // Imprimir la carpeta actual (repositorio)
    printf("📂 Estás en el repositorio: %s\n", repo_name);

    // Verificar si el directorio ya es un repositorio Git
    if (access(".git", F_OK) != -1) {
        printf("🚨 Este directorio ya es un repositorio Git.\n");
    } else {
        // Si no es un repositorio Git, inicializamos uno
        printf("Inicializando repositorio Git...\n");
        if (!execute_command("git init")) {
            return 1;
        }
    }

    // Pedir el nombre del repositorio para configurar remotos
    char remote_url[256];
    printf("Ingresa la URL del repositorio remoto (por ejemplo, https://github.com/usuario/%s.git):\n", repo_name);
    if (fgets(remote_url, sizeof(remote_url), stdin) == NULL) {
        fprintf(stderr, "Error al leer la URL del repositorio remoto.\n");
        return 1;
    }

    // Eliminar salto de línea al final de la URL
    remote_url[strcspn(remote_url, "\n")] = 0;

    // Configurar el repositorio remoto
    char command[512];
    snprintf(command, sizeof(command), "git remote add origin %s", remote_url);
    if (!execute_command(command)) {
        return 1;
    }

    // Pedir un mensaje para el primer commit
    char commit_message[256];
    printf("Ingresa un mensaje para el primer commit:\n");
    if (fgets(commit_message, sizeof(commit_message), stdin) == NULL) {
        fprintf(stderr, "Error al leer el mensaje del commit.\n");
        return 1;
    }

    // Eliminar salto de línea al final del mensaje
    commit_message[strcspn(commit_message, "\n")] = 0;

    // Agregar todos los cambios, hacer commit y push
    if (!execute_command("git add .")) {
        return 1;
    }

    snprintf(command, sizeof(command), "git commit -m \"%s\"", commit_message);
    if (!execute_command(command)) {
        return 1;
    }

    if (!execute_command("git push -u origin master")) {
        return 1;
    }

    printf("🎉 Repositorio '%s' creado y enviado a GitHub con éxito.\n", repo_name);
    return 0;
}
