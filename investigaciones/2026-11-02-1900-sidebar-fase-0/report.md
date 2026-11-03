# Sidebar nativa configurable: investigación de fase 0

## Resumen ejecutivo

Recomendación: diseñar una sidebar **nativa, de presentación**, con un ajuste independiente por sección y un selector **dentro de CPAMC** para la presentación inspirada en hudr o en Auto Switcher Pro. No se ha demostrado que estas opciones compartan un contrato de datos ni que sean intercambiables como motores de control. Conservar los sistemas de origen como autoridades y representar lo desconocido y lo obsoleto explícitamente. Confianza alta para la dirección de límites; baja para la viabilidad completa hasta verificar contratos y datos reales. [S-001][S-002][S-003][S-004]

La imagen muestra una tarjeta de pool Gemini, ventanas 5 horas/semanal, líder/reserva y centinela; es una referencia visual puntual, no especificación de API ni evidencia de que esas cifras estén disponibles para Telar. [S-005]

## Pregunta de investigación

¿Cómo trasladar el sidebar completo de hudr a la TUI de Telar, con casillas por sección y elección de presentación CPAMC, sin duplicar autoridad ni comprometer seguridad y compatibilidad?

## Recomendación o respuesta

Separar configuración visual persistente, proyecciones tipadas con procedencia/fecha y acciones explícitas, autorizadas por la autoridad correspondiente. Las casillas ocultan secciones; no deberían apagar servicios ni modificar cuentas por implicación. El selector CPAMC selecciona presentación/fuente compatible, no toda la sidebar ni un cambio implícito de estrategia de routing. Esta es una propuesta, no una funcionalidad existente. [S-001][S-003][S-006]

## Hallazgos principales

Inventario de referencia del README de hudr 1.0.0, commit fijado: Estado; Contexto; Engram; Integraciones/cuentas; Antigravity/cuotas; Git y Cambios; Herramientas; Servidores MCP. Incluye colapsar, scroll, refrescar y migración; la enumeración describe el ejemplo/documentación, no prueba de paridad funcional. [S-002] La captura añade la presentación pool/centinela; no demuestra una novena sección hudr. [S-005]

| Campo/sección | Fuente candidata y alcance | Frescura y ausencia | Seguridad / límite |
|---|---|---|---|
| Estado: proyecto, rama, modelo, cuenta | Sesión/agente y workspace activos; Git para rama | Identidad por pane/sesión debe verificarse; desconectado ≠ inactivo | Ruta y nombre de cuenta pueden ser sensibles. |
| Contexto: tokens, ventana, coste | Telemetría del agente/modelo de la sesión | Sin métricas o límites: desconocido; no calcular coste ficticio | No extraer prompts ni resultados de herramientas. |
| Engram: estado, proyecto, observaciones/cloud | Engram, autoridad propia | Sin conexión o sin capacidades: no disponible; datos con marca temporal | No copiar base ni exportar recuerdos. |
| Integraciones y Antigravity: cuenta, cuotas, resets | Proveedor/CLIProxyAPI y adaptador autorizado; hudr lee auth y llama al proveedor [S-007] | Mostrar fecha y caducidad; error, permiso denegado y cero son distintos | No leer ni mostrar tokens desde render; no asumir cuotas universales. |
| CPAMC estilo Pro: pool, ventanas, líder, reserva, centinela | Contrato verificable de Auto Switcher Pro y/o extensión Pi; captura [S-004][S-005] | Endpoints, correspondencia exacta y eventos no establecidos; datos caducados como tales | Prefijos y cuentas sensibles; autoridad de rotación no pasa a Telar. |
| Git y cambios | Git del workspace concreto [S-008] | Instantánea con tiempo; no confundir cambios de Git con cambios del agente | Evitar mostrar contenido de diffs por defecto; trabajo fuera de render. |
| Herramientas | Eventos agregados del agente pertinente | Sin fuente de eventos no inventar recuentos; sesión acotada | No exponer argumentos, comandos ni resultados. |
| MCP | Estado/capacidades del host del agente | Configurado no significa conectado; ausencia desconocida | No exponer secretos ni URL privadas. |

Las fuentes candidatas son propuestas, no contratos confirmados para cada agente o endpoint. Los intervalos adecuados y el alcance multi-cliente requieren medición y decisión posterior. [S-001][S-002]

## Revisión de evidencia

- **Telar, inspección, confianza alta:** checkout `cade6079ebfd203ceb10effa2d9681c4f8ea2573`, Cargo identifica `herdr` 0.9.5. `src/ui/sidebar.rs` contiene geometría y entradas de espacios/agentes; `src/config/sidebar.rs` configura tokens y filas; `src/client/shell/settings.rs` contiene ajustes Theme, Indicators, Sound, Toast, Typography e Integrations. Existencia de estos mecanismos no demuestra los nuevos checkboxes. [S-006]
- **Límite, confianza alta:** AGENTS separa hechos compartidos del runtime y presentación del cliente; render puro y sin I/O en bucles escalados. El canon protege codecs, sesión y registro de plugins; proyecciones no poseen procesos. El canon contiene además afirmaciones históricas de ejecución y diseño que no se revalidaron aquí, algunas más fuertes que sus propios detalles; no son prueba nueva de funcionamiento. [S-001]
- **Widget, confianza alta:** `src/ports/widget.rs` define `WidgetProjectionPort`, métricas limitadas y ejemplo en memoria; no se ha probado su conexión a la sidebar ni que cubra pools, frescura o plugins dinámicos. Su default booleano y tratamiento de proveedor sin cuota no establecen semántica fiable de desconocido. [S-006]
- **hudr, confianza media:** README describe proceso Node separado, polling y acciones sobre sesiones; código de cuota lee archivos auth y usa bearer hacia proveedor; código Git ejecuta procesos. Son patrones observados, no recomendación de copiarlos ni pruebas de seguridad/eficiencia. No se validaron sus cifras de rendimiento o comportamiento en vivo. [S-002][S-007][S-008]
- **Pro, confianza media:** el README del plugin declara C-ABI v1 en host CLIProxyAPI, UI `/profiles`, y PATCH autenticado de campos auth; la extensión Pi separada declara proveedor, failover y QuotaCard. Son dos superficies distintas y sus afirmaciones no se probaron. No confundir el plugin CLIProxyAPI con el registro ejecutable de Telar ni suponer que su C-ABI esté admitida por Telar. [S-003][S-004]

## Compensaciones y riesgos

La fidelidad visual no justifica acceso directo a credenciales, SQLite ajena o mutaciones de sesión. Mantener acciones de refresh, migración, failover y cambios de prefijo separadas del render y sujetas a capacidades explícitas, confirmación apropiada y autoridad de origen; no inferir que alternar presentación modifica el backend. Los clientes remotos requieren permisos, transporte y redacción distintos de localhost. No introducir cambios incompatibles a codecs generación 1. [S-001][S-007]

Licencias: `hudr/package.json` declara MIT pero no se observó archivo LICENSE en el árbol consultado: verificar titularidad y texto antes de reutilizar código. La raíz de Pro contiene texto rotulado PolyForm Noncommercial 1.0.0 y restricciones comerciales; GitHub lo clasifica NOASSERTION. Revisar texto exacto, ámbitos de subdirectorios y dependencias con asesoría antes de redistribuir; no inferir permiso comercial. [S-002][S-009] Inspiración de interfaz no establece por sí sola derechos de copia de código o activos.

## Alternativas consideradas

1. Ejecutar hudr como pane: conserva aislamiento descrito, pero no satisface por sí mismo integración nativa, configuración unificada ni límites de credenciales. [S-002]
2. Copiar lecturas y escrituras directas: dependencia frágil y autoridad duplicada; no recomendado. [S-007]
3. Incorporar directamente el plugin C-ABI a Telar: mezcla hosts, lifecycle y privilegios; no hay evidencia de compatibilidad. [S-003]
4. Proyecciones nativas y adaptadores por capacidad: opción recomendada para diseño posterior, condicionada a contratos y ensayos. [S-001]

## Incógnitas y límites

No se ejecutaron pruebas, instalaciones, servicios, redes contra credenciales ni cambios de código; únicamente lectura local, imagen y fuentes públicas. No se inspeccionaron exhaustivamente todos los consumidores o implementación externa. No se verificaron memoria 3261, API de telemetría de pool/centinela, asociación pane-cuenta, política de expiración, par exacto de versiones CPAMC/plugin, defaults de configuración, derechos por subárbol, ni equivalencia funcional de las dos opciones. La captura es un ejemplo visual, no contrato. El alcance y resultado de las acciones de hudr y Pro requieren auditoría antes de habilitarlas. [S-002][S-003][S-004][S-005]

## Próximas acciones recomendadas

1. Inventariar contratos reales por campo y versión, permisos, autoridades y tiempos de caducidad; confirmar identidad por sesión/pane y modos remoto/local antes de diseñar adaptadores.
2. Definir persistencia, migración, accesibilidad, scroll y geometría estrecha para casillas y selector; caracterizar sidebar existente y compatibilidad cliente/servidor.
3. Especificar proyecciones con estado disponible/obsoleto/error/no compatible y acciones separadas; probar con datos sintéticos, incluidos secretos y desconexión.
4. Revisar licencias y compatibilidad concreta del plugin; después planificar pruebas de integración y rendimiento multipane sin usar credenciales reales sin autorización específica.
