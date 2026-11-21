# RusTask

CLI para gestionar tareas escrito en Rust con clap v4.

## Comandos disponibles

### 1. Crear tarea (`create`)
```bash
cargo run -- create "<NOMBRE>" <ESTADO>
```
Estados válidos: `todo`, `doing`, `done`.

### 2. Mostrar tareas (`show`)
```bash
# Mostrar todo
cargo run -- show

# Mostrar con fechas y detalles
cargo run -- show -c

# Mostrar por ID específico
cargo run -- show <ID>

# Filtrar por estado
cargo run -- show todo
cargo run -- show doing
cargo run -- show done

# Resumen de totales
cargo run -- show summary
```

### 3. Editar tarea (`edit`)
```bash
# Cambiar nombre
cargo run -- edit name <ID> "<NUEVO_NOMBRE>"

# Cambiar estado
cargo run -- edit status <ID> <ESTADO>
```

### 4. Eliminar tarea (`delete`)
```bash
cargo run -- delete <ID>
```

## Validación
- Nombres no vacíos ni con solo espacios en blanco.
- Estados restringidos estrictamente al enum `TaskStatus`.
