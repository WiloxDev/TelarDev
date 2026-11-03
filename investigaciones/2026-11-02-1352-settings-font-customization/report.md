# Personalización de Fuentes en Settings de Telar / Herdr: Viabilidad Técnica, Catálogo Curado y Hoja de Ruta TUI vs GUI

## Executive Summary

La solicitud de incorporar una opción de personalización tipográfica en el menú de **Settings** de Telar (`src/client/shell/settings.rs` y `src/client/shell/settings_overlay.rs`), ofreciendo al menos 12 fuentes modernas, minimalistas y delgadas (thin/fine), aborda un aspecto crucial de la experiencia de usuario y la estética de la herramienta.

El análisis técnico concluye que la viabilidad depende estrictamente de la capa arquitectónica:

1. **Arquitectura TUI Actual (Rust + Ratatui 0.30 + Crossterm 0.29 + Terminal Externo)**:
   - En una aplicación TUI de terminal estándar, el cliente no controla la rasterización de glifos ni el pipeline de fuentes (FreeType/HarfBuzz). El terminal anfitrión (Ghostty, Alacritty, Kitty, WezTerm, iTerm2, Windows Terminal) es el único propietario del renderizado de fuentes y archivos TTF/OTF [S-002, S-003].
   - Las secuencias de escape legacy como **OSC 50** (`\x1b]50;FontName\x07`), usadas históricamente por XTerm para conmutar fuentes en caliente, han sido deliberadamente deprecadas o ignoradas en emuladores modernos (Kitty, Alacritty, Windows Terminal) debido a fallos de seguridad y asincronía arquitectónica [S-001, S-006].
   - **Viabilidad en TUI**: No es viable cambiar arbitrariamente la tipografía del emulador mediante secuencias de escape ANSI universales. Sin embargo, sí es 100% viable implementar:
     - Una sección consultiva de **Tipografía** en Settings con preview visual de pesos simulados (`Modifier::DIM` para aspecto fino/thin, `Modifier::BOLD`, `Modifier::ITALIC`).
     - Un generador interactivo de snippets de configuración (`copy-to-clipboard` o exportación a `ghostty.config`, `alacritty.toml`, `kitty.conf`, `wezterm.lua`).
     - Soporte opcional de IPC selectivo para terminales con control remoto habilitado (e.g. `kitty @ set-font`, `wezterm cli`) [S-004, S-005].

2. **Arquitectura GUI Nativa / Runtime Autónomo (Roadmap Fase 2)**:
   - Si Telar evoluciona o empaqueta su propio frontend nativo de ventana (utilizando Winit/wgpu, GPUI, o un contenedor Webview/Tauri con xterm.js/Ghostty-wasm) [S-020], Telar asume el control directo del pipeline gráfico.
   - En este modelo, el cambio dinámico entre más de 12 fuentes vectoriales empaquetadas en binario (o descargadas bajo demanda) es **completamente viable en tiempo real** con latencia menor a 16ms y recarga fluida de atlas de texturas.

Se documenta a continuación el catálogo exhaustivo de 13 tipografías de élite y la hoja de ruta de implementación técnica en dos fases.

---

## Research Question

¿Qué tan viable es integrar un selector de cambio de fuente tipográfica con al menos 12 opciones modernas, minimalistas y de trazo fino en la ventana de Settings de Telar, qué limitaciones técnicas impone el modelo TUI/terminal actual y cuál es la estrategia arquitectónica óptima para implementarlo a corto plazo (TUI) y a largo plazo (GUI)?

---

## Recommendation or Answer

Se recomienda adoptar una **estrategia bifurcada en dos fases**:

### 1. Recomendación Inmediata (Fase 1 - Cliente TUI Rust):
- **Nueva Sección en Settings**: Añadir `ClientSettingsSection::Typography` a `ClientSettingsSection::ALL` en `src/client/shell/settings.rs` [S-020].
- **Catálogo Curado con Muestra y Presets**:
  - Presentar la lista interactiva de las 13 fuentes recomendadas (Geist Mono, Iosevka, Berkeley Mono, JetBrains Mono, Cascadia Code, Commit Mono, Fira Code, SF Mono, IBM Plex Mono, Victor Mono, Monaspace Argon/Krypton, Source Code Pro, Intel One Mono).
  - Incluir metadata visible: Diseñador, Licencia, Características (Light/Thin, Cursiva, Ligaduras) y Estado de Instalación sugerido.
  - Proveer un simulador de densidad/peso tipográfico en la TUI:
    - *Modo Thin/Light*: Renderizado con `Modifier::DIM` y contraste calibrado sobre el fondo, atenuando la luminancia para dar la percepción visual de trazo ultra-fino en pantallas Retina/HiDPI.
    - *Modo Regular*: Estilo neutral estándar.
    - *Modo High-Contrast*: Modificadores nítidos para legibilidad.
- **Generador de Configuración para Emulador Activo**:
  - Detección de terminal anfitrión vía variables de entorno (`$TERM_PROGRAM`, `$GHOSTTY_RESOURCES_DIR`, `$KITTY_PID`, `$ALACRITTY_LOG`, etc.).
  - Botón de acción rápida: *"Copiar configuración para <Mi Terminal>"* que coloca la directiva exacta (`font-family = "..."`) en el portapapeles del sistema.
  - Para terminales compatibles con IPC (`kitty`, `wezterm`), botón opcional *"Aplicar en caliente"* si el socket o CLI está disponible.

### 2. Recomendación Futura (Fase 2 - GUI Window / Standalone Container):
- Incorporar abstracción de renderizado de texto mediante `cosmic-text` o `glyphon` + `wgpu` (o `font-family` dinámico en Webview).
- Empaquetar variantes WOFF2/TTF de peso fino (ExtraLight 200, Light 300) de fuentes permisivas (SIL OFL) para garantizar disponibilidad de fábrica sin depender de si el usuario las instaló en el sistema operativo.

---

## Key Findings

### 1. Desacoplamiento entre TUI y Rasterizador de Glifos
En la arquitectura de sistemas UNIX y Windows modernos, las aplicaciones TUI corren desacopladas del motor de rendering gráfico:
- La aplicación (Telar) escribe secuencias de bytes formateadas según estándares ANSI/VT100 sobre un pseudoterminal (PTY).
- El emulador de terminal (Ghostty, Kitty, Alacritty, Windows Terminal) lee el stream del PTY, parsea las celdas, consulta su propia configuración local (`config.json`, `alacritty.toml`, etc.), carga la fuente tipográfica configurada en el sistema operativo mediante directivas del sistema (CoreText en macOS, Fontconfig/FreeType en Linux, DirectWrite en Windows) y rasteriza los glifos en la GPU.
- **Consecuencia**: Una aplicación TUI no tiene canal estándar bidireccional para ordenar al emulador: *"Cambia tu fuente de renderizado a Geist Mono 11pt Thin"*.

### 2. Estado de las Secuencias de Escape Dinámicas
| Secuencia | Propósito Histórico | Compatibilidad Actual | Diagnóstico de Seguridad / Estabilidad |
| :--- | :--- | :--- | :--- |
| **OSC 50** (`\x1b]50;...?`) | Conmutación de fuente en X11/XTerm | Inexistente en Ghostty, Alacritty y Windows Terminal; deprecada en xterm | Considerada vector de ataque o inestabilidad gráfica. Kitty emite advertencia o la ignora. **Inviable** [S-001, S-003]. |
| **SGR 1 / 2** | `Modifier::BOLD` / `Modifier::DIM` | Universal (Ratatui / Crossterm) | **100% Viable**. Permite simular trazo fino (*Light/Thin effect*) reduciendo la intensidad del haz de caracteres [S-002]. |
| **SGR 3** | `Modifier::ITALIC` | Universal en emuladores modernos | **100% Viable**. Vital para fuentes con cursivas itálicas estilizadas (Victor Mono, Monaspace Radon/Krypton). |
| **SGR 4 / 21** | Underline / Double Underline / Curly | Altamente compatible | Viable para detalles visuales de realce en la TUI. |

### 3. IPC y Mecanismos de Control Externo de Terminales
Para usuarios que exigen conmutación en caliente desde el interior de la TUI, varios emuladores soportan protocolos propietarios:
- **Ghostty**: Actualmente no expone IPC de conmutación de fuente en caliente por diseño de seguridad, pero recarga instantáneamente cambios en su archivo de configuración `~/.config/ghostty/config` mediante file watchers (`kqueue`/`inotify`) [S-003].
- **Kitty**: Permite `kitty @ set-font -f "Geist Mono"` siempre y cuando `allow_remote_control yes` o `listen_on` esté configurado en `kitty.conf` [S-004].
- **WezTerm**: Dispone de utilitario `wezterm cli set-tab-navigator` y recarga en caliente de `wezterm.lua` [S-005].
- **Alacritty**: Dispone de auto-recarga al modificar `alacritty.toml`. Escribir o hacer touch en la clave `font.normal.family` surte efecto en menos de 50ms [S-006].

---

## Catálogo Curado de 13 Fuentes Modernas, Minimalistas y Finas

A continuación se detalla la selección de 13 tipografías contemporáneas ideales para programación, interfaces TUI y agentes autónomos, con énfasis en estética minimalista, variantes delgadas (Light / Thin / ExtraLight) y legibilidad de celdas:

| # | Familia Tipográfica | Categoría y Estilo | Pesos Destacados | Soporte Ligaduras | Licencia | Características Distintivas y Afinidad Estética |
| :- | :--- | :--- | :--- | :---: | :--- | :--- |
| **1** | **Geist Mono** | Neo-grotesco minimalista, suizo | UltraLight, Light, Regular | No | SIL OFL 1.1 | Creada por Vercel y Basement Chronicle. Altura de x generosa, trazos geométricos sumamente limpios y aireados. Es la definición actual de minimalismo moderno [S-007]. |
| **2** | **Iosevka** | Condensada, modular, ultra-esbelta | Thin (100), ExtraLight (200), Light | Sí (configurable) | SIL OFL 1.1 | Máxima eficiencia de espacio horizontal en pantallas divididas (splits/paneles). Permite ver 120+ columnas con gran holgura visual [S-010]. |
| **3** | **Berkeley Mono** | Tipografía de ingeniería de precisión | Thin, Light, Regular | No | Comercial (US Graphics) | Considerada el estándar de oro en tipografía monospaced para sistemas de misión crítica. Claridad insuperable de glifos ambiguos (0/O, 1/l/I) [S-008]. |
| **4** | **JetBrains Mono** | Desarrollador contemporáneo | Thin, ExtraLight, Light | Sí | SIL OFL 1.1 | Ángulos tallados en 8 grados para acelerar el escaneo visual de código. Muy balanceada, excelente versión Nerd Font [S-009]. |
| **5** | **Cascadia Code / Mono** | Humanista tecnológico | ExtraLight, Light, SemiLight | Sí (Code) / No (Mono) | SIL OFL 1.1 | Diseñada por Microsoft para Windows Terminal y VS Code. Redondeos amables y balance impecable en paneles de densidad alta [S-011]. |
| **6** | **Commit Mono** | Minimalismo neutro anonimizado | Regular, Light | No | SIL OFL 1.1 | Tipografía neutral inspirada en el diseño suizo sin distracciones; líneas rectas y espaciado rigurosamente calibrado [S-012]. |
| **7** | **Monaspace Argon / Krypton** | Innovación super-textura (GitHub) | Light, Regular, Medium | Sí | SIL OFL 1.1 | Desarrollada por GitHub Next. Utiliza 'Texture Healing' para ensanchar caracteres naturalmente estrechos sin perder la grilla monoespaciada [S-013]. |
| **8** | **Victor Mono** | Semiótica moderna + Itálicas cursivas | Thin, ExtraLight, Light | Sí | MIT License | Famosa por combinar un cuerpo monospaced pulido con itálicas en estilo caligráfico manuscrito fluido, ideal para comentarios y decoradores [S-014]. |
| **9** | **IBM Plex Mono** | Neo-grotesco industrial | Thin, ExtraLight, Light | No | SIL OFL 1.1 | Expresa la herencia técnica de IBM en la era moderna. Estructura mecánica rígida pero profundamente refinada [S-015]. |
| **10** | **Intel One Mono** | Diseñada para baja fatiga visual | Light, Regular, Medium | No | SIL OFL 1.1 | Optimizada ergonómicamente por Frere-Jones Type e Intel para reducir el cansancio visual tras jornadas prolongadas de depuración [S-016]. |
| **11** | **Fira Code** | Estándar de ligaduras técnicas | Light, Regular, Retina | Sí | SIL OFL 1.1 | Pionera en ligaduras de operadores lógicos y relacionales (`!=`, `===`, `=>`, `<!--`). Trazos limpios inspirados en Fira Mono de Mozilla [S-017]. |
| **12** | **SF Mono** | Estilo visual macOS / San Francisco | Light, Regular, Medium | No | Propietaria (Apple Developer) | Diseñada para alinearse armónicamente con la interfaz de Xcode y macOS. Trazo firme, balance métrico insuperable en pantallas Retina [S-019]. |
| **13** | **Source Code Pro** | Clásico contemporáneo de Adobe | ExtraLight, Light, Regular | No | SIL OFL 1.1 | Gran apertura de ojos tipográficos (`c`, `e`, `s`), garantizando legibilidad en cuerpos tipográficos microscópicos (8-10 pt) [S-018]. |

---

## Evidence Review

### 1. Inspección del Código Base de Telar (`src/client/shell/`)
En `src/client/shell/settings.rs` y `src/client/shell/settings_overlay.rs` [S-020]:
- La enumeración actual de secciones en Settings es:
  ```rust
  pub(crate) enum ClientSettingsSection {
      Theme,
      Indicators,
      Sound,
      Toast,
      Integrations,
  }
  ```
- El renderizado se apoya íntegramente en buffers de Ratatui (`ratatui::buffer::Buffer`), asignando celdas con caracteres (`&str` o `char`) y modificadores `crossterm::style::Modifier` (e.g. `BOLD`, `DIM`).
- **Conclusión Técnica**: Ratatui no dispone de tipos de datos para familias tipográficas (e.g., no existe `Style::default().font_family("Geist")`). Esto confirma que cualquier lógica de fuentes en TUI debe limitarse a la configuración externa o a modificadores de estilo (Dim/Light, Bold, Italic).

### 2. Pruebas de Secuencias de Escape ANSI / VT
- Documentación de Ghostty y xterm: El control OSC 50 fue eliminado por razones de seguridad en casi todos los emuladores modernos creados después de 2015. Escribir `\x1b]50;#1\x07` a través de `crossterm` resulta en caracteres descartados silenciosamente o basura impresa en pantalla en Ghostty y Windows Terminal [S-001, S-003].
- Experiencia de IPC: Herramientas CLI como `kitty @` dependen de que el usuario haya iniciado el terminal con un socket Unix (`--listen-on`) o permiso explícito en `kitty.conf`. Asumir que está disponible por defecto generaría errores o rechazos de conexión en el 90% de los entornos [S-004].

---

## Trade-offs and Risks

| Estrategia | Ventajas | Desventajas / Riesgos | Nivel de Riesgo |
| :--- | :--- | :--- | :---: |
| **A. Modificadores de Peso TUI (Dim/Normal/Bold) + Advisory** | 100% portable, cero dependencias externas, no corrompe terminales, encaja perfecto en arquitectura actual. | No cambia el glifo real si el usuario no tiene la fuente configurada en su terminal. | **BAJO (Recomendado)** |
| **B. Modificación Automática de Archivos de Configuración del Host** | Cambia la fuente real en emuladores como Ghostty o Alacritty mediante hot-reload. | Invasivo; puede sobrescribir configuraciones manuales complejas del usuario o fallar por permisos; difícil de revertir limpiamente. | **MEDIO - ALTO** |
| **C. Invocación de Comandos IPC (`kitty @`, `wezterm cli`)** | Conmutación instantánea sin reiniciar terminal cuando está soportado. | Falla de forma silenciosa o ruidosa si el terminal no está configurado para permitir control remoto. No soportado en Ghostty/Alacritty. | **MEDIO** |
| **D. Forzar Secuencias OSC 50** | En teoría transparente vía PTY. | Obsoleto, roto en Ghostty/Windows Terminal, posible desincronización de terminal. | **CRÍTICO (Descartado)** |

---

## Alternatives Considered

1. **Alteración forzada de secuencias de escape legacy (OSC 50)**: Descartada de inmediato por incompatibilidad con el emulador objetivo (Ghostty / Alacritty / Windows Terminal).
2. **Auto-escritura silenciosa en `~/.config/ghostty/config` o `~/.config/alacritty/alacritty.toml`**: Descartada por violar el principio de no invasividad y seguridad del espacio del usuario. Solo debe realizarse si el usuario pulsa un botón explícito de "Guardar en configuración de terminal".
3. **Sección "Typography" Consultiva + Snippets + Simulación de Peso (Elegida)**: Ofrece valor inmediato sin comprometer la estabilidad del runtime TUI.
4. **GUI Nativo Autónomo (Elegida para Fase 2)**: La verdadera solución para control tipográfico 100% autónomo.

---

## Unknowns and Limits

- **Emulador anfitrión del usuario**: Telar corre en entornos diversos (WSL2 con Windows Terminal, macOS con Ghostty/iTerm2, Linux con WezTerm/Alacritty). No siempre es determinista identificar la ruta exacta del archivo de configuración del emulador anfitrión.
- **Fuentes locales instaladas**: El sistema operativo del usuario puede no tener instalada localmente la fuente elegida (por ejemplo, Geist Mono o Berkeley Mono), lo cual requiere instrucciones claras o links de descarga oficiales.
- **Glifos y Nerd Fonts**: Algunas fuentes (como Victor Mono o Geist Mono vanilla) no incluyen de fábrica los glifos de Seti UI / iconos de desarrollo que Telar utiliza en su panel lateral [S-020]. Se debe advertir al usuario utilizar versiones con parche *Nerd Fonts* (e.g. `GeistMono Nerd Font` o `JetBrainsMono Nerd Font`).

---

## Recommended Next Actions

### Paso 1: Extender el Modelo de Settings en `src/client/shell/settings.rs`
1. Añadir el discriminante tipográfico:
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   pub(crate) enum ClientSettingsSection {
       Theme,
       Typography, // <--- Nueva sección
       Indicators,
       Sound,
       Toast,
       Integrations,
   }
   ```
2. Registrar la estructura de fuentes curadas con metadata:
   ```rust
   pub(crate) struct FontCatalogItem {
       pub id: &'static str,
       pub name: &'static str,
       pub designer: &'static str,
       pub weights: &'static str,
       pub license: &'static str,
       pub nerd_font_name: &'static str,
       pub tag: &'static str,
   }
   ```

### Paso 2: Implementar la UI del Catálogo en `src/client/shell/settings_overlay.rs`
1. Renderizar en la columna de opciones del overlay la lista de las 13 fuentes con su etiqueta descriptiva (`[Fine]`, `[Geometric]`, `[Code]`).
2. En el panel de previsualización derecho del overlay, mostrar un recuadro de prueba con código de muestra (Rust/TypeScript) y glifos Seti UI.
3. Proporcionar un toggle para la simulación de grosor:
   - `Density / Weight: Thin (Simulated via SGR Dim)` vs `Density: Regular`.
4. Incluir un botón de atajo `[ c ] Copiar directiva para emulador`.

### Paso 3: Planificar la Arquitectura de Renderizado para el Cliente GUI (Fase 2)
1. Integrar el motor tipográfico (`cosmic-text` / `glyphon`) en el prototipo de ventana nativa.
2. Empaquetar los binarios `.woff2` de Geist Mono Light, Iosevka Light y JetBrains Mono ExtraLight directamente en los assets del binario.
