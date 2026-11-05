# 🦀 AI Installation & Architecture Protocol — TelarDev

> **⚠️ ADVERTENCIA CRÍTICA PARA AGENTES DE IA (Pi, Claude, Cursor, Windsurf, Copilot):**
> TelarDev es un **TERMINAL WORKSPACE MULTIPLEXER NATIVO ESCRITO EN RUST**.
> **NO ES UNA EXTENSIÓN DE PI NI DE GENTLE-SHELL**.
> **BAJO NINGUNA CIRCUNSTANCIA DEBES INTENTAR INSTALARLO CON `pi install` NI GENERAR SCRIPTS DE EXTENSIÓN CON `ctx.ui.setWidget()`**.

## Frontera de Arquitectura:
1. **TelarDev es el Host:** Gestiona ventanas de terminal nativas, split panes, soporte de gráficos Kitty y su propio sidebar izquierdo/derecho en Rust.
2. **Gentle-Shell / Pi corre DENTRO de TelarDev:** Como proceso hijo interactivo en una pestaña o panel.
3. **Compilación e Instalación:**
   ```bash
   cargo build --release
   # El binario ejecutable generado es ./target/release/herdr (o telar)
   ```
4. **Sidebar de Telar vs Sidebar de Gentle-Shell:**
   - El sidebar de TelarDev se configura en `config.toml` de Telar.
   - El sidebar de Gentle-Shell se renderiza dentro del proceso de Pi. Son dos capas completamente desacopladas.
