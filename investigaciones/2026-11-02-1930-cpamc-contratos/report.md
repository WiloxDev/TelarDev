# Contratos CPAMC y Auto Switcher para la sidebar

## Resumen ejecutivo

La integración viable a investigar es una **proyección de solo lectura con capacidad detectada**, no una segunda autoridad de rotación. Existe un contrato concreto candidato: `GET /v0/management/auto-switcher-pro/manifest`, consumido por la extensión y servido por el plugin inspeccionado. No está demostrada su disponibilidad con el CLIProxyAPI v8.0.51 y CPAMC v1.27.2 fijados por Telar. Especialmente importante: el manifiesto fabrica 95% restante cuando falta telemetría; la tarjeta también contiene otros valores de ejemplo. No se debe presentar esa cifra como cuota real. [S-001][S-002][S-003]

Confianza **alta** en el comportamiento estático señalado del commit inspeccionado; **baja** en compatibilidad desplegada, permisos efectivos y fidelidad de cada campo. No se ejecutaron servicios, pruebas de API ni acceso a credenciales. La fase 0 ya cubre inventario general y límites de la sidebar; aquí se refinan los contratos. [S-004]

## Pregunta de investigación

¿Qué datos, autenticación, caducidad y autoridades concretas permitirían reproducir la tarjeta CPAMC de referencia sin asumir que la consola, el plugin y la extensión de agente son el mismo producto?

## Recomendación o respuesta

1. Tratar CPAMC como frontend; CLIProxyAPI como host de management; Auto Switcher Pro como plugin con registro y estado propios; la extensión Pi/Gentle-Shell como cliente y posible controlador adicional. La presencia del HTML no prueba la del plugin. [S-001][S-002][S-003]
2. Antes de diseñar un adaptador definitivo, fijar y verificar la combinación exacta y acordar una fuente que distinga cuota real de valores por defecto. Hasta entonces ofrecer «no disponible / desconocido», no `0%`, ni 95%, ni un número ficticio.
3. Separar UI y lectura del control: ningún cambio de estilo o checkbox implica iniciar servicios, forzar rotación o modificar auth. Una acción futura requiere contrato de autoridad y permisos explícitos, no deducidos de un GET. [S-004][S-005]

## Hallazgos principales

### Versiones y superficies

El canon menciona CLIProxyAPI `v8.0.51` y frontend CPAMC `v1.27.2` y resultados históricos de HTTP; no se repitieron ni se estableció compatibilidad con Pro. El código Pro examinado está fijado al commit `2e9c7f64bae90e55f835da102b9de23905ecbd9a`; su constante interna de versión dice `0.1.1`, que no identifica por sí sola un release distribuido ni equivale a la versión del frontend o del host. El plugin declara ABI C v1; no se verificó que el host fijado implemente esa ABI o las rutas montadas. [S-001][S-002][S-006]

| Superficie observada | Contrato estático y significado | Límite |
|---|---|---|
| `GET /v0/management/auto-switcher-pro/manifest` | Extensión envía `Accept: application/json` y Bearer si dispone de clave; plugin registra `/auto-switcher-pro/manifest`. Respuesta directa `{ok,timestamp,epoch,pools,telemetry,accounts,standalone_aliases?,lifecycle?,lifecycles}`. `timestamp` es Unix segundos de generación, no fecha de cada cuota. [S-002][S-003] | Montaje y autorización reales dependen del host. El handler admite GET/HEAD, pero el registro mostrado declara GET; HEAD externo no garantizado. |
| `accounts[]` | `account_key`, `email`, `pool_id`, `base_prefix`, `role` (`leader`, `reserve`, `unassigned`), `alias`, `window_5h_pct`, `weekly_pct`, `window_5h_reset_in`, `weekly_reset_in`, `is_cooling`, `fatigue_pct`, y estabilidad opcional. Porcentajes son **restantes**; resets en segundos. [S-002][S-007] | Resumen sin timestamp de muestra individual ni indicador de valor sintético. Identificadores pueden revelar emails y nombres de archivos. |
| `pools[]` | `id`, `name`, `family`, `base_prefix`, `members`, `active_leader`, `leader_auth`, `reserves`, `policy`, timestamps y otros campos opcionales. `active_leader` es líder dinámico; `leader_auth` es compatibilidad/fallback. [S-008] | Los roles son visión del plugin, no prueba de cuenta realmente usada por una petición concreta ni por un pane. |
| `telemetry[]` | `account_key`, `family?`, porcentajes restantes, resets, `plan?`, `updated_at`; también GET y POST en `/auto-switcher-pro/predictive/telemetry` registrados por plugin. POST admite objeto individual o `{quotas:[...]}`. [S-007][S-009] | Existen múltiples productores y claves por familia/base; correlación y validez deben verificarse. `updated_at` puede ser aportado por productor; no demuestra medición directa del proveedor. |
| Fallback de extensión | Consulta en paralelo `/auto-switcher-pro/predictive/telemetry` **sin** `/v0/management` y `/v0/management/auto-switcher-pro/pools`; después lee estado local guardado. [S-003] | La diferencia de rutas es real en el cliente; no se verificó que ambas sean accesibles en un despliegue. El fallback local no es autoridad remota ni garantiza frescura. |
| CLIProxyAPI | Cliente Pro usa `GET /v0/management/auth-files` → `{files:[{id,auth_index,name,type,provider,status,disabled,email,project_id}]}`; `POST /v0/management/api-call` con `{authIndex,method,url,header?,data?}` devuelve `{status_code,header,body}`. [S-010] | Es el contrato esperado por ese cliente, no validación independiente de todos los campos de v8.0.51. `api-call` es capacidad privilegiada de proxy, no necesaria para render. |

### Correspondencia con captura

La captura examinada en fase 0 muestra pool Gemini, ventanas 5h y semanal, líder/reserva y centinela; es referencia visual, no una respuesta API. [S-004]

| Elemento | Mapeo candidato | Confianza y límites |
|---|---|---|
| Pool/familia/prefijo | `pools[].id`, `family`, `base_prefix`; `accounts[].pool_id` | Media: vínculo por IDs solo tras validar consistencia; no inferir asociación con pane. [S-002][S-008] |
| Líder y reserva | `accounts[].role`, `email`/`alias`, `pool_id`; `pools[].active_leader` | Media: líder global de manifiesto no garantiza afinidad por proyecto. Registro también mantiene `projectLeaders` volátil. No escoger una «siguiente» reserva como verdad sin política contrastada. [S-002][S-008] |
| 5h/semanal y libre/uso | Telemetría por cuenta y familia: restante = `window_5h_pct`/`weekly_pct`; uso = `100 - restante` solo con valores válidos y observados; reset `*_reset_in` en segundos | Baja hasta comprobar la procedencia de cada ventana; el poller rellena 100 para una ventana ausente, y el manifiesto 95 para cuota ausente. Incluso `updated_at` no certifica ambas ventanas. No promover cálculos en ausencia de un indicador de validez. [S-002][S-007][S-011] |
| Centinela | La extensión registra la tarjeta en la ranura UI `sentinel` y también usa telemetría de guardas locales. | Alta como nombre de ranura; baja como indicador de salud o estado remoto. No hay contrato demostrado que convierta la etiqueta en «protección activa». [S-012] |
| Duración líder / ritmo / candidato | `lifecycle`/`lifecycles` y cálculos del render son candidatos parciales. | No reproducir cifras por defecto: renderer usa, entre otros, `14h`, `14h 06m`, `21` y `0%` si faltan datos. Ritmo y resets no equivalen a hechos del proveedor. [S-012] |
| Cuenta activa de sesión, garantía de cuota, identidad real por pane | Ninguna correspondencia confirmada | **No soportado por evidencia**; mostrar desconocido, no vincular por nombre o prefijo. [S-003][S-008] |

## Revisión de evidencia

**Hecho estático, confianza alta:** el manifiesto crea 95.0 para ambos porcentajes cuando falta `UpdatedAt`; su `accounts[]` no incorpora ese `UpdatedAt`. La extensión a su vez usa 95 si faltan campos, y fallback con ceros. Aunque se cruce con `telemetry[]`, la estructura del poller puede suplir 100 a una ventana no observada; no existe en lo inspeccionado un bit por ventana que garantice medición. Por tanto, un timestamp reciente no basta para resolver toda la ambigüedad. [S-002][S-003][S-011]

**Frescura, confianza alta en el código:** extensión cachea cuentas 25 s y fuerza consulta periódica cada 25 s; el poller autónomo inicia tras 2 s y programa cada 5 min, con hasta 3 consultas concurrentes y timeout de 15 s. Esos periodos son distintos: recibir un manifiesto nuevo no significa cuota nueva; fallos y latencia pueden aumentar edad indefinidamente. No se establece SLA ni TTL universal. El reset relativo puede quedar obsoleto: conservar instante de muestra, tiempo local de recepción, incertidumbre de reloj y saturar visualización sin inventar nuevo reset. [S-003][S-011][S-013]

**Autenticación, confianza media:** el cliente de extensión busca clave en variable de entorno y archivos y envía Bearer si existe; el cliente Go envía Bearer y `X-Management-Key`. Esto demuestra comportamiento del cliente, no el conjunto aceptado por cada host, ni permiso de lectura separado. La misma clave potencialmente habilita mutaciones y descarga de archivos auth: no considerarla un token inocuo de visualización. Nunca consultar o registrar esas rutas/archivos sensibles durante render. [S-003][S-010]

**Control y concurrencia, confianza alta en superficies; baja en garantías:** plugin registra POST de pools/rotate y operaciones de creación/borrado, además de virtualización; cliente puede `PATCH /v0/management/auth-files/fields` con `{name,prefix,note}`. La extensión declara failover automático y código programa rebalance proactivo; el plugin también tiene estado/polling propios. Que una operación se describa «atómica» en registro no demuestra transacción distribuida, CAS, idempotencia, orden entre clientes o reconciliación con afinidad por proyecto. Evitar múltiples controladores independientes; no se ha verificado protocolo de liderazgo exclusivo ni semántica de conflicto. [S-005][S-008][S-010][S-013]

## Compensaciones y riesgos

- **Lectura no necesariamente sin efectos:** handler del manifiesto invoca `GetOrAssignStandaloneAlias` para cuentas sin pool; no se ha auditado si esa llamada persiste cambios en todos los casos. Por ello GET no debe prometerse como operación sin efectos laterales. [S-002]
- **Local/remoto:** Telar exige loopback para su backend gestionado. Un endpoint remoto independiente cambia frontera de confianza: no reenviar automáticamente credenciales a URLs descubiertas; exigir transporte autenticado, autorización y redacción adecuados. No hay contrato demostrado de permisos finos ni transporte remoto Telar. [S-001][S-003]
- **Secretos/PII:** auth-files y endpoint de descarga pueden exponer credenciales; incluso manifest expone email, `account_key` y rutas lógicas. Recolectar mínimo, limitar difusión entre clientes y nunca volcar payloads en logs. [S-002][S-010]
- **Licencia:** texto raíz Pro rotulado PolyForm Noncommercial; alcance de subárbol y uso comercial no determinados. Inspirarse en contratos observados no equivale a permiso de copiar código o activos; consultar antes de redistribución. [S-004]

## Alternativas consideradas

- Lectura directa de estado persistido: la extensión la usa como fallback, pero puede estar desactualizada, omitir estado RAM y fallar remotamente; no recomendada como autoridad. [S-003][S-008]
- Hablar directamente con proveedor mediante `api-call`: mayor privilegio, dependencia de tokens y contratos específicos; no apropiado como requisito de render. [S-010][S-011]
- Consumir solo CPAMC sin plugin: no demuestra los campos de pool/centinela; degradar capacidad en vez de fingir paridad. [S-001][S-002]

## Incógnitas y límites

No se revisaron exhaustivamente middleware de host, releases ni permisos por ruta; no hay prueba de compatibilidad entre los tres componentes fijados, esquema completo de todas las versiones, políticas de retención, orden de respuestas, ni vínculo fiable pane/proyecto/cuenta. No se verificó si acceso a manifiesto requiere licencia activa. No se ejecutaron pruebas vivas, instalaciones, configuración, ni solicitudes a proveedores. Los resultados son inspección de código fijado, no garantías de producción. [S-001][S-002][S-003]

## Próximas acciones recomendadas

1. Acordar combinación y capacidades mínimas, revisar host/versiones y middleware exactos; no asumir ABI ni prefijo de ruta.
2. Solicitar un contrato de datos con validez y timestamp por ventana, identidad por familia, procedencia y errores; sin él representar cuotas ambiguas como desconocidas.
3. Diseñar después pruebas sintéticas aisladas para defaults 95/100/0, errores, múltiples familias, expiración, relojes, clientes concurrentes y redacción, sin credenciales ni servicio real salvo autorización independiente.
4. Si se desean acciones, definir dueño único, autorización, operaciones idempotentes/gestión de conflictos y reconciliación; evaluación jurídica antes de reutilizar código.
