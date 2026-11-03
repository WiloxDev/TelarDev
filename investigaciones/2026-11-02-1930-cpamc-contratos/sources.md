# Sources

## Source Index

### S-001: Canon e instrucciones de Telar
- Family: PRIMARY
- URL or locator: `AGENTS.md`; `docs/architecture/telar-canon.md`
- Date/version: árbol de trabajo, revisado 2026-11-03
- Access method: lectura local con bash
- Used for: límites, versiones fijadas y evidencia histórica
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Canon mezcla decisiones y afirmaciones de ejecución; estas últimas no se revalidaron. La salida de lectura extensa fue truncada; se consultaron las secciones pertinentes visibles.

### S-002: Manifest handler y registro de rutas del plugin
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/plugins/auto-switcher-pro/internal/plugin/manifest_handler.go ; https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/plugins/auto-switcher-pro/internal/plugin/plugin.go
- Date/version: commit `2e9c7f64bae90e55f835da102b9de23905ecbd9a`
- Access method: GitHub raw con Python urllib
- Used for: campos manifest, defaults, rutas registradas y efectos potenciales
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Inspección estática; montaje externo y middleware no comprobados. Invocación de asignación de alias no demuestra por sí sola persistencia.

### S-003: Store de extensión Pi
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/extensions/cpamc-auto-switcher/src/store.ts
- Date/version: commit `2e9c7f64bae90e55f835da102b9de23905ecbd9a`
- Access method: GitHub raw con Python urllib
- Used for: cache, consulta, mapeo, fallback, credenciales y defaults
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Comportamiento de cliente; no evidencia de éxito de rutas en vivo.

### S-004: Investigación sidebar fase 0 y fuentes
- Family: SECONDARY
- URL or locator: `investigaciones/2026-11-02-1900-sidebar-fase-0/report.md`; `investigaciones/2026-11-02-1900-sidebar-fase-0/sources.md`
- Date/version: 2026-11-02, según directorio
- Access method: lectura local con bash
- Used for: alcance anterior, captura, separación de responsabilidades y licencia
- Usefulness: MATERIAL
- Confidence impact: MEDIUM
- Notes: La descripción de la captura y licencia se reutiliza de la fase anterior; sin examen nuevo de la imagen o dictamen legal.

### S-005: Registro de acciones y extensión de failover
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/plugins/auto-switcher-pro/internal/plugin/plugin.go ; https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/extensions/cpamc-auto-switcher/README.md
- Date/version: commit `2e9c7f64bae90e55f835da102b9de23905ecbd9a`
- Access method: GitHub raw con Python urllib
- Used for: acciones de mutación y failover declarado
- Usefulness: MATERIAL
- Confidence impact: MEDIUM
- Notes: Las descripciones de operaciones no prueban transaccionalidad ni comportamiento de despliegue.

### S-006: Constante de versión de plugin
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/plugins/auto-switcher-pro/internal/version/version.go
- Date/version: commit fijado; constante `0.1.1`
- Access method: GitHub raw con Python urllib
- Used for: identificación del código de plugin
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: No identifica inequívocamente binario instalado o release.

### S-007: Tipos y almacenamiento de telemetría
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/plugins/auto-switcher-pro/internal/plugin/predictive.go
- Date/version: commit fijado
- Access method: GitHub raw con Python urllib
- Used for: porcentajes restantes, familia y updated_at
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Updated_at se asigna si falta, pero también puede proceder del emisor; no garantiza origen ni integridad.

### S-008: Registro de pools
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/plugins/auto-switcher-pro/internal/plugin/registry.go
- Date/version: commit fijado
- Access method: GitHub raw con Python urllib
- Used for: estructura pool y líderes por proyecto
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Inspección selectiva; no auditoría completa de concurrencia o persistencia.

### S-009: Handler predictivo
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/plugins/auto-switcher-pro/internal/plugin/predictive_handler.go
- Date/version: commit fijado
- Access method: GitHub raw con Python urllib
- Used for: GET y POST de telemetría, respuesta y lote
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Ruta montada y autorización efectivas no verificadas.

### S-010: Cliente Go de management
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/internal/client/client.go
- Date/version: commit fijado
- Access method: GitHub raw con Python urllib
- Used for: auth-files, api-call, PATCH y cabeceras
- Usefulness: MATERIAL
- Confidence impact: MEDIUM
- Notes: Cliente esperado, no especificación independiente ni prueba de host v8.0.51.

### S-011: Poller de cuota del plugin
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/plugins/auto-switcher-pro/internal/plugin/quota_poller.go
- Date/version: commit fijado
- Access method: GitHub raw con Python urllib
- Used for: frecuencia, seguridad, proveedor, límites y default de ventana
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Código no ejecutado; API de proveedor no verificada ni contactada.

### S-012: Renderer de tarjeta en extensión
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/extensions/cpamc-auto-switcher/src/adapter.ts
- Date/version: commit fijado
- Access method: GitHub raw con Python urllib
- Used for: sentinel, representaciones y valores de fallback
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Defaults observados en ramas del renderer; no se afirma que aparezcan siempre.

### S-013: Entrypoint de extensión
- Family: IMPLEMENTATION
- URL or locator: https://github.com/WiloxDev/cpamc-auto-switcher-pro/blob/2e9c7f64bae90e55f835da102b9de23905ecbd9a/extensions/cpamc-auto-switcher/index.ts
- Date/version: commit fijado
- Access method: GitHub raw con Python urllib
- Used for: ciclo de actualización y programación proactiva
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Código inspeccionado, sin ejecución de hooks o comprobación de comportamiento en concurrencia.
