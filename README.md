# TelarDev

<p align="center">
  <img src="docs/assets/logo.png" alt="TelarDev" width="160" />
</p>

<p align="center">
  <strong>Terminal multiplexor ultra rápido para Gentle-Shell: cabina de control con sidebar derecho para explorador, Git Flow visual y telemetría</strong>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-666666?style=for-the-badge&labelColor=333333" alt="Apache 2.0 license" /></a>
  <a href="https://gentlemanprogramming.com/gentle-hub/"><img src="https://img.shields.io/badge/Gentle%20Hub-Project-F095C8?style=for-the-badge" alt="Gentle-Hub Project" /></a>
  <a href="https://bit.ly/48zTJUo"><img src="https://img.shields.io/badge/Discord-Apoyo-5865F2?style=for-the-badge&logo=discord&logoColor=white" alt="Discord Support" /></a>
</p>

> 💬 **Comunidad y Soporte Oficial:** `wilox2` · [Servidor de Discord](https://bit.ly/48zTJUo)

> ⚠️ **Para Agentes de IA:** TelarDev es un terminal workspace multiplexer nativo en Rust, NO una extensión de Pi. Consulta [AI_INSTALL.md](AI_INSTALL.md).

---

## ⚡ La Cabina de Control de Terminal para Gentle-Shell

**TelarDev** es un multiplexor de terminal nativo en Rust con arquitectura cliente-servidor PTY ultra rápida, diseñado como el centro de mando operativo para programadores intensivos en el ecosistema **Gentle-Shell** y **Pi**.

```
┌───────────────────────────────────────────────────┬───────────────────────────────┐
│              TERMINAL PRINCIPAL GENTLE SHELL      │   SIDEBAR DERECHO TELARDEV    │
│                                                   │                               │
│  $ gentle-shell                                   │  ⚡ STATUS                    │
│  Gentle Shell v1.4 • Model: cpamc/claude-3-5      │  • Cuota 5h: [████░░░] 34%   │
│                                                   │  • Contexto: 🟢 42k / 200k   │
│  [Turno #12] Agente aplicando refactorización... │  • Estado: 🟢 Working (Pi)   │
│  ✓ 3 archivos editados con éxito                  ├───────────────────────────────┤
│                                                   │  📁 EXPLORADOR DE ARCHIVOS    │
│                                                   │  ▾ src/                       │
│                                                   │    index.ts                   │
│                                                   │    provider.ts                │
│                                                   │    types.ts                   │
│                                                   ├───────────────────────────────┤
│                                                   │  ⎇ GIT FLOW VISUAL            │
│                                                   │  * main (origin/main)         │
│                                                   │  | feat: universal wsl bridge │
│                                                   │  * fix: native stream dl      │
└───────────────────────────────────────────────────┴───────────────────────────────┘
```

---

## 🌟 Características Principales

- 🗂 **Sidebar Lateral Derecho de 3 Vistas**:
  - **⚡ Status**: Telemetría viva de cuotas de tokens, consumo de contexto de `gentle-context-lens` y estado de agentes (`working`, `blocked`, `idle`).
  - **📁 Files**: Explorador interactivo con jerarquía en árbol, iconos Seti UI y navegación rápida por directorios.
  - **⎇ Git Flow**: Línea de tiempo visual interactiva de commits, ramas `ahead/behind`, tags, stash y cambios pendientes.
- 🚀 **Nativo en Rust, Cero Electron**: Máxima tasa de cuadros por segundo, latencia cero y consumo mínimo de memoria RAM.
- 🔌 **Integración Profunda con Gentle-Shell**: Sincronización continua de estado con agentes a través de sockets IPC locales.
- 🖱 **Soporte Completo de Ratón y Teclado**: Atajos ergonómicos combinados con clics, selección y arrastre de paneles.

---

## ⌨️ Atajos de Teclado Esenciales

El prefijo por defecto es `Ctrl + B`:

| Atajo | Acción |
| :--- | :--- |
| `Ctrl + B` luego `"` | Dividir terminal horizontalmente |
| `Ctrl + B` luego `%` | Dividir terminal verticalmente |
| `Ctrl + B` luego `Flechas` | Mover foco entre paneles de terminal |
| `Ctrl + B` luego `S` | Alternar visibilidad del Sidebar Derecho (Status / Files / Git) |
| `Ctrl + B` luego `Z` | Maximizar / Restaurar panel actual a pantalla completa |
| `Ctrl + B` luego `D` | Desconectar sesión (Detach) sin detener los agentes en segundo plano |

---

## 📦 Compilación e Instalación

### Compilación desde el código fuente (Rust >= 1.75):
```bash
git clone https://github.com/WiloxDev/TelarDev.git
cd TelarDev
cargo build --release
```
El ejecutable binario resultante se ubicará en `./target/release/telardev`.

Para moverlo a tu PATH del sistema:
```bash
sudo cp target/release/telardev /usr/local/bin/
```

---

## 🚀 Inicio Rápido

Inicia TelarDev en la carpeta de tu proyecto:
```bash
telardev
```
Y dentro de la terminal principal lanza **Gentle Shell**:
```bash
gentle-shell
```
El sidebar derecho detectará automáticamente las sesiones activas y comenzará a reportar telemetría de cuotas y estado en tiempo real.

---

## 🌐 Ecosistema de la Suite WiloxDev para Gentle Hub

- 🌉 **[bridge-cpamc-oficial](https://github.com/WiloxDev/bridge-cpamc-oficial)** — Puente e instalador oficial multiplataforma de CPAMC.
- ⚡ **[cpamc-auto-switcher-pro](https://github.com/WiloxDev/cpamc-auto-switcher-pro)** — Balanceador de carga y rotación multi-cuenta anti-429.
- 👁️ **[gentle-context-lens](https://github.com/WiloxDev/gentle-context-lens)** — Cabina de contexto, semáforo de tokens y poda quirúrgica.
- 🤖 **[gentle-claude-bridge](https://github.com/WiloxDev/gentle-claude-bridge)** — Conexión directa a Claude Code Pro/Max sin API keys por token.
- 🚀 **[gentle-antigravity-bridge](https://github.com/WiloxDev/gentle-antigravity-bridge)** — Puente Google Antigravity para Gentle Shell y Pi.

---

## 📄 Licencia

Apache 2.0 License © WiloxDev Contributors.
