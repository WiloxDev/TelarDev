# Investigación Técnica: Identidad Visual y Paleta de Gentleman Programming & Gentle-Shell

**Fecha:** 2026-11-03 16:00  
**Objetivo:** Investigar y documentar la identidad visual, colores y tokens de diseño oficiales de **Gentleman Programming** (`gentlemanprogramming.com`), `gentle-shell` y `gentle-pi`, para diseñar la paleta canónica correspondiente a la estructura de 19 tokens de Telar (`Palette`).  
**Modo:** Solo lectura sobre código de producción; documentación en artefacto de investigación.

---

## 1. Resumen Ejecutivo

Gentleman Programming cuenta con una identidad visual distintiva, elegante y reconocible dentro de la comunidad de desarrolladores y en el ecosistema **Gentle-AI / Gentle-Shell**:

1. **La Estética Principal ("Gentleman Sexy" / "Gentle Official"):**
   - **Fondo:** Negro obsidian profundo con sutil tinte magenta/rubí oscuro (`#060407` / `#060308`, paneles en `#100A0F` / `#140a12`).
   - **Acento Primario (Signature):** Rosa neón / Hot Pink vibrante (`#F43888` / `#FF76C3` / `#FF4F9A`).
   - **Acento Secundario (Elegancia):** Oro champagne suave (`#E0C27A` / `#DFBD76`).
   - **Tipografía y Textos:** Blanco perla suave de alto confort (`#F6EFF3`), atenuados en malva/rosa apagado (`#A78E9B`, `#D7A0B8`).
   - **Bordes y Superficies:** Marcos oscuros en tono ciruela/borgoña profundo (`#563040`, `#2A1720`).

2. **La Estética TUI ("Gentle-Shell" / "Arch-Electric"):**
   - Utilizada en cabeceras de TUI (`CCGHeader.ts`), monitores de tokens y banners CLI.
   - Combina el **Cian Eléctrico (`#00E5FF`)**, el **Rosa Neón (`#FF2DF7`)** y toques de **Lima Eléctrico (`#B6FF3B`)** sobre cajas con esquinas redondeadas (`╭─`, `╰─`).

---

## 2. Evidencia Extraída de Fuentes Reales

### A. Sitio Web Oficial (`gentlemanprogramming.com`)
- Metadatos de tema: `theme-color: #1a1218`
- Fondo general: `#060308eb` y paneles secundarios en `#140a12`
- Texto principal: `#f6eff3`
- Acento / Enlaces / Botones: `#ff76c3` y `#d7a0b8`
- Elementos del stack: herencia visual compartida con `herdr` (`var(--herdr-green)` y `var(--st-base)`).

### B. Temas Oficiales en `gentle-pi` (`~/.pi/agent/git/github.com/Gentleman-Programming/gentle-pi/themes/`)
- **`Gentleman-Sexy.json`:**
  - `bg`: `#060407`, `bgPanel`: `#100A0F`, `border`: `#563040`, `borderSubtle`: `#2A1720`
  - `accent`: `#F43888`, `activePink`: `#FF4F9A`, `champagne`: `#E0C27A`
  - `violet`: `#C49BFF`, `powderBlue`: `#A9C7EE`, `warning`: `#F2B86D`, `error`: `#FF718F`
- **`Gentle.json`:**
  - `bg`: `#06080f`, `bgSubtle`: `#0d0f14`, `border`: `#313342`
  - `accent`: `#7FB4CA` (blue) / `#E0C15A` (gold), `green`: `#B7CC85`

### C. Componentes TUI de `center-control-gentle` & `gentle-shell`
- `CCGHeader.ts`:
  ```typescript
  const CYAN = "\x1b[1;38;2;0;229;255m";  // #00E5FF
  const PINK = "\x1b[1;38;2;255;45;247m"; // #FF2DF7
  const LIME = "\x1b[1;38;2;182;255;59m"; // #B6FF3B
  ```
- `sidebarGovernance.ts`:
  > *"Gentleman's accent is pink; card titles use a separate champagne role."*

---

## 3. Mapeo Canónico a la Paleta de Telar (`Palette` de 19 Tokens)

Para integrar nativamente el tema **`gentleman`** en Telar (respetando la estructura de `src/app/state.rs:Palette`), los 19 tokens se mapean con exactitud quirúrgica:

```rust
/// Gentleman Programming — Dark Obsidian & Hot Pink aesthetic.
pub fn gentleman() -> Self {
    Self {
        accent: Color::Rgb(244, 56, 136),      // #f43888 - Hot Pink Neón oficial
        panel_bg: Color::Rgb(16, 10, 15),      // #100a0f - Fondo panel / header / tab bar
        sidebar_bg: Color::Reset,              // Color::Reset - Preserva fondo terminal / obsidian
        active_row_bg: Color::Rgb(40, 18, 30), // #28121e - Fila activa / selección sutil
        selection_bg: Color::Rgb(65, 30, 48),  // #411e30 - Cursor en Navigate mode
        surface0: Color::Rgb(35, 18, 28),      // #23121c - Pestañas inactivas / cajas
        surface1: Color::Rgb(55, 28, 44),      // #371c2c - Hover / elementos interactivos
        surface_dim: Color::Rgb(42, 23, 32),   // #2a1720 - Separadores y bordes tenues
        overlay0: Color::Rgb(140, 115, 128),   // #8c7380 - Texto secundario / números apagados
        overlay1: Color::Rgb(190, 160, 175),   // #bea0af - Texto secundario brillante
        text: Color::Rgb(246, 239, 243),       // #f6eff3 - Texto principal blanco perlado
        subtext0: Color::Rgb(215, 160, 184),   // #d7a0b8 - Subtexto / etiquetas suaves
        mauve: Color::Rgb(224, 194, 122),      // #e0c27a - Champagne Gold (ramas / insignia)
        green: Color::Rgb(180, 231, 199),      // #b4e7c7 - Menta suave / éxito
        yellow: Color::Rgb(242, 184, 109),     // #f2b86d - Ámbar cálido / advertencia
        red: Color::Rgb(255, 113, 143),        // #ff718f - Rosa coral / error
        blue: Color::Rgb(169, 199, 238),       // #a9c7ee - Azul pastel / lectura
        teal: Color::Rgb(0, 229, 255),         // #00e5ff - Cyan Eléctrico Gentle-Shell
        peach: Color::Rgb(255, 177, 221),      // #ffb1dd - Rosa pastel suave
    }
}
```

### Variante Gentle-Shell / Electric (`gentle-electric`)
Para usuarios que prefieren la vibra terminal/cyberpunk de `gentle-shell`:
- **`accent`**: `#00e5ff` (Cyan eléctrico)
- **`mauve`**: `#ff2df7` (Pink neón)
- **`panel_bg`**: `#07131d` (Azul profundo void)
- **`green`**: `#b6ff3b` (Lima eléctrico)

---

## 4. Conclusiones y Próximos Pasos

1. **Autenticidad:** La paleta original de Gentleman Programming no es verde petróleo ni azul estándar; es **negro obsidian con acento hot pink (`#F43888`) y toques champagne gold (`#E0C27A`)**, con un contraste cuidado para que no canse la vista.
2. **Implementación Limpia:** Agregar el tema `gentleman` a Telar solo requiere registrar la función en `src/app/state.rs`, su nombre canónico en `src/config/theme.rs`, y su selector en `SettingsOverlay`.
