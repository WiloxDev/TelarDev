# Investigación: 12 Temas Modernos para Descanso Visual y Programación

**Fecha:** 2026-11-03 14:41  
**Objetivo:** Investigar, evaluar y curar al menos 10 estilos modernos orientados a la ergonomía visual, bajo cansancio ocular y estética refinada para programadores, mapeados al contrato de 19 tokens de `Palette` de Telar (`src/app/state.rs`).  
**Modo:** Solo lectura / Documentación técnica.

---

## 1. Criterios Ergonómicos de Descanso Visual

1. **Rango de contraste controlado (Sweet spot 7:1 a 10:1):** Evita el deslumbramiento y la aberración cromática generada por el negro puro (`#000000`) contra el blanco puro (`#ffffff`).
2. **Lienzo de fondo atenuado y cálido:** Fondos pizarra, carbón orgánico o piedra cálida (p. ej., `#2d353b` en Everforest, `#212733` en Ayu Mirage, `#22272e` en GitHub Dark Dimmed) reducen la fatiga del músculo ciliar.
3. **Distribución cromática perceptualmente calibrada:** Se desaturan los acentos secundarios y se evitan los azules y rojos puros de alta frecuencia para eliminar el re-enfoque constante de la retina.
4. **Temas claros tipo pergamino:** Para trabajo de día con luz natural, se emplean fondos marfil/papel cálido (`#fdf6e3`), evitando el impacto cegador del blanco absoluto.

---

## 2. Los 12 Temas Seleccionados

1. **Everforest Dark (Medium):** Inspiración botánica y terrosa. El estándar de la comunidad Neovim para reducir fatiga nocturna.
2. **Everforest Light (Soft):** Tono pergamino natural sin reflejo cegador para jornadas con luz diurna.
3. **Ayu Mirage:** Equilibrio crepuscular entre tonos fríos de pizarra marina y acentos pastel cálidos.
4. **Poimandres:** Diseño minimalista de pmndrs (`#1b1e28`), atmósfera serena y limpia.
5. **Catppuccin Frappé:** La variante intermedia de Catppuccin, más suave y reposada que Mocha.
6. **GitHub Dark Dimmed:** El estándar ergonómico oficial de GitHub diseñado para revisión intensiva de código.
7. **Nightfly:** Azul medianoche profundo (`#011627`) con acentos esmeralda y sandía perfectamente calibrados.
8. **Oxocarbon Dark:** Diseño suizo-industrial minimalista derivado de IBM Carbon (`#161616`).
9. **Monokai Pro (Octagon):** Evolución ergonómica de Monokai que sustituye los neones agresivos por tonos arcilla y ámbar.
10. **Melange Dark:** Armonías matemáticas basadas en el sistema de color Munsell y tonos de madera cálida.
11. **Zenbones Dark:** Diseñado en el espacio de color CIELAB con pasos lumínicos estrictos para máxima tranquilidad retiniana.
12. **Cyberdream:** Estética contemporánea tipo cyberpunk con base carbón y alta nitidez en pantallas HiDPI.

---

## 3. Matriz de Mapeo de Tokens (19 tokens de `Palette`)

| Token | Everforest Dark | Everforest Light | Ayu Mirage | Poimandres | Catppuccin Frappé | GitHub Dark Dimmed |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **accent** | `#a7c080` | `#8da101` | `#ffcc66` | `#5de4c7` | `#8caaee` | `#539bf5` |
| **panel_bg** | `#2d353b` | `#fdf6e3` | `#212733` | `#1b1e28` | `#303446` | `#22272e` |
| **sidebar_bg** | `Reset` | `Reset` | `Reset` | `Reset` | `Reset` | `Reset` |
| **active_row_bg** | `#343f44` | `#f4f0d9` | `#242936` | `#252b37` | `#292c3c` | `#2d333b` |
| **selection_bg** | `#3d484d` | `#e6e2cc` | `#343d4d` | `#303340` | `#51576d` | `#373e47` |
| **surface0** | `#3d484d` | `#efebd4` | `#272d38` | `#252b37` | `#414559` | `#2d333b` |
| **surface1** | `#475258` | `#e6e2cc` | `#343d4d` | `#303340` | `#51576d` | `#373e47` |
| **surface_dim** | `#232a2e` | `#efebd4` | `#1c212b` | `#171922` | `#232634` | `#1c2128` |
| **overlay0** | `#7a8478` | `#a6b0a0` | `#5c6773` | `#7390aa` | `#737994` | `#768390` |
| **overlay1** | `#859289` | `#829181` | `#707a8c` | `#a6accd` | `#838ba7` | `#909dab` |
| **text** | `#d3c6aa` | `#5c6a72` | `#cccac2` | `#e4f0fb` | `#c6d0f5` | `#adbac7` |
| **subtext0** | `#9da9a0` | `#708077` | `#959da6` | `#91b4d5` | `#a5adce` | `#768390` |
| **mauve** | `#d699b6` | `#df69ba` | `#d4bfff` | `#d0679d` | `#ca9ee6` | `#b083f0` |
| **green** | `#a7c080` | `#8da101` | `#bae67e` | `#5de4c7` | `#a6d189` | `#57ab5a` |
| **yellow** | `#dbbc7f` | `#dfa000` | `#ffd580` | `#fffac2` | `#e5c890` | `#c69026` |
| **red** | `#e67e80` | `#f85552` | `#f28779` | `#d0679d` | `#e78284` | `#e5534b` |
| **blue** | `#7fbbb3` | `#3a94c5` | `#73d0ff` | `#89ddff` | `#8caaee` | `#539bf5` |
| **teal** | `#83c092` | `#35a77c` | `#95e6cb` | `#5de4c7` | `#81c8be` | `#39c5cf` |
| **peach** | `#e69875` | `#f57d26` | `#ffcc66` | `#fae3b0` | `#ef9f76` | `#f69d50` |

---

| Token | Nightfly | Oxocarbon Dark | Monokai Pro Octagon | Melange Dark | Zenbones Dark | Cyberdream |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **accent** | `#82aaff` | `#33b1ff` | `#ffd866` | `#ebc06d` | `#819b69` | `#5eff6c` |
| **panel_bg** | `#011627` | `#161616` | `#282a3a` | `#292522` | `#1c1917` | `#16181a` |
| **sidebar_bg** | `Reset` | `Reset` | `Reset` | `Reset` | `Reset` | `Reset` |
| **active_row_bg** | `#0b2942` | `#262626` | `#323546` | `#34302c` | `#282523` | `#1e2124` |
| **selection_bg** | `#1d3b53` | `#393939` | `#414458` | `#403a36` | `#3d3835` | `#2c313a` |
| **surface0** | `#0b2942` | `#262626` | `#323546` | `#34302c` | `#282523` | `#1e2124` |
| **surface1** | `#1d3b53` | `#393939` | `#414458` | `#48413a` | `#3d3835` | `#2c313a` |
| **surface_dim** | `#010e1a` | `#121212` | `#212330` | `#221f1d` | `#161412` | `#101112` |
| **overlay0** | `#7c8f8f` | `#525252` | `#696d77` | `#786d65` | `#68615c` | `#5b6078` |
| **overlay1** | `#9caab6` | `#dde1e6` | `#888d9d` | `#a3978c` | `#888079` | `#7b8496` |
| **text** | `#c3ccdc` | `#ffffff` | `#eaf2f1` | `#ece1d7` | `#dcd5cb` | `#ffffff` |
| **subtext0** | `#9caab6` | `#f2f4f8` | `#a4a8b6` | `#c1b5a8` | `#9f9891` | `#b4b5b9` |
| **mauve** | `#c792ea` | `#be95ff` | `#ab9df2` | `#cf9bc2` | `#b279a7` | `#bd5eff` |
| **green** | `#21c7a8` | `#42be65` | `#a9dc76` | `#85b695` | `#819b69` | `#5eff6c` |
| **yellow** | `#ecc48d` | `#ffe97b` | `#ffd866` | `#ebc06d` | `#c48858` | `#f1ff5e` |
| **red** | `#ff5874` | `#ee5396` | `#ff6188` | `#d47766` | `#de6e7c` | `#ff6e5e` |
| **blue** | `#82aaff` | `#33b1ff` | `#78dce8` | `#7b9695` | `#6099a0` | `#5ea1ff` |
| **teal** | `#7fdbca` | `#3ddbd9` | `#78dce8` | `#89b3b6` | `#66a5ad` | `#5ef1ff` |
| **peach** | `#f78c6c` | `#ff7eb6` | `#fc9867` | `#e49b5d` | `#d68c67` | `#ffbd5e` |

---

## 4. Recomendación de Uso

- **Día o ambientes iluminados:** `Everforest Light (Soft)` (pergamino anti-reflejo).
- **Tarde / Luz tenue:** `Everforest Dark` o `Melange` (tonos orgánicos y reposados).
- **Noche profunda:** `Poimandres` o `Ayu Mirage` (azul crepúsculo sin contraste punzante).
- **Pantallas OLED:** `Oxocarbon Dark` o `Cyberdream` (negros puros con píxeles apagados).
- **Jornadas continuas de 8+ horas:** `GitHub Dark Dimmed` o `Catppuccin Frappé`.
