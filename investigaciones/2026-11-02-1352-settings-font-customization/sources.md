# Sources

## Source Index

### S-001: ANSI/VT Terminal Control Escape Sequences Specification (OSC 50)
- Family: PRIMARY
- URL or locator: https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands
- Date/version: 2024 (xterm patch #395)
- Access method: Documentation specification review
- Used for: Inviabilidad técnica de conmutación dinámica de fuentes mediante secuencias de escape legacy (OSC 50) en emuladores modernos.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Confirma que OSC 50 (`OSC 50 ; ? Pt ST`) fue creado para X11 Font server y carece de soporte o se encuentra intencionalmente deshabilitado por motivos de seguridad en terminales modernos como Ghostty, Alacritty y Windows Terminal.

### S-002: Ratatui 0.30 & Crossterm 0.29 Architecture Documentation
- Family: PRIMARY
- URL or locator: https://docs.rs/ratatui/0.30.0/ratatui/style/struct.Modifier.html
- Date/version: 2024 / Ratatui v0.30.0, Crossterm v0.29
- Access method: Cargo codebase inspection and docs.rs API specification
- Used for: Limitaciones del buffer de celdas TUI y uso de modificadores `Modifier::DIM`, `Modifier::BOLD` e `Modifier::ITALIC` para simular pesos visuales.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Ratatui abstrae el grid de caracteres en celdas con estilo cromático y modificadores SGR. No existe API ni representación interna de familias tipográficas a nivel de celda en terminales de texto estándar.

### S-003: Ghostty Terminal Architecture & Escape Sequence Handling
- Family: PRIMARY
- URL or locator: https://ghostty.org/docs
- Date/version: 2024-2025
- Access method: Project documentation review & `crates/ghostty-vt` reference
- Used for: Análisis del rasterizador de glifos en Ghostty y soporte de recarga en caliente vía configuración.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Ghostty rasteriza fuentes en GPU mediante Metal (macOS) o Vulkan/OpenGL. No procesa OSC 50 para cambio dinámico de familia tipográfica, pero reacciona instantáneamente a cambios en `~/.config/ghostty/config`.

### S-004: Kitty Remote Control Protocol Specification
- Family: PRIMARY
- URL or locator: https://sw.kovidgoyal.net/kitty/remote-control/
- Date/version: 2024 / Kitty 0.36+
- Access method: Official documentation review
- Used for: Viabilidad de IPC selectivo con `kitty @ set-font`.
- Usefulness: SUPPORTING
- Confidence impact: HIGH
- Notes: Requiere que el usuario configure explícitamente `allow_remote_control yes` o use un socket Unix para aceptar comandos de control remoto; de lo contrario el comando es rechazado.

### S-005: WezTerm CLI Command Specification
- Family: PRIMARY
- URL or locator: https://wezfurlong.org/wezterm/cli/cli/index.html
- Date/version: 2024 / WezTerm 20240203
- Access method: Official documentation review
- Used for: Evaluación de capacidades de scripting e IPC de WezTerm desde una aplicación TUI hija.
- Usefulness: SUPPORTING
- Confidence impact: MEDIUM
- Notes: Permite comunicación por socket Unix (`wezterm cli`), pero alterar la fuente requiere reescribir variables de configuración Lua o recargar el archivo de configuración.

### S-006: Alacritty Configuration & Live Reload Specification
- Family: PRIMARY
- URL or locator: https://alacritty.org/config-alacritty.html
- Date/version: 2024 / Alacritty v0.13+
- Access method: Official documentation review
- Used for: Análisis de IPC pasivo vía file watcher en `alacritty.toml`.
- Usefulness: SUPPORTING
- Confidence impact: HIGH
- Notes: Alacritty carece intencionalmente de interfaz IPC o escape sequences de control de fuentes, pero su hilo de monitoreo de archivos (`notify`) aplica cambios de `font.normal.family` en menos de 50 milisegundos.

### S-007: Vercel Geist Mono Typeface Specification
- Family: PRIMARY
- URL or locator: https://vercel.com/font
- Date/version: 2023-2024 / v1.3.0
- Access method: Official foundry release notes & repository (github.com/vercel/geist-font)
- Used for: Catálogo curado: Geist Mono (#1), análisis de pesos finos (UltraLight/Light) y licencia SIL OFL.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Diseñada en colaboración con Basement Chronicle. Altamente recomendada para entornos minimalistas modernos. Requiere parche Nerd Fonts para símbolos de desarrollo no estándar.

### S-008: US Graphics Berkeley Mono Specification & Licensing
- Family: PRIMARY
- URL or locator: https://usgraphics.com/products/berkeley-mono
- Date/version: 2023-2024
- Access method: Commercial foundry specification review
- Used for: Catálogo curado: Berkeley Mono (#3), evaluación estética y licencias comerciales de desarrollador.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Fuente comercial de altísima precisión. No puede ser redistribuida gratuitamente dentro del instalador de Telar, pero debe ser soportada mediante configuración del sistema anfitrión.

### S-009: JetBrains Mono Open Source Typeface
- Family: PRIMARY
- URL or locator: https://www.jetbrains.com/lp/mono/
- Date/version: 2024 / v2.304
- Access method: Typeface design documentation & repository
- Used for: Catálogo curado: JetBrains Mono (#4), análisis de legibilidad en código y variantes Thin/ExtraLight.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Disponible bajo SIL OFL 1.1 y cuenta con variante oficial en el repositorio de Nerd Fonts.

### S-010: Iosevka Custom Modular Font Project
- Family: PRIMARY
- URL or locator: https://typeof.net/Iosevka/
- Date/version: 2024 / v31.0+
- Access method: Project documentation & repository (github.com/be5invis/Iosevka)
- Used for: Catálogo curado: Iosevka (#2), análisis de densidad condensada y pesos delgados (Thin 100).
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Ideal para ventanas TUI divididas (splits) debido a su caja ultra-estrecha de glifo que ahorra hasta un 30% de ancho horizontal frente a fuentes tradicionales.

### S-011: Microsoft Cascadia Code & Cascadia Mono Typography
- Family: PRIMARY
- URL or locator: https://github.com/microsoft/cascadia-code
- Date/version: 2024 / v2407.24
- Access method: Repository specification & releases
- Used for: Catálogo curado: Cascadia Code / Mono (#5), compatibilidad con Windows Terminal y variantes Light/SemiLight.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Diseñada por Aaron Bell (Saja Typeworks) para Microsoft. Incluye pesos finos (ExtraLight 200, Light 300) y soporte opcional de glifos Powerline.

### S-012: Commit Mono Typeface Specification
- Family: PRIMARY
- URL or locator: https://commitmono.com/
- Date/version: 2023-2024 / v1.143
- Access method: Typeface release page & source repository
- Used for: Catálogo curado: Commit Mono (#6), neutralidad tipográfica suiza sin ornamentos.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Desarrollada bajo licencia SIL OFL. Enfatiza la neutralidad geométrica y elimina cualquier ambigüedad visual en símbolos de código.

### S-013: GitHub Monaspace Typography Family
- Family: PRIMARY
- URL or locator: https://monaspace.githubnext.com/
- Date/version: 2023-2024 / v1.101
- Access method: Technical paper & repository (github.com/githubnext/monaspace)
- Used for: Catálogo curado: Monaspace Argon y Krypton (#7), análisis de Texture Healing.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Diseñada por Lettermatic para GitHub Next. El algoritmo de Texture Healing expande y contrae glifos adyacentes manteniendo la grilla fija monoespacio.

### S-014: Victor Mono Cursive & Monospace Font
- Family: PRIMARY
- URL or locator: https://rubjo.github.io/victor-mono/
- Date/version: 2023-2024 / v1.5.6
- Access method: Project documentation & repository
- Used for: Catálogo curado: Victor Mono (#8), evaluación de itálicas cursivas y trazos finos (Thin/ExtraLight).
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Provee itálicas caligráficas genuinas que se activan con secuencias SGR `Modifier::ITALIC` en TUI. Licencia MIT.

### S-015: IBM Plex Mono Design System
- Family: PRIMARY
- URL or locator: https://www.ibm.com/plex/
- Date/version: 2024 / v6.4.0
- Access method: IBM Design Language repository
- Used for: Catálogo curado: IBM Plex Mono (#9), estructura neo-grotesca industrial y pesos Thin/ExtraLight.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Licencia SIL OFL 1.1. Excelente consistencia métrica entre pesos regulares y delgados.

### S-016: Intel One Mono Accessibility Font
- Family: PRIMARY
- URL or locator: https://github.com/intel/intel-one-mono
- Date/version: 2023-2024 / v1.3.0
- Access method: Repository specification & releases
- Used for: Catálogo curado: Intel One Mono (#10), ergonomía visual y reducción de fatiga ocular.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Desarrollada por Frere-Jones Type para Intel Brand. Diseñada específicamente para maximizar la legibilidad en usuarios con baja agudeza visual.

### S-017: Fira Code Ligatures Monospace Font
- Family: PRIMARY
- URL or locator: https://github.com/tonsky/FiraCode
- Date/version: 2023-2024 / v6.2
- Access method: Project documentation & repository
- Used for: Catálogo curado: Fira Code (#11), ligaduras de programación y variante Light.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Una de las tipografías más adoptadas en la comunidad de desarrolladores. Licencia SIL OFL.

### S-018: Adobe Source Code Pro
- Family: PRIMARY
- URL or locator: https://github.com/adobe-fonts/source-code-pro
- Date/version: 2023-2024 / v2.042
- Access method: Foundry documentation & repository
- Used for: Catálogo curado: Source Code Pro (#13), legibilidad en tamaños reducidos y pesos ExtraLight.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Diseñada por Paul D. Hunt para Adobe Systems bajo SIL OFL 1.1. Excelente apertura de contraformas.

### S-019: Apple SF Mono Specification
- Family: PRIMARY
- URL or locator: https://developer.apple.com/fonts/
- Date/version: 2023-2024
- Access method: Apple Developer Typography guidelines
- Used for: Catálogo curado: SF Mono (#12), compatibilidad estética con ecosistemas macOS y Terminal.app.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Licencia propietaria reservada a desarrolladores del ecosistema Apple. No redistribuible de forma pública.

### S-020: Telar Codebase Settings Overlay & Client Shell
- Family: IMPLEMENTATION
- URL or locator: `src/client/shell/settings.rs` & `src/client/shell/settings_overlay.rs`
- Date/version: Git commit local / Herdr v0.9.5
- Access method: Local file inspection
- Used for: Verificación del diseño actual de overlay, secciones de configuración (`ClientSettingsSection`), interacción de teclado y modelo de buffer Ratatui.
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Confirma que la adición de una sección tipográfica encaja limpiamente en la arquitectura de overlay existente y que la integración debe respetar el ciclo de pintado y eventos de Ratatui/Crossterm.
