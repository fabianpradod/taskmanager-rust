# Task Manager en Rust
Este proyecto es una aplicación para administrar tareas, desarrollada como parte de la Entrega #4 del curso de Algoritmos y Estructura de datos. Implementado en Rust, el programa demuestra el uso de estructuras de datos nativas del lenguaje.

Funcionalidades
- Agregar tareas: Crea nuevas tareas con descripción, prioridad y etiquetas.
- Mostrar tareas: Lista todas las tareas ordenadas por prioridad.
- Ver próxima tarea: Muestra la tarea de mayor prioridad sin eliminarla.
- Completar tarea: Elimina la tarea de mayor prioridad.
- Búsqueda por etiquetas: Organiza tareas según sus etiquetas.

Estructuras de datos implementadas
- Vector: Almacenamiento principal de tareas
- BinaryHeap: Cola de prioridad para gestionar el orden de las tareas
- HashMap: Índice de etiquetas para búsquedas eficientes

Cómo usar
1. Clona este repositorio
2. Ejecuta el programa con:
cargo run
3. Navega por el menú interactivo:
- Opción 1: Agregar una nueva tarea
- Opción 2: Mostrar todas las tareas ordenadas por prioridad
- Opción 3: Ver la próxima tarea pendiente
- Opción 4: Completar la tarea de mayor prioridad
- Opción 5: Salir del programa

Requisitos
- Rust (edición 2024)
- Cargo (incluido con Rust)
