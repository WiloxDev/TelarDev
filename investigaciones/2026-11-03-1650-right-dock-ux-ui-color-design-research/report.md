# Investigación UX/UI: Refinamiento de Color, Superficies y Jerarquía del Sidebar Derecho (Right Dock)

**Fecha:** 2026-11-03 16:50  
**Objetivo:** Diagnosticar las deficiencias de diseño UX/UI identificadas en la captura del Right Dock (`5.png`), analizar las mejores prácticas de 10 herramientas y repositorios TUI modernos líderes, y presentar 10 modelos de diseño y color arquitecturados con precisión sobre los 19 tokens de Telar (`Palette`).  
**Modo:** Solo lectura sobre código de producción; documentación en artefacto de investigación.

---

## 1. Diagnóstico UX/UI de la Captura Actual (`5.png`)

Al examinar detalladamente la captura `5.png`, se identifican cuatro problemas críticos de diseño y experiencia de usuario:

### A. El "Efecto Arcoíris" (Sobrecarga Cognitiva)
- **Cada tarjeta tiene un color de título diferente y no coordinado:**
  - *Estado*: `accent` (Hot Pink / Cyan)
  - *Contexto*: `accent` con texto en `green` y costo en `yellow`
  - *Proyecto*: `peach` (Naranja)
  - *POOL / Pi*: `yellow` (Ámbar) con icono de escudo en `blue`
  - *SDD*: `mauve` (Púrpura / Dorado)
  - *Cambios / Git*: `teal` con rama en `peach` y estado en `green`
  - *Engram*: `peach` con texto `text`
  - *Tools*: `teal` con contadores en `peach`
  - *MCP*: `blue` con estados en `text`
- **Impacto:** El ojo no sabe a dónde mirar. En lugar de transmitir un sistema cohesivo de telemetría profesional, parece una colección de widgets desconectados compitiendo por atención.

### B. Falta de Profundidad y Superficies ("Todo Flota en el Vacío")
- Las cajas usan el marco de línea delgada `╭─ ─[-]─╮` dibujado directamente sobre el canvas del fondo.
- No hay relleno de superficie (`surface0` o `active_row_bg`), lo que produce una sensación de "rejilla de alambre" con texto apretado en lugar de módulos o tarjetas físicas con volumen.

### C. Ausencia de Jerarquía Clave-Valor (Labels vs Values)
- Rótulos estáticos como `Proyecto:`, `Rama:`, `Modelo:`, `Context:`, `@ 27 lecturas` tienen un peso visual similar al dato real.
- Una regla dorada de UI en dashboards es: **los labels deben ser discretos (`overlay0`) y los datos deben resaltar (`text` en negrita o con contraste limpio)**.

### D. Indicadores de Estado y Acciones Poco Claros
- El badge de colapso `[-]` y `[+]` se confunde con la línea del borde superior.
- Abajo, las leyendas de atajos (`1-9: colap · c: todo · d: libre...`) se muestran en blanco crudo sin contraste respecto al contenido superior.

---

## 2. Benchmark de 10 Repositorios y Modelos TUI Líderes

Se analizaron 10 proyectos de referencia en Rust, Go, C++ y TypeScript que destacan por su diseño visual pulido en consola:

### 1. BTOP (Aristocratos/btop4win / btop)
- **Filosofía:** Visualización densa pero extremadamente limpia con gradientes sutiles y tarjetas bien delimitadas.
- **Patrón de color:** Fondo oscuro uniforme, marcos en gris tenue, un único color primario para headers de sección y degradados de acento funcional exclusivamente para datos dinámicos (CPU, RAM).
- **Lección para Telar:** Los bordes de las tarjetas deben ser siempre neutros (`surface_dim`); el color solo debe destacar el dato que cambia.

### 2. Lazygit (jesseduffield/lazygit)
- **Filosofía:** Máxima sobriedad para jornadas laborales de 8+ horas sin fatiga visual.
- **Patrón de color:** Marco general en gris tenue, marco activo en verde/cian, texto de títulos en blanco limpio sin estridencias, selección con fondo suave (`surface1`).
- **Lección para Telar:** Una UI profesional usa 1 color de acento principal y reserva los colores semánticos (verde, rojo, amarillo) solo para estado de alerta o git.

### 3. Zellij (zellij-org/zellij)
- **Filosofía:** Workspace multiplexer moderno mouse-first con micro-pills y ribbons informativos.
- **Patrón de color:** Badges con fondo sólido redondeado, separación nítida entre pestañas activas e inactivas con contraste WCAG AAA.
- **Lección para Telar:** Las pestañas del dock (`Status`, `Files`, `Changes`) deben lucir como chips de diseño contemporáneo, no como botones planos pegados.

### 4. K9s (derailed/k9s)
- **Filosofía:** Panel de monitoreo de clusters Kubernetes de misión crítica.
- **Patrón de color:** Estructura monocromática de base con semáforo estricto: Verde (Running), Amarillo (Warning), Rojo (Error/Crash). No se usan colores decorativos en datos informativos.
- **Lección para Telar:** El pool y los MCP no deben tener colores decorativos arbitrarios; solo verde (Online), amarillo (Waiting/Rate-limit) o rojo (Error).

### 5. Neovim + Lualine / Dropbar (nvim-lualine/lualine.nvim)
- **Filosofía:** Segmentación visual modular mediante "pills" y fondos diferenciados.
- **Patrón de color:** Secciones `A > B > C` donde el fondo transita de saturado (modo) a tenue (archivo) y oscuro (rama/stats).
- **Lección para Telar:** Cada tarjeta puede tener un encabezado con una mini-cápsula (pill) que agrupe icono + nombre con un fondo `surface0` sutil.

### 6. Superfile (yorukot/superfile)
- **Filosofía:** El explorador de archivos TUI más estético y moderno de la actualidad.
- **Patrón de color:** Tarjetas flotantes con bordes suaves, esquinas redondeadas completas, espaciado generoso (padding) y jerarquía tipográfica basada en tonalidades pastel.
- **Lección para Telar:** Dejar 1 espacio de respiro horizontal y unificar los bordes reduce drásticamente la sensación de abarrotamiento.

### 7. Yazi (sxyazi/yazi)
- **Filosofía:** Ultrarrápido, minimalista y reactivo a temas de 24 bits.
- **Patrón de color:** Acento discreto, fondos de previsualización oscurecidos, selectores sin bordes gruesos.
- **Lección para Telar:** Mantener la barra lateral derecha visualmente equilibrada con la barra lateral izquierda (sin un marco más grueso ni más ruidoso que el izquierdo).

### 8. Bottom (ClementTsang/bottom)
- **Filosofía:** Monitor de recursos multiplataforma altamente configurable.
- **Patrón de color:** Barras de gauge con bloques continuos (`█`, `▆`, `▄`, `▂`), texto de porcentajes en alineación derecha fija.
- **Lección para Telar:** La barra de contexto de tokens (`render_mini_bar`) debe usar bloques homogéneos y alinearse perfectamente con la etiqueta de porcentaje.

### 9. Gentle-Shell / CCG (Gentleman Programming TUI System)
- **Filosofía:** Elegancia "cyber-noir" con acento Hot Pink (`#F43888`), Champagne Gold (`#E0C27A`) y Cyan Eléctrico (`#00E5FF`).
- **Patrón de color:** Banners enmarcados con `╭─`, títulos en Champagne Gold, indicadores activos en Cyan/Lime y fondo obsidian puro.
- **Lección para Telar:** Como Telar es la terminal de Gentle, el sidebar derecho debe reflejar esta firma visual exclusiva en lugar de colores genéricos.

### 10. Linear CLI / Vercel TUI (Design Systems de Nueva Generación)
- **Filosofía:** Menos es más. 95% escala de grises/obsidian y 5% de color intencional.
- **Patrón de color:** Fondo `#080808`, bordes `#222222`, texto `#EDEDED`, labels `#707070`, un solo acento violeta o blanco puro.
- **Lección para Telar:** Una paleta monocromática refinada eleva la percepción de calidad técnica de forma inmediata.

---

## 3. Diez Modelos de Diseño y Paleta Propuestos para el Right Dock

Mapeados sobre los tokens estándar de Telar (`src/app/state.rs:Palette`):

### Modelo 1: **"Gentle Obsidian Signature" (Recomendado para Telar)**
*Inspiración: gentlemanprogramming.com + gentle-shell.*
- **Fondo Canvas:** `sidebar_bg` (Reset / obsidian `#060407`)
- **Fondo de Tarjetas:** `surface0` (`#100A0F` sutilmente iluminado)
- **Marcos de Tarjetas:** `surface_dim` (`#2A1720`) — sutiles y elegantes.
- **Títulos de Tarjetas:** `mauve` (`#E0C27A` Champagne Gold) unificado en todas las tarjetas.
- **Labels (`Proyecto:`, etc.):** `overlay0` (`#8C7380`) atenuado.
- **Valores (`telar-dev`, etc.):** `text` (`#F6EFF3`) blanco perlado.
- **Acentos Activos / Badges:** `accent` (`#F43888` Hot Pink) para elementos en foco y `teal` (`#00E5FF`) para métricas activas.
- **Sensación:** Sofisticada, moderna, auténtica de Gentleman Programming.

### Modelo 2: **"Linear Slate Precision" (Ultra Minimalista)**
*Inspiración: Linear / Vercel CLI.*
- **Fondo Canvas:** `sidebar_bg` (Reset)
- **Fondo de Tarjetas:** `panel_bg` muy tenue
- **Marcos de Tarjetas:** `surface_dim` gris neutro
- **Títulos de Tarjetas:** `text` en negrita (blanco/marfil sobrio)
- **Labels:** `overlay0` gris medio
- **Valores:** `text` blanco limpio
- **Acento:** Único color de foco `accent` (Cian o Azul) solo en el elemento activo.
- **Sensación:** Cero distracción, máxima claridad ejecutiva.

### Modelo 3: **"Cyberpunk Neon Grid" (Gentle-Shell Electric)**
*Inspiración: arch-electric / Cyberdream.*
- **Fondo Canvas:** Obsidian `#05080D`
- **Marcos de Tarjetas:** `teal` (`#00E5FF` cian eléctrico) atenuado al 40%
- **Títulos de Tarjetas:** `accent` (`#FF2DF7` rosa neón brillante)
- **Labels:** `subtext0` azul grisáceo
- **Valores:** `text` blanco hielo (`#DCEBF2`)
- **Métricas:** `green` (`#B6FF3B` lima eléctrico)
- **Sensación:** Hacker retro-futurista de alto impacto.

### Modelo 4: **"Lazygit Muted Functional" (Enfoque Tareas Diarias)**
*Inspiración: Lazygit / Neovim kickstart.*
- **Marcos:** Todos en `surface_dim` (1 solo tono uniforme).
- **Títulos:** Texto plano en `subtext0` con corchetes `[ Estado ]`, `[ Contexto ]`.
- **Valores:** `text` normal.
- **Color semántico estricto:** Verde = OK, Amarillo = Esperando/Dirty, Rojo = Bloqueado.
- **Sensación:** Cero fatiga visual durante jornadas de 10+ horas.

### Modelo 5: **"Tokyo Night Deep Focus"**
*Inspiración: Tokyo Night Storm.*
- **Fondo Canvas:** `#1A1B26`
- **Fondo de Tarjetas:** `#24283B`
- **Marcos:** `#414868`
- **Títulos:** `#7AA2F7` (Azul lavanda suave)
- **Labels:** `#565F89`
- **Valores:** `#C0CAF5`
- **Acento secundario:** `#BB9AF7` (Púrpura suave)
- **Sensación:** Tonos fríos sedantes, ideal para programar de noche.

### Modelo 6: **"Gruvbox Warm Retro Paper"**
*Inspiración: Gruvbox Dark Hard.*
- **Fondo Canvas:** `#1D2021`
- **Marcos:** `#504945`
- **Títulos:** `#FABD2F` (Oro cálido tostado)
- **Labels:** `#928374`
- **Valores:** `#EBDBB2` (Crema cálido)
- **Acentos:** `#83A598` (Azul piedra) y `#B8BB26` (Oliva)
- **Sensación:** Confort analógico cálido estilo teclado mecánico retro.

### Modelo 7: **"Nordic Frost Minimal"**
*Inspiración: Arctic Nord.*
- **Fondo Canvas:** `#2E3440` (Polar Night)
- **Marcos:** `#4C566A`
- **Títulos:** `#88C0D0` (Frost turquesa)
- **Labels:** `#4C566A`
- **Valores:** `#ECEFF4` (Blanco nieve)
- **Métricas:** `#A3BE8C` (Verde aurora)
- **Sensación:** Limpia, fría, organizada y de alto contraste estandinavo.

### Modelo 8: **"Catppuccin Macchiato Harmony"**
*Inspiración: Catppuccin Macchiato.*
- **Fondo Canvas:** `#1E2030`
- **Marcos:** `#494D64`
- **Títulos:** `#C6A0F6` (Mauve suave)
- **Labels:** `#6E738D`
- **Valores:** `#CAD3F5`
- **Métricas:** `#8AADF4` (Azul zafiro)
- **Sensación:** Pastel contemporáneo muy popular y amigable.

### Modelo 9: **"Superfile Card Deck" (Micro-Badges y Pastillas)**
*Inspiración: Superfile.*
- **Estructura:** Cada tarjeta tiene una pastilla de color en su cabecera:
  - `╭─ [● Estado] ─╮` (Icono con pastilla)
- **Bordes:** Finos y redondeados en `surface_dim`.
- **Valores numéricos:** En `accent` con formato tabular alineado.
- **Sensación:** Moderna, estructurada como app web de última generación en TUI.

### Modelo 10: **"Monochrome Charcoal & Champagne"**
*Inspiración: Editorial Design & Neumorphism en terminal.*
- **Base:** 100% escala de carbón (`#121212`, `#1E1E1E`, `#2D2D2D`).
- **Texto:** Escala de gris medio a blanco puro.
- **Único Acento:** Champagne Gold (`#E0C27A`) reservado exclusivamente para la tarjeta activa o seleccionada por el mouse.
- **Sensación:** Lujo discreto ("Quiet Luxury"), máxima elegancia visual.

---

## 4. Recomendaciones Concretas de Implementación en `src/client/shell/dock.rs`

Para transformar el dock de `5.png` en una interfaz moderna y coherente:

1. **Unificar el color de los encabezados de tarjeta:**
   - En lugar de alternar arbitrariamente entre `accent`, `peach`, `yellow`, `mauve`, `teal` y `blue`, usar **un solo token semántico para todos los títulos de tarjetas** (por ejemplo `config.palette.mauve` Champagne Gold o `config.palette.accent` Hot Pink en tema Gentleman).
2. **Aplicar la regla Label-Value:**
   - Modificar `render_card_kv`:
     - `key_style`: Cambiar de `overlay1` a `config.palette.overlay0` (más atenuado).
     - `val_style`: Usar `config.palette.text` con estilo negrita o `accent` solo en valores destacados.
3. **Pills de estado consistentes:**
   - En la tarjeta **POOL** y **MCP**, en lugar de pintar títulos y textos en amarillo o azul, usar un badge semántico:
     - `● En línea` -> `palette.green`
     - `○ Esperando` -> `palette.overlay1`
     - `▲ Warning` -> `palette.yellow`
4. **Simplificar la cabecera superior:**
   - Reemplazar `:: TELAR · GENTLE-AI ::` en cian brillante por un banner sobrio y centrado en `palette.overlay0` o `palette.mauve` sutil.
5. **Footer de atajos limpio:**
   - Cambiar los atajos de teclado (`1-9: colap · c: todo...`) a estilo `palette.overlay0` atenuado para que no llamen la atención a menos que se busquen deliberadamente.
