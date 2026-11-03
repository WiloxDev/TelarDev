# Fuentes Consultadas: Right Dock UX/UI Color & Design Research

**Fecha:** 2026-11-03 16:50

## 1. Repositorios y Herramientas TUI Analizadas
- **BTOP:** `https://github.com/aristocratos/btop` — Arquitectura de métricas, densidad de información y gradientes funcionales en terminal.
- **Lazygit:** `https://github.com/jesseduffield/lazygit` — Diseño de paneles TUI en Go, paletas de bajo cansancio visual y navegación por teclado.
- **Zellij:** `https://github.com/zellij-org/zellij` — Sistema de diseño TUI en Rust, micro-pills y layout modular.
- **K9s:** `https://github.com/derailed/k9s` — Patrones de color semánticos basados en severidad y estado operativo.
- **Lualine.nvim:** `https://github.com/nvim-lualine/lualine.nvim` — Principio de encapsulación por pastillas (pills) y transiciones de superficie.
- **Superfile:** `https://github.com/yorukot/superfile` — TUI de última generación con tarjetas redondeadas, espaciado moderno y paddings generosos.
- **Yazi:** `https://github.com/sxyazi/yazi` — Diseño asíncrono y ligero en Rust con adaptación a temas terminales de 24 bits.
- **Bottom:** `https://github.com/ClementTsang/bottom` — Implementación en Rust de widgets de rendimiento y barras de progreso proporcionales.
- **Gentle-Shell & Center-Control-Gentle:** `/home/wilsonlavio/projects/center-control-gentle/src/` — Componentes `CCGHeader.ts`, `sidebarGovernance.ts` y tokens de Gentleman Programming.
- **Linear Design System / Vercel CLI:** Principios de diseño "Quiet Luxury" y monocromo con acento intencional en CLI/TUI.

## 2. Archivos del Código Base de Telar
- `src/client/shell/dock.rs` — Implementación actual de `render_status_content`, `render_card_kv`, `draw_card_box` y las 8 tarjetas de telemetría.
- `src/app/state.rs` — Definición de los 19 tokens de `Palette` y paletas nativas (`Palette::gentleman`, `Palette::cyberdream`, `Palette::tokyo_night`, etc.).
- `src/client/shell/sidebar.rs` — Referencia de diseño y sobriedad del sidebar izquierdo.
