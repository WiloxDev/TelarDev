# Fuentes de Investigación: Desincronización y Color Verde en el Sidebar Derecho

- `/home/wilsonlavio/.config/herdr/config.toml` (Configuración de tema activo con `panel_bg = "#12272b"`)
- `src/client/shell/dock.rs` (Implementación del dock derecho: renderizado de fondo en línea 672, tarjetas y uso de colores)
- `src/client/shell/render.rs` (Función `render_sidebar_background` en línea 21 y pipeline de composición)
- `src/client/shell/sidebar.rs` (Renderizado del sidebar izquierdo y uso de `sidebar_bg`, `active_row_bg`, `selection_bg`)
- `src/client/shell/agent_sidebar.rs` (Renderizado de agentes y filas del sidebar izquierdo)
- `src/app/state.rs` (Definición de `struct Palette`, temas por defecto como Catppuccin, y valores para `panel_bg` vs `sidebar_bg`)
