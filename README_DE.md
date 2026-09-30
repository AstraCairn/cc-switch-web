# CC Switch Web

<p align="center">
  <strong>Headless-Web-Verwaltungskonsole für CC Switch — Optimiert für Linux-Server, Cloud-VPS und WSL2</strong>
</p>

<p align="center">
  <a href="README.md">English</a> | <a href="README_ZH.md">简体中文</a> | <a href="README_JA.md">日本語</a> | Deutsch
</p>

<p align="center">
  <a href="#-warum-cc-switch-web">Vorteile</a> •
  <a href="#-architektur--funktionsweise">Architektur</a> •
  <a href="#-funktionen">Funktionen</a> •
  <a href="#-schnellstart">Schnellstart</a> •
  <a href="#%EF%B8%8F-befehlszeilenoptionen">Optionen</a> •
  <a href="#%EF%B8%8F-produktionseinsatz">Deployment</a> •
  <a href="#-h%C3%A4ufig-gestellte-fragen-faq">FAQ</a> •
  <a href="#-danksagung--lizenz">Lizenz</a>
</p>

---

## 💡 Warum CC Switch Web?

[CC Switch](https://github.com/farion1231/cc-switch) ist ein leistungsfähiger Konfigurations- und Provider-Manager für KI-Programmierassistenten (Claude Code, Codex, Gemini CLI, Grok Build usw.).

Die offizielle Desktop-Anwendung basiert auf Tauri 2 und benötigt grafische Systembibliotheken (wie WebKitGTK unter Linux):
* **Keine Ausführung auf Headless-Servern**: Auf Remote-Linux-Servern, VPS, Docker-Containern oder WSL2 ohne Desktop-Umgebung startet die Desktop-App mangels Grafiktreiber/GUI-Bibliotheken nicht.
* **Einschränkungen von Terminal-CLIs**: Reine TUI-Tools sind zwar schlank, bieten jedoch bei Aufgaben wie parallelem Provider-Vergleich, Echtzeit-Anfrageprotokollen, grafischen Token-Nutzungsstatistiken und Skill-Paketverwaltung nicht den Komfort einer vollwertigen Konsole.

**CC Switch Web** schließt diese Lücke: **Es entfernt die Abhängigkeit von Fenstermanagern und WebKitGTK, behält jedoch die offizielle Rust-Backend-Logik und die React-Oberfläche vollständig bei**. Ein schlanker Web-Daemon läuft im Hintergrund auf dem Server, während Sie die gewohnte Verwaltungsoberfläche direkt im Browser Ihres lokalen Rechners bedienen!

---

## 🏗️ Architektur & Funktionsweise

CC Switch Web trennt strikt zwischen **Steuerungsebene (Control Plane)** und **Datenebene (Data Plane)**:

```text
┌─────────────────────────────────────────────────────────────┐
│                 Remote-Server (Headless Linux / VPS)        │
│                                                             │
│   [ Claude Code / Codex / Gemini CLI / Grok Build ... ]     │
│             │                                               │
│             │ (Modellanfragen)                              │
│             ▼                                               │
│   [ Lokaler Modell-Router ] 127.0.0.1:15721                 │
│             │                                               │
│             │ (Automatisches Failover / Request-Rektifiz.)  │
│             ▼                                               │
│     Upstream-Provider (Claude / OpenAI / Gemini ...)        │
│                                                             │
│ ─────────────────────────────────────────────────────────── │
│                                                             │
│   [ CC Switch Web Daemon ]                                  │
│      ├── SQLite-DB: ~/.cc-switch/cc-switch.db               │
│      ├── CLI-Tool-Konfigurationen: ~/.claude.json usw.      │
│      ├── Dauerhafter Authentifizierungs-Token: web-token    │
│      └── Web-Verwaltungsport: 127.0.0.1:3927 (Loopback)     │
└──────────────────────────────▲──────────────────────────────┘
                               │
                               │ Lokale SSH-Portweiterleitung (Empfohlen)
                               │ ssh -N -L 3927:127.0.0.1:3927 user@server
                               │
┌──────────────────────────────┴──────────────────────────────┐
│                    Lokaler Rechner                          │
│                                                             │
│   [ Moderner Browser ] ─▶ http://127.0.0.1:3927/#token=... │
│   (Reine Verwaltung: Provider, MCP, Skills, Statistiken)    │
└─────────────────────────────────────────────────────────────┘
```

### 🔒 Zentrale Sicherheitsprinzipien
1. **Zugangsdaten & Modellverkehr bleiben auf dem Server**: API-Schlüssel, OAuth-Sitzungen, SQLite-Datenbanken und hochfrequenter Modellverkehr verlassen niemals den Server. Der Browser empfängt und sendet lediglich Management-Befehle.
2. **Standardmäßige Loopback-Isolation**: Der Verwaltungsport (`3927`) und der Proxy-Router (`15721`) lauschen standardmäßig ausschließlich auf `127.0.0.1`, um eine versehentliche Freigabe im Internet zu verhindern.
3. **Dauerhafte Token-Authentifizierung**: Beim ersten Start wird ein sicherer Zufalls-Token generiert und in `~/.cc-switch/web-token` (Berechtigung `0600`) abgelegt. Nach dem ersten Aufruf mit `#token=...` speichert der Browser die Sitzung dauerhaft.

---

## ✨ Funktionen

- 🖥️ **Identische Desktop-Oberfläche**: Dieselbe React-Oberfläche wie in der Desktop-Version — Provider, MCP-Server, Skills-Marktplatz, Prompts, Sitzungshistorie und Token-Nutzungsgrafiken.
- ⚡ **Breite Werkzeugunterstützung**: Nahtloser Wechsel zwischen Konfigurationen für Claude Code, Codex, Gemini CLI, Grok Build, OpenCode, OpenClaw, Hermes Agent, Pi und mehr.
- 🔄 **Zuverlässiges Routing & Resilienz**: Unterstützung für Sofortwechsel (Hot-Switching), automatische Failover-Warteschlangen und automatische Request-Korrekturen (Rectifier).
- 🐧 **Optimiert für Headless-Umgebungen**:
  - Vollständiger Verzicht auf WebKitGTK / GTK3 bei minimalem Ressourcenverbrauch;
  - Native Datei- und Verzeichnisdialoge werden durch serverseitige Pfadeingaben ersetzt;
  - Nahtlose Zwischenablagen-Synchronisation über die Web Clipboard API;
  - Externe Links öffnen sich direkt im lokalen Browser;
  - Integriertes Vendored OpenSSL verhindert Build-Fehler auf minimalen Serverinstallationen.
- 📦 **Dual-Target-Codebasis**: Der Feature-Flag `desktop` bleibt erhalten; auf Systemen mit grafischer Oberfläche kann weiterhin die native Desktop-App kompiliert werden.

---

## 🚀 Schnellstart

### 1. Voraussetzungen
Stellen Sie sicher, dass auf dem Server folgende Werkzeuge vorhanden sind:
* **Node.js** (>= 18) und **pnpm**
* **Rust** (>= 1.85) und **Cargo**

### 2. Kompilierung

Führen Sie im Projektverzeichnis Folgendes aus:

```bash
# 1. Frontend-Abhängigkeiten installieren
pnpm install

# 2. Release-Build erstellen (Frontend-Build + Rust-Web-Binary)
pnpm build:web
```

> **Schrittweiser Debug-Build (optional)**:
> ```bash
> # Frontend-Assets nach ./dist bauen
> pnpm build:renderer
> 
> # Debug-Web-Binary kompilieren
> cargo build --manifest-path src-tauri/Cargo.toml --no-default-features --features web --bin cc-switch-web
> ```

### 3. Starten

```bash
./src-tauri/target/release/cc-switch-web --port 3927 --dist ./dist
```

Nach dem Start wird eine Ausgabe ähnlich der folgenden angezeigt:
```text
CC Switch 网页已启动
  本机地址: http://127.0.0.1:3927/
  第一次打开: http://127.0.0.1:3927/#token=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
  令牌文件: /home/username/.cc-switch/web-token
  页面目录: /path/to/dist
  停止: Ctrl+C
```

### 4. Konsole aufrufen

#### Option A: Zugriff über SSH-Tunnel (Empfohlen & Sicher)
Öffnen Sie auf Ihrem **lokalen Rechner** ein Terminal und starten Sie die Portweiterleitung:
```bash
ssh -N -L 3927:127.0.0.1:3927 <benutzer>@<server-ip>
```
Rufen Sie anschließend im lokalen Browser die vom Server ausgegebene Initialisierungs-URL auf:
```text
http://127.0.0.1:3927/#token=<ihr-token>
```
Sobald der Token im Browser gespeichert wurde, genügt künftig der Aufruf von `http://127.0.0.1:3927/`.

#### Option B: Lokaler Zugriff / WSL2
Öffnen Sie `http://127.0.0.1:3927/#token=<ihr-token>` direkt in Ihrem Browser.

---

## 🛠️ Befehlszeilenoptionen

`cc-switch-web` unterstützt folgende Optionen:

| Option | Standard | Umgebungsvariable | Beschreibung |
| :--- | :--- | :--- | :--- |
| `--host <IP>` | `127.0.0.1` | - | Zu bindende IP-Adresse (aus Sicherheitsgründen wird Loopback empfohlen). |
| `--port <PORT>` | `3927` | - | Port der Web-Verwaltungskonsole. |
| `--dist <PFAD>` | `./dist` | `CC_SWITCH_WEB_DIST` | Verzeichnis der statischen Frontend-Dateien (muss `index.html` enthalten). |
| `-h, --help` | - | - | Hilfemeldung anzeigen und beenden. |

---

## ⚙️ Produktionseinsatz

Für den dauerhaften Betrieb im Hintergrund und den automatischen Start beim Systemhochlauf empfiehlt sich die Verwaltung über `systemd`.

### 1. Dateien bereitstellen
Kopieren Sie die Binärdatei und die statischen Assets in ein Standardverzeichnis (z. B. `/opt/cc-switch-web`):
```bash
sudo mkdir -p /opt/cc-switch-web
sudo cp ./src-tauri/target/release/cc-switch-web /opt/cc-switch-web/
sudo cp -r ./dist /opt/cc-switch-web/
```

### 2. Systemd-Dienstdatei anlegen
Erstellen Sie `/etc/systemd/system/cc-switch-web.service` (ersetzen Sie `your_user` durch Ihren tatsächlichen Linux-Benutzernamen):

```ini
[Unit]
Description=CC Switch Web Service
After=network.target

[Service]
Type=simple
User=your_user
Group=your_user
WorkingDirectory=/opt/cc-switch-web
ExecStart=/opt/cc-switch-web/cc-switch-web --port 3927 --dist /opt/cc-switch-web/dist
Restart=always
RestartSec=5
Environment=HOME=/home/your_user

[Install]
WantedBy=multi-user.target
```

### 3. Dienst aktivieren und starten
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now cc-switch-web

# Status prüfen
sudo systemctl status cc-switch-web
```

---

## ❓ Häufig gestellte Fragen (FAQ)

<details>
<summary><strong>Q: Wo wird der Zugriffs-Token gespeichert? Wie setze ich ihn zurück?</strong></summary>

Der Token wird in `~/.cc-switch/web-token` hinterlegt.
So setzen Sie ihn zurück:
1. Beenden Sie den `cc-switch-web`-Prozess;
2. Löschen Sie die Datei: `rm ~/.cc-switch/web-token`;
3. Starten Sie den Dienst neu; es wird automatisch ein neuer Token generiert und ausgegeben.
</details>

<details>
<summary><strong>Q: Kann ich <code>--host 0.0.0.0</code> angeben, um den Port direkt im Internet freizugeben?</strong></summary>

**Von einer unverschlüsselten Freigabe im öffentlichen Internet wird dringend abgeraten!**
Bei unverschlüsseltem HTTP könnten Tokens und vertrauliche API-Schlüssel im Netzwerk abgefangen werden.
Empfohlene Wege für den Remote-Zugriff:
1. **SSH-Tunneling** (Empfohlen): `ssh -N -L 3927:127.0.0.1:3927 user@server`;
2. **VPN / Mesh-Netzwerke**: Zugriff über Tailscale, WireGuard etc.;
3. **Nginx Reverse Proxy**: Absicherung über SSL/TLS (HTTPS) kombiniert mit HTTP Basic Auth oder IP-Filterung.
</details>

<details>
<summary><strong>Q: Wo befinden sich Konfigurationen und Datenbanken?</strong></summary>

Identisch zur offiziellen Desktop-Version:
* **SQLite-Datenbank**: `~/.cc-switch/cc-switch.db`
* **Geräteeinstellungen**: `~/.cc-switch/settings.json`
* **Web-Token**: `~/.cc-switch/web-token`
* **Tool-Konfigurationen**: `~/.claude/`, `~/.codex/`, `~/.gemini/` usw.
</details>

<details>
<summary><strong>Q: Kann aus diesem Repository weiterhin die Desktop-App kompiliert werden?</strong></summary>

Ja. Auf Systemen mit installierten grafischen Abhängigkeiten (WebKitGTK und GTK3):
```bash
pnpm install
pnpm dev      # Desktop-App im Entwicklungsmodus
pnpm build    # Desktop-Installationspakete erstellen
```
</details>

---

## 📄 Danksagung & Lizenz

* Dieses Repository ist ein Derivat von [CC Switch](https://github.com/farion1231/cc-switch), angepasst für Headless-Serverumgebungen. **Es ist kein offizielles Projekt.**
* Das Urheberrecht des Originalprojekts liegt bei **Jason Young (2025)** unter der **[MIT License](LICENSE)**.
* Offizielle Releases, Handbücher und Sponsoring-Informationen zur Desktop-Version finden Sie im Upstream-Repository: [farion1231/cc-switch](https://github.com/farion1231/cc-switch) sowie auf der offiziellen Website [ccswitch.io](https://ccswitch.io).
