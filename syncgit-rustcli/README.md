# SyncGit CLI

Una herramienta de línea de comandos para sincronizar fácilmente repositorios Git.

## Características

- 🔄 Sincroniza automáticamente el repositorio actual con su remoto
- 📂 Explora y sincroniza sub-repositorios Git
- 🌐 Verifica la conexión a internet antes de intentar operaciones
- 💻 Interfaz intuitiva y amigable

## Instalación

### Desde el código fuente

1. Clona este repositorio:

   ```
   git clone https://github.com/tu-usuario/syncgit_cli.git
   cd syncgit_cli
   ```

2. Compila con Cargo:

   ```
   cargo build --release
   ```

3. El binario ejecutable estará en `target/release/syncgit_cli`

### Instalación global (opcional)

Para hacer que la herramienta esté disponible globalmente:

```
cargo install --path .
```

## Uso

Simplemente ejecuta el programa en cualquier carpeta que contenga un repositorio Git:

```
syncgit_cli
```

Sigue las opciones del menú interactivo para:

- Sincronizar el repositorio actual
- Ver y sincronizar sub-repositorios
- Salir del programa

## Requisitos

- Rust 2021 Edition o superior
- Conexión a internet para sincronizar con remotos
- Git instalado en el sistema

## Licencia

Este proyecto está bajo la Licencia MIT. Ver el archivo `LICENSE` para más detalles.
