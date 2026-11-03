# Investigación: Desincronización y Color Verde en el Sidebar Derecho (Right Dock)

**Fecha:** 2026-11-03 14:30  
**Objetivo:** Investigar por qué el sidebar derecho no respeta la apariencia y sincronización con el sidebar izquierdo, y por qué se visualiza de color verde en lugar del tema correspondiente.  
**Modo:** Solo lectura (investigación técnica sin mutación de código de producción).

---

## 1. Resumen Ejecutivo (Root Causes)

Se identificaron dos causas principales que originan la discrepancia visual:

1. **Discrepancia en la propiedad de fondo del tema (`panel_bg` vs `sidebar_bg`):**
   - El **sidebar izquierdo** (`render_sidebar_background` en `src/client/shell/render.rs:21`) se dibuja usando `palette.sidebar_bg`.
   - El **sidebar derecho** (`render_right_dock` en `src/client/shell/dock.rs:672`) se dibuja usando `config.palette.panel_bg`.
   - En la configuración activa del usuario (`~/.config/herdr/config.toml`), el tema personalizado define:
     ```toml
     panel_bg = "#12272b" # Verde petróleo / teal oscuro
     ```
     mientras que `sidebar_bg` no está definido en `[theme.custom]`, por lo que hereda el valor por defecto de Catppuccin: `Color::Reset` (fondo transparente o negro nativo del terminal).
   - **Resultado:** El sidebar izquierdo es negro/transparente (`sidebar_bg = Reset`), mientras que el sidebar derecho se pinta con un fondo verde petróleo sólido (`#12272b`), haciendo que todo el panel derecho resalte en verde.

2. **Uso de `config.palette.green` en elementos del cuerpo:**
   - En `dock.rs`, varias líneas usan directamente `config.palette.green` (`#00f5d4` en la paleta del usuario, verde/menta brillante):
     - Línea 1227: Métrica de tokens/contexto (`Context: ...`).
     - Línea 1370: Estado de Centinela (`🛡 [CENTINELA] - ● En guardia`).
     - Línea 1570: Estado limpio de Git (`git: limpio`).
     - Línea 1927: Pestaña Changes (`Working tree clean`).

3. **Desincronización de Lenguaje Visual y Estructura:**
   - **Sidebar izquierdo:** Diseño plano, sobrio y minimalista. Cabeceras discretas (`" spaces"` y `" agents"` en `overlay0` bold), sin bordes perimetrales ni cajas cerradas, selección con `active_row_bg` y `selection_bg`.
   - **Sidebar derecho:** Cajas enmarcadas con bordes redondeados (`╭`, `╮`, `╰`, `╯`) en `surface_dim`, banner superior centrado (`:: TELAR · GENTLE-AI ::`), pestañas tipo botón (`Status`, `Files`, `Changes`) con fondos `surface0` y `accent`, y múltiples colores fuertes (`peach`, `yellow`, `teal`, `mauve`, `green`, `blue`).

---

## 2. Comparativa Detallada: Sidebar Izquierdo vs Sidebar Derecho

| Elemento | Sidebar Izquierdo (`sidebar.rs`, `agent_sidebar.rs`) | Sidebar Derecho (`dock.rs`) |
| :--- | :--- | :--- |
| **Color de Fondo** | `palette.sidebar_bg` (`Color::Reset` en temas base) | `palette.panel_bg` (`#12272b` verde petróleo) |
| **Separador de borde** | Sin borde vertical propio; delimitado por el área de división del TUI. | Dibuja barra separadora vertical `│` en columna 0 con `palette.surface_dim`. |
| **Cabeceras de sección** | Texto plano: `" spaces"`, `" agents"` con estilo `overlay0` bold. | Cajas cerradas con marcos ASCII (`╭─ Titulo ─[-]─╮`). |
| **Filas y Contenido** | Filas compactas estilo árbol con indentación limpia. | Claves/Valores en cajas agrupadas (`render_card_kv`). |
| **Fondo de Selección** | `active_row_bg` / `selection_bg` / `surface1`. | `surface0` en archivos/filas seleccionadas. |
| **Jerarquía de Color** | Sutil (`overlay0`, `text`, `active_row_bg`). | Múltiples colores simultáneos (`peach`, `yellow`, `teal`, `mauve`, `green`, `blue`). |

---

## 3. Puntos Exactos en el Código

### A. Fondo del Dock Derecho
- **Archivo:** `src/client/shell/dock.rs:672`
  ```rust
  // Fondo del dock
  buffer.set_style(area, Style::default().bg(config.palette.panel_bg));
  ```
  *Corrección esperada:* Debe alinearse a `config.palette.sidebar_bg` al igual que `render_sidebar_background` en `src/client/shell/render.rs:21`:
  ```rust
  buffer.set_style(area, Style::default().bg(config.palette.sidebar_bg));
  ```

### B. Header de Pestañas y Chips Inactivos
- **Archivo:** `src/client/shell/dock.rs:727-732`
  ```rust
  let active_style = Style::default()
      .fg(config.palette.panel_bg)
      .bg(config.palette.accent);
  let inactive_style = Style::default()
      .fg(config.palette.subtext0)
      .bg(config.palette.surface0);
  ```
  Si el dock usa `sidebar_bg`, el fondo activo debe contrastar con `sidebar_bg` y el inactivo no debe generar bloques brillantes discordantes.

### C. Bordes de Tarjetas y Colores Internos
- **Archivo:** `src/client/shell/dock.rs:1073-1080`
  ```rust
  let card_border_style = Style::default().fg(config.palette.surface_dim);
  let card_title_style = Style::default().fg(config.palette.peach);
  let card_badge_style = Style::default().fg(config.palette.subtext0);
  let key_style = Style::default().fg(config.palette.yellow);
  let val_cyan = Style::default().fg(config.palette.teal);
  ```
- **Líneas con `palette.green`:**
  - `src/client/shell/dock.rs:1227`: `Style::default().fg(config.palette.green)` en Tokens.
  - `src/client/shell/dock.rs:1370`: `Style::default().fg(config.palette.green)` en Centinela.
  - `src/client/shell/dock.rs:1570`: `Style::default().fg(config.palette.green)` en Git status.
  - `src/client/shell/dock.rs:1927`: `Style::default().fg(config.palette.green)` en Changes.

---

## 4. Opciones de Solución Recomendadas

1. **Sincronización Inmediata del Fondo (Minimal Fix):**
   - Cambiar `config.palette.panel_bg` por `config.palette.sidebar_bg` en `src/client/shell/dock.rs:672`.
   - Con este cambio, el sidebar derecho adopta inmediatamente el mismo color de fondo exacto que tiene el sidebar izquierdo en cualquier tema activo o personalizado.

2. **Homogeneización del Lenguaje Visual (Full Parity):**
   - En lugar de cajas enmarcadas pesadas con esquinas redondeadas (`╭`, `╰`), adoptar divisores y cabeceras de sección sutiles equivalentes a las del sidebar izquierdo (`spaces` / `agents`).
   - Usar `overlay0` y `text` para etiquetas y valores por defecto, reservando colores acentuados únicamente para indicadores de estado reales (p. ej. error en rojo, ejecutando en amarillo, éxito en verde).
