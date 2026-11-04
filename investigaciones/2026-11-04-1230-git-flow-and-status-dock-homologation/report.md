# Informe de Ingeniería: Homologación de Status y Timeline Reactivo de Git Flow

## 1. Resumen Ejecutivo
Se implementó con éxito la homologación del sidebar derecho (Right Dock) en Telar TUI bajo los requerimientos visuales de `6666.png`, `8888.png` y el canvas interactivo de Git Flow (`x3.jpg`, `q111.png`, `q12.png`).
Se eliminaron las simulaciones sintéticas o datos placeholders para asegurar que cada workspace/proyecto reporte exclusivamente sus eventos genuinos con aislamiento estricto en tiempo real.

## 2. Decisiones Arquitectónicas (Render Puro & Cero I/O en UI)
1. **Desacoplamiento Estricto**:
   - `src/client/shell/git_flow.rs`: Modela los 10 pasos canónicos del ciclo de vida de Git/GitHub mediante estructuras algebraicas inmutables (`GitFlowStep`, `GitFlowCanvasSnapshot`).
   - `src/client/shell/telemetry.rs`: Recolector asíncrono y directo sobre `.git/` de microsegundos (`read_git_head_info`, `read_git_stash_info`, `read_git_latest_tag`, `detect_in_progress_git_op`, `try_query_github_pr`).
   - `src/client/shell/dock.rs`: Renderizado puro en Ratatui (`render_git_flow_content` y `render_status_content`).

2. **Homologación de Tarjeta `Status`**:
   - Fila unificada `Project <path> ◇ <branch>` para ahorrar espacio vertical.
   - Modelo alineado y divisor horizontal interno tenue (`surface_dim`).
   - Sub-estados en línea de Engram (puerto + observaciones) y MCP (servidores separados por comas).
   - Tarjeta `Git` separada e independiente abajo gobernada por `right_dock_show_git`.

3. **Evolución de Pestañas**:
   - Reemplazo de la pestaña redundante `[± Changes]` por `[⎇ Flow]`.
   - Navegación ágil por teclado (`j`/`k`, `g` para scroll a TOP) y ratón.

## 3. Configuración y Control
- Clave de configuración: `right_dock_show_git_flow = true` bajo `[ui]`.
- Conmutador en tiempo real en Settings Overlay (`prefix + ,`).
