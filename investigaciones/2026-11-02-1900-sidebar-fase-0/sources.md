# Sources

## Source Index

### S-001: Instrucciones y canon de Telar
- Family: PRIMARY
- URL or locator: `AGENTS.md`; `docs/architecture/telar-canon.md`
- Date/version: checkout `cade6079ebfd203ceb10effa2d9681c4f8ea2573`, con contenido de árbol de trabajo
- Access method: lectura local con bash
- Used for: límites, compatibilidad y autoridad
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: El canon combina planes y registros históricos; no se repitieron los ensayos ni se asumió que cada afirmación histórica esté validada.

### S-002: hudr README, manifiesto y árbol
- Family: PRIMARY
- URL or locator: https://github.com/Totopo27/hudr/tree/e764f8c1bef04cc32406bd4a3128455fbcf29128 ; `README.md`, `package.json`
- Date/version: commit `e764f8c1bef04cc32406bd4a3128455fbcf29128`, paquete 1.0.0
- Access method: GitHub API y raw mediante curl
- Used for: inventario, comportamiento declarado, licencia declarada
- Usefulness: MATERIAL
- Confidence impact: MEDIUM
- Notes: No se ejecutó. Árbol consultado no muestra archivo LICENSE; README y rendimiento son declaraciones del proyecto.

### S-003: Auto Switcher Pro plugin README
- Family: PRIMARY
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/plugins/auto-switcher-pro/README.md
- Date/version: commit `2e9c7f64bae90e55f835da102b9de23905ecbd9a`; ABI declarada v1
- Access method: curl raw
- Used for: host, ABI, UI y mutaciones declaradas
- Usefulness: MATERIAL
- Confidence impact: MEDIUM
- Notes: Documentación de proyecto; integración y controles no probados.

### S-004: Extensión Pi/Gentle-Shell README
- Family: PRIMARY
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/extensions/cpamc-auto-switcher/README.md
- Date/version: commit `2e9c7f64bae90e55f835da102b9de23905ecbd9a`
- Access method: curl raw
- Used for: separación entre extensión de agente y plugin backend; QuotaCard declarada
- Usefulness: MATERIAL
- Confidence impact: MEDIUM
- Notes: Declaraciones sin prueba en vivo; no se siguieron instrucciones de instalación.

### S-005: Captura suministrada
- Family: PRIMARY
- URL or locator: `/mnt/c/Users/WilsonLavio/Pictures/ty.png`
- Date/version: desconocida
- Access method: lectura de imagen
- Used for: referencia visual de pool y centinela
- Usefulness: MATERIAL
- Confidence impact: MEDIUM
- Notes: Solo apariencia observada; datos, origen y semántica no verificados.

### S-006: Código de interfaz y widget de Telar
- Family: IMPLEMENTATION
- URL or locator: `Cargo.toml`, `src/ui/sidebar.rs`, `src/config/sidebar.rs`, `src/client/shell/settings.rs`, `src/ports/widget.rs`
- Date/version: checkout `cade6079ebfd203ceb10effa2d9681c4f8ea2573`, árbol de trabajo
- Access method: bash, rg y sed
- Used for: superficies existentes, puerto y limitaciones
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Inspección parcial, no prueba de conexión ni ejecución; árbol puede contener cambios locales.

### S-007: hudr colector de cuotas
- Family: IMPLEMENTATION
- URL or locator: https://github.com/Totopo27/hudr/blob/e764f8c1bef04cc32406bd4a3128455fbcf29128/src/collectors/quotas.mjs
- Date/version: commit `e764f8c1bef04cc32406bd4a3128455fbcf29128`
- Access method: curl raw y búsqueda selectiva
- Used for: lectura de auth y solicitudes bearer
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Lectura de código, sin lectura de archivos de usuario ni solicitud al proveedor.

### S-008: hudr colector Git
- Family: IMPLEMENTATION
- URL or locator: https://github.com/Totopo27/hudr/blob/e764f8c1bef04cc32406bd4a3128455fbcf29128/src/collectors/git.mjs
- Date/version: commit `e764f8c1bef04cc32406bd4a3128455fbcf29128`
- Access method: curl raw y búsqueda selectiva
- Used for: comandos Git de la referencia
- Usefulness: SUPPORTING
- Confidence impact: MEDIUM
- Notes: Inspección parcial, no medición de costes.

### S-009: Licencia raíz Auto Switcher Pro
- Family: PRIMARY
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/LICENSE
- Date/version: commit `2e9c7f64bae90e55f835da102b9de23905ecbd9a`, copyright 2026
- Access method: curl raw y metadatos GitHub API
- Used for: restricciones declaradas y necesidad de revisión
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Texto del repositorio; no se determinó cobertura jurídica de cada subárbol ni compatibilidad comercial.
