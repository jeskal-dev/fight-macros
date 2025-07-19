# Fight Macros

Un sistema de gestión de macros de teclado basado en Rust diseñado específicamente para juegos de lucha, particularmente Super Smash Brothers Ultimate. [1](#1-0) La aplicación proporciona ejecución automatizada de secuencias complejas de teclas a través de perfiles configurables y disparadores, permitiendo a los jugadores realizar movimientos precisos de juegos de lucha con combinaciones de teclas simples. <cite/>

## Características

- **Gestión de Macros Basada en Perfiles**: Múltiples perfiles de configuración para diferentes personajes de juegos de lucha [2](#1-1)
- **Ejecución de Secuencias de Teclas en Tiempo Real**: Reproducción asíncrona de macros con control de tiempo preciso [3](#1-2)
- **Cambio Dinámico de Perfiles**: Cambia entre perfiles durante la ejecución usando teclas dedicadas [4](#1-3)
- **Movimientos Complejos de Juegos de Lucha**: Macros preconfigurados para movimientos como Shoryuken, tilts y ataques especiales [5](#1-4)

## Arquitectura

La aplicación sigue una arquitectura modular y basada en eventos construida sobre el ecosistema async/await de Rust usando tokio: [6](#1-5)

- **`config`**: Gestión y persistencia de configuración [7](#1-6)
- **`input`**: Captura y procesamiento de eventos de teclado [8](#1-7)
- **`engine`**: Ejecución de macros y secuenciación de acciones [9](#1-8)
- **`utils`**: Utilidades de soporte para mapeo de teclas y logging <cite/>

## Instalación

### Prerrequisitos

- Rust 1.70+ con Cargo
- Sistema operativo con soporte para simulación de eventos de teclado (Windows, macOS, Linux)

### Compilar desde el Código Fuente

```bash
git clone https://github.com/jeskal-dev/fight-macros.git
cd fight-macros
cargo build --release
```

## Configuración

La aplicación usa un archivo `config.json` para definir perfiles y macros: [10](#1-9)

### Estructura de Perfil

Cada perfil contiene:

- `name`: Identificador legible del perfil
- `switch_key`: Tecla para activar el perfil (F1, F2, etc.)
- `macros`: Array de definiciones de macros

### Definición de Macro

Cada macro incluye:

- `name`: Nombre descriptivo para el macro
- `trigger`: Combinación de teclas que activa el macro [11](#1-10)
- `sequence`: Array de acciones (keydown, keyup, delay) [12](#1-11)

### Ejemplo de Configuración

```json
{
  "profiles": {
    "General (Default)": {
      "name": "General (Default)",
      "switch_key": "F1",
      "macros": [
        {
          "name": "Left Tilt",
          "trigger": {
            "modifiers": [],
            "key": "Q"
          },
          "sequence": [
            { "type": "keydown", "key": "Z" },
            { "type": "delay", "ms": 5.0 },
            { "type": "keydown", "key": "J" },
            { "type": "keyup", "key": "J" },
            { "type": "keyup", "key": "Z" }
          ]
        }
      ]
    }
  },
  "active_profile": "General (Default)"
}
```

## Uso

1. **Iniciar la Aplicación**:

   ```bash
   cargo run
   ```

2. **Cambiar Perfiles**: Usa las teclas de cambio configuradas (F1, F2, etc.) para cambiar perfiles activos [4](#1-3)

3. **Ejecutar Macros**: Presiona las teclas disparadoras configuradas para ejecutar secuencias de macros [13](#1-12)

## Cómo Funciona

La aplicación opera a través de un pipeline basado en eventos: [14](#1-13)

1. **Captura de Entrada**: Monitorea eventos de teclado usando el crate `rdev`
2. **Procesamiento de Eventos**: Convierte entrada de teclado cruda en mensajes `HotkeyEvent` [15](#1-14)
3. **Búsqueda de Macros**: Busca en el perfil activo definiciones de macros coincidentes [16](#1-15)
4. **Ejecución en Cola**: Ejecuta asíncronamente secuencias de macros a través del motor [17](#1-16)
5. **Simulación de Teclas**: Envía eventos de teclado sintetizados al sistema operativo [18](#1-17)

## Dependencias

Crates clave de Rust utilizados:

- `rdev`: Captura y simulación de eventos de entrada multiplataforma
- `tokio`: Runtime asíncrono para ejecución concurrente de macros [19](#1-18)
- `serde`/`serde_json`: Serialización de configuración
- `anyhow`: Manejo unificado de errores [20](#1-19)

## Desarrollo

### Estructura del Proyecto

```
src/
├── main.rs           # Punto de entrada y bucle de eventos
├── config/           # Gestión de configuración
│   ├── mod.rs
│   ├── handler.rs    # Gestión de estado global de config
│   └── types.rs      # Estructuras de datos de configuración
├── input/            # Procesamiento de entrada
│   ├── mod.rs
│   ├── keyboard.rs   # Manejo de eventos de teclado
│   └── combo.rs      # Procesamiento de combinaciones de teclas
├── engine/           # Motor de ejecución de macros
│   ├── mod.rs
│   ├── queue.rs      # Cola asíncrona de macros
│   ├── executor.rs   # Ejecución de secuencias de acciones
│   └── mapper.rs     # Simulación de eventos del SO
└── utils/            # Utilidades
    ├── mod.rs
    ├── helpers.rs    # Funciones de mapeo de teclas
    └── logging.rs    # Configuración de logging
```

### Compilación

```bash
# Compilación de desarrollo
cargo build

# Compilación de release
cargo build --release

# Ejecutar con logging
RUST_LOG=info cargo run
```

## Licencia

Este proyecto fue creado por JeskallDev en el commit [66a27bb9](https://github.com/jeskal-dev/fight-macros/commit/66a27bb9) el 2025-07-19. <cite/>

## Notas

La aplicación está específicamente diseñada para juegos de lucha e incluye macros preconfigurados para movimientos comunes como Shoryuken, varios tilts y ataques especiales. [21](#1-20) El sistema usa una estrategia de "el último gana" donde las nuevas ejecuciones de macros automáticamente cancelan cualquier macro que esté ejecutándose actualmente para prevenir conflictos. [22](#1-21)

Wiki pages you might want to explore:

- [Getting Started (jeskal-dev/fight-macros)](/wiki/jeskal-dev/fight-macros#2)
- [Configuration System (jeskal-dev/fight-macros)](/wiki/jeskal-dev/fight-macros#4)
