# Bluetooth Manager

Un gestor de Bluetooth en terminal usando Ratatui, similar a blueman-manager pero con interfaz de terminal interactiva.

## Características

- **Gestión de dispositivos Bluetooth**: Ver, conectar, emparejar y eliminar dispositivos
- **Escaneo de dispositivos cercanos**: Buscar dispositivos Bluetooth reales en el área
- **Interfaz de terminal interactiva**: Navegación intuitiva con teclado
- **Múltiples vistas**: Lista de dispositivos, detalles del dispositivo y configuración
- **Estado en tiempo real**: Mostrar estado de conexión, emparejamiento y señal
- **Diseño modular**: Código bien estructurado y reutilizable
- **Preparado para integración real**: Estructura lista para conectar con BlueZ

## Comportamiento

### Escaneo de Dispositivos
- Presiona `s` para iniciar el escaneo de dispositivos cercanos
- Solo muestra dispositivos Bluetooth reales que estén en el área
- No incluye dispositivos simulados o de ejemplo
- Muestra información real: dirección MAC, tipo, intensidad de señal

### Gestión de Dispositivos
- Conectar/desconectar dispositivos encontrados
- Emparejar dispositivos nuevos
- Eliminar dispositivos de la lista
- Ver detalles completos de cada dispositivo

## Vistas

### 1. Lista de Dispositivos (Vista Principal)
- Tabla con dispositivos Bluetooth reales encontrados
- Información: Nombre, dirección MAC, tipo, estado, intensidad de señal, conocido
- Mensaje claro cuando no hay dispositivos ("Press 's' to scan")
- Navegación con flechas o j/k
- Dispositivo seleccionado resaltado

### 2. Detalles del Dispositivo
- Información completa del dispositivo seleccionado
- Estado de conexión, emparejamiento y confianza
- Acciones disponibles: conectar, emparejar, eliminar

### 3. Configuración
- Estado del Bluetooth (activado/desactivado)
- Estadísticas: total de dispositivos, conectados
- Controles de escaneo

## Controles

### Vista de Lista de Dispositivos
- `↑/k` - Seleccionar dispositivo anterior
- `↓/j` - Seleccionar dispositivo siguiente
- `Enter` - Ver detalles del dispositivo seleccionado
- `s` - Iniciar/detener escaneo
- `t` - Activar/desactivar Bluetooth
- `?` - Ir a configuración
- `q` - Salir

### Vista de Detalles del Dispositivo
- `c` - Conectar/desconectar dispositivo
- `p` - Emparejar dispositivo
- `r` - Eliminar dispositivo
- `ESC` - Volver a la lista de dispositivos

### Vista de Configuración
- `t` - Activar/desactivar Bluetooth
- `s` - Iniciar/detener escaneo
- `ESC` - Volver a la lista de dispositivos

## Instalación y Ejecución

### Prerrequisitos

- Rust (versión 1.70 o superior)
- Cargo
- Sistema con soporte Bluetooth
- BlueZ (para integración real)

### Compilar y Ejecutar

```bash
# Compilar el proyecto
cargo build

# Ejecutar en modo debug
cargo run

# Ejecutar en modo release
cargo run --release
```

## Estructura del Proyecto

```
src/
├── main.rs      # Punto de entrada y manejo de eventos
├── app.rs       # Lógica de la aplicación y estado de Bluetooth
└── ui.rs        # Componentes de interfaz de usuario modulares
```

## Tecnologías Utilizadas

- **Ratatui**: Biblioteca para interfaces de terminal
- **Crossterm**: Manejo de eventos de terminal
- **Tokio**: Runtime asíncrono
- **zbus**: Para integración futura con D-Bus (Bluetooth real)

## Estado Actual

### Implementado
- ✅ Interfaz de usuario completa y funcional
- ✅ Navegación entre vistas
- ✅ Estructura de datos para dispositivos Bluetooth
- ✅ Manejo de eventos de teclado
- ✅ Diseño modular y reutilizable

### Pendiente (Integración Real)
- 🔄 Escaneo real de dispositivos Bluetooth via BlueZ
- 🔄 Conexión real con dispositivos
- 🔄 Emparejamiento real
- 🔄 Obtención de RSSI y información real
- 🔄 Persistencia de dispositivos conocidos

## Integración con BlueZ

La aplicación está estructurada para integrarse fácilmente con BlueZ:

### Funciones a Implementar
1. **`scan_for_devices()`** - Llamar a BlueZ D-Bus para iniciar discovery
2. **`connect_device()`** - Conectar usando BlueZ D-Bus methods
3. **`pair_device()`** - Emparejar usando BlueZ D-Bus methods
4. **`remove_device()`** - Eliminar dispositivo de BlueZ

### D-Bus Integration Points
- `org.bluez.Adapter1` - Para operaciones de adaptador
- `org.bluez.Device1` - Para operaciones de dispositivo
- `org.bluez.Adapter1.StartDiscovery()` - Para iniciar escaneo
- `org.bluez.Device1.Connect()` - Para conectar dispositivos

## Desarrollo

El proyecto está estructurado de manera modular siguiendo principios DRY:

- **app.rs**: Contiene la lógica de negocio y estado de dispositivos Bluetooth
- **ui.rs**: Contiene todos los componentes de interfaz reutilizables
- **main.rs**: Maneja la inicialización, bucle principal y enrutamiento de eventos

### Componentes UI Modulares
- `create_device_table_widget()` - Tabla de dispositivos con estado vacío
- `create_device_info_widget()` - Información detallada del dispositivo
- `create_device_actions_widget()` - Acciones disponibles
- `create_settings_widget()` - Panel de configuración
- `create_status_widget()` - Barra de estado

## Próximos Pasos

Para completar la integración real con Bluetooth:

1. **Implementar D-Bus communication** usando zbus
2. **Conectar con BlueZ** para operaciones reales
3. **Agregar event listeners** para DeviceAdded/DeviceRemoved
4. **Implementar RSSI monitoring** en tiempo real
5. **Agregar persistencia** de dispositivos conocidos
6. **Manejar errores** de conexión y emparejamiento

## Notas

Esta aplicación está diseñada para mostrar solo dispositivos Bluetooth reales que estén cerca. No incluye dispositivos simulados o de ejemplo. Para ver dispositivos, asegúrate de tener dispositivos Bluetooth activos cerca y presiona 's' para iniciar el escaneo. 